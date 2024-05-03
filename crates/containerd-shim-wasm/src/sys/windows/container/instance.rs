use std::marker::PhantomData;
use std::os::windows::prelude::AsRawHandle;
use std::process::Stdio;
use std::sync::Mutex;
use std::time::Duration;
use std::{env, fs, thread};

use anyhow::Context;
use chrono::{DateTime, Utc};
use power_process::power_command::{Command, CommandExt};
use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::Threading::{TerminateProcess, PROC_THREAD_ATTRIBUTE_JOB_LIST};

use crate::container::{Engine, RuntimeContext, WasiContext};
use crate::sandbox::sync::WaitableCell;
use crate::sandbox::{
    containerd, Error as SandboxError, Instance as SandboxInstance, InstanceConfig,
    Stdio as WasiStdio,
};
use crate::sys::job::new_job;
use crate::sys::signals::SIGKILL;

pub struct Instance<E: Engine> {
    exit_code: WaitableCell<(u32, DateTime<Utc>)>,
    id: String,
    process_handle: Mutex<Option<isize>>,
    _phantom: PhantomData<E>,
    namespace: String,
    address: String,
    bundle: std::path::PathBuf,
    //bundle: &std::path::Path,
}

impl<E: Engine> SandboxInstance for Instance<E> {
    type Engine = E;

    fn new(id: String, cfg: Option<&InstanceConfig<Self::Engine>>) -> Result<Self, SandboxError> {
        log::info!("creating new instance: {}", id);
        let cfg = cfg.context("missing instance config")?;
        let namespace = cfg.get_namespace();
        let address = cfg.get_containerd_address();
        let bundle = cfg.get_bundle();

        log::info!("writing id file: {}", bundle.to_str().unwrap());
        let id_file = bundle.join("id");
        fs::write(id_file, &id).context("unable to write id file");

        Ok(Self {
            id,
            namespace,
            address,
            bundle: bundle.to_path_buf(),
            exit_code: WaitableCell::new(),
            process_handle: Mutex::new(None),
            _phantom: Default::default(),
        })
    }

    /// Start the instance
    /// The returned value should be a unique ID (such as a PID) for the instance.
    /// Nothing internally should be using this ID, but it is returned to containerd where a user may want to use it.
    fn start(&self) -> Result<u32, SandboxError> {
        log::debug!("starting instance {}", self.id);

        // make sure we have an exit code by the time we finish (even if there's a panic)
        let guard = self.exit_code.set_guard_with(|| (137, Utc::now()));

        let mut cmd = Command::new("containerd-wasmtime-windows");
        cmd.current_dir(self.bundle.as_path())
            .arg("--address")
            .arg(self.address.clone())
            .arg("--namespace")
            .arg(self.namespace.clone());

        // todo read the config and set up cpu/mem limits
        let job = new_job(&self.id).unwrap();
        unsafe {
            cmd.raw_attribute(
                PROC_THREAD_ATTRIBUTE_JOB_LIST as usize,
                job.as_raw_handle() as isize,
            );
        }

        let mut child_process = cmd.spawn().context("failed to start container")?;

        let process_id = child_process.id() as u32;
        self.process_handle
            .lock()
            .unwrap()
            .replace(child_process.as_raw_handle() as isize);

        let exit_code = self.exit_code.clone();
        let _ = thread::spawn(move || {
            let _guard = guard;

            let status = match child_process.wait() {
                Err(e) => panic!("waitpid failed: {}", e),
                Ok(status) => status.code().unwrap(),
            };
            log::debug!("wasi instance exited with status {status}");
            let _ = exit_code.set((status as u32, Utc::now()));
        });

        Ok(process_id)
    }

    /// Send a signal to the instance
    fn kill(&self, signal: u32) -> Result<(), SandboxError> {
        if signal != SIGKILL as u32 {
            return Err(SandboxError::InvalidArgument(
                "only SIGKILL is supported".to_string(),
            ));
        }

        let h = self.process_handle.lock().unwrap().unwrap();

        kill_process_by_id(HANDLE(h));
        Ok(())
    }

    /// Delete any reference to the instance
    /// This is called after the instance has exited.
    fn delete(&self) -> Result<(), SandboxError> {
        Ok(())
    }

    /// Waits for the instance to finish and retunrs its exit code
    /// Returns None if the timeout is reached before the instance has finished.
    /// This is a blocking call.
    fn wait_timeout(&self, t: impl Into<Option<Duration>>) -> Option<(u32, DateTime<Utc>)> {
        self.exit_code.wait_timeout(t).copied()
    }
}

fn kill_process_by_id(handle: HANDLE) -> Result<(), String> {
    let result = unsafe { TerminateProcess(handle, 1) };
    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to terminate process: {:?}", e)),
    }
}
