use std::os::windows::prelude::AsRawHandle;
use std::path::PathBuf;
use std::process::Command;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;

use chrono::Utc;
use containerd_shim_wasm::sandbox::error::Error;
use containerd_shim_wasm::sandbox::instance::{ExitCode, Wait, SIGKILL};
use containerd_shim_wasm::sandbox::instance_utils::determine_rootdir;
use containerd_shim_wasm::sandbox::{oci, Instance, InstanceConfig, Stdio};
use log::debug;
use oci_spec::runtime::Spec;
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::System::Threading::{
    OpenProcess, TerminateProcess, PROCESS_QUERY_INFORMATION, PROCESS_TERMINATE,
};

pub struct Wasi {
    id: String,
    exit_code: ExitCode,
    engine: wasmtime::Engine,
    stdio: Stdio,
    bundle: String,
    spec: Option<Spec>,
    process_handle: Mutex<Option<isize>>,
}

impl Instance for Wasi {
    type Engine = wasmtime::Engine;

    fn new(id: String, cfg: Option<&InstanceConfig<Self::Engine>>) -> Self {
        log::info!("creating new instance: {}", id);
        let cfg = cfg.unwrap();
        let bundle = cfg.get_bundle().unwrap_or_default();
        Wasi {
            id,
            exit_code: Arc::new((Mutex::new(None), Condvar::new())),
            engine: cfg.get_engine(),
            stdio: Stdio {
                stdin: cfg.get_stdin().try_into().unwrap(),
                stdout: cfg.get_stdout().try_into().unwrap(),
                stderr: cfg.get_stderr().try_into().unwrap(),
            },
            bundle,
            spec: cfg.get_spec(),
            process_handle: Mutex::new(None),
        }
    }

    fn start(&self) -> std::result::Result<u32, Error> {
        debug!("starting instance");
        self.stdio.redirect()?;
        let code = self.exit_code.clone();

        let spec = self.spec.as_ref().unwrap();
        let (module_name, method) = oci::get_module(&spec);
        let module_name = match module_name {
            Some(m) => m,
            None => {
                return Err(Error::InvalidArgument(
                    "no module provided, cannot load module from file within container".to_string(),
                ));
            }
        };
        let module_name = module_name.strip_prefix("/").unwrap();

        let root = oci::get_root(&spec);
        let mod_path = root.join(module_name);
        let mut child_process = Command::new("wasmtime")
            .arg("run")
            .arg(mod_path.to_str().unwrap())
            .spawn()
            .map_err(|err| Error::Any(anyhow::anyhow!("failed to start container: {}", err)))?;

        let process_id = child_process.id() as u32;
        self.process_handle
            .lock()
            .unwrap()
            .replace(child_process.as_raw_handle() as isize);
        let _ = thread::spawn(move || {
            let (lock, cvar) = &*code;

            let status = match child_process.wait() {
                Err(e) => panic!("waitpid failed: {}", e),
                Ok(status) => status.code().unwrap(),
            };
            debug!("wasi instance exited with status ");
            let mut ec = lock.lock().unwrap();
            *ec = Some((status as u32, Utc::now()));
            drop(ec);
            cvar.notify_all();
        });

        Ok(process_id)
    }

    fn kill(&self, signal: u32) -> std::result::Result<(), Error> {
        if signal != SIGKILL as u32 {
            return Err(Error::InvalidArgument(
                "only SIGKILL is supported".to_string(),
            ));
        }

        let handle = self.process_handle.lock().unwrap().unwrap();

        kill_process_by_id(HANDLE(handle));
        Ok(())
    }

    fn delete(&self) -> std::result::Result<(), Error> {
        // todo: clean up jobobject?
        Ok(())
    }

    fn wait(&self, waiter: &Wait) -> std::result::Result<(), Error> {
        let id = self.id.clone();
        let exit_code = self.exit_code.clone();
        log::info!("waiting for instance: {}", id);
        let code = exit_code;
        waiter.set_up_exit_code_wait(code)
    }
}

fn kill_process_by_id(handle: HANDLE) -> Result<(), String> {
    let result = unsafe { TerminateProcess(handle, 1) };
    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to terminate process: {:?}", e)),
    }
}
