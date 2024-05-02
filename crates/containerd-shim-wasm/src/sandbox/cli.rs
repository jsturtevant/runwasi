use std::{env, fs};
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use std::sync::Arc;

use anyhow::{Context, Error};
use containerd_shim::{parse, run, Config, Flags};
use oci_spec::image::Platform;
use oci_spec::runtime::{ProcessBuilder, RootBuilder, Spec, SpecBuilder};
use tokio::sync::mpsc::error;
use ttrpc::Server;

use crate::container::{Engine as WasmEngine, WasiContext};
use crate::sandbox::manager::Shim;
use crate::sandbox::shim::Local;
use crate::sandbox::{containerd, Instance, InstanceConfig, ManagerService, ShimCli, Stdio};
use crate::services::sandbox;
use crate::services::sandbox_ttrpc::{create_manager, Manager};

pub mod r#impl {
    pub use git_version::git_version;
}

pub use crate::{revision, version};

use super::stdio::StdioStream;

#[macro_export]
macro_rules! version {
    () => {
        env!("CARGO_PKG_VERSION")
    };
}

#[macro_export]
macro_rules! revision {
    () => {
        match $crate::sandbox::cli::r#impl::git_version!(
            args = ["--match=:", "--always", "--abbrev=15", "--dirty=.m"],
            fallback = "",
        ) {
            "" => None,
            version => Some(version),
        }
    };
}

pub fn shim_main<'a, I>(
    name: &str,
    version: &str,
    revision: impl Into<Option<&'a str>>,
    shim_version: impl Into<Option<&'a str>>,
    config: Option<Config>,
) where
    I: 'static + Instance + Sync + Send,
    I::Engine: WasmEngine,
{
    let os_args: Vec<_> = std::env::args_os().collect();
    let flags = parse(&os_args[1..]).unwrap();
    let argv0 = PathBuf::from(&os_args[0]);
    let argv0 = argv0.file_stem().unwrap_or_default().to_string_lossy();

    if flags.version {
        println!("{argv0}:");
        println!("  Runtime: {name}");
        println!("  Version: {version}");
        println!("  Revision: {}", revision.into().unwrap_or("<none>"));
        println!();

        std::process::exit(0);
    }
    let shim_version = shim_version.into().unwrap_or("v1");

    let lower_name = name.to_lowercase();
    let shim_cli = format!("containerd-shim-{lower_name}-{shim_version}");
    let shim_client = format!("containerd-shim-{lower_name}d-{shim_version}");
    let shim_daemon = format!("containerd-{lower_name}d");
    let shim_id = format!("io.containerd.{lower_name}.{shim_version}");
    let windows_shim_instance = format!("containerd-{lower_name}-windows");

    match argv0.to_lowercase() {
        s if s == shim_cli => {
            run::<ShimCli<I>>(&shim_id, config);
        }
        s if s == shim_client => {
            run::<Shim>(&shim_client, config);
        }
        s if s == shim_daemon => {
            log::info!("starting up!");
            let s: ManagerService<Local<I>> = Default::default();
            let s = Arc::new(Box::new(s) as Box<dyn Manager + Send + Sync>);
            let service = create_manager(s);

            let mut server = Server::new()
                .bind("unix:///run/io.containerd.wasmwasi.v1/manager.sock")
                .expect("failed to bind to socket")
                .register_service(service);

            server.start().expect("failed to start daemon");
            log::info!("server started!");
            let (_tx, rx) = channel::<()>();
            rx.recv().unwrap();
        }
        s if s == windows_shim_instance => {
            match run_windows_instance::<I>(&flags) {
                Ok(_) => {
                    log::info!("instance exited successfully");
                    std::process::exit(0);
                }
                Err(e) => {
                    eprintln!("error: {}", e);
                    std::process::exit(10);
                }
            }
        }
        _ => {
            eprintln!("error: unrecognized binary name, expected one of {shim_cli}, {shim_client}, or {shim_daemon}.");
            std::process::exit(1);
        }
    }
}


fn run_windows_instance<I: Instance>(flags: &Flags) -> Result<(), Error>
where <I as Instance>::Engine: WasmEngine {   
    let e = I::Engine::default();

    let cd = env::current_dir()?;
    let id = fs::read_to_string(cd.join("id"))?;

    let wasi_stdio = Stdio{
        stdin: StdioStream::try_from_path("stdin")?,
        stderr: StdioStream::try_from_path("stderr")?,
        stdout: StdioStream::try_from_path("stdout")?,
    };

    let (modules, platform) = containerd::Client::connect(&flags.address, &flags.namespace)?
            .load_modules(&id, &e).context("failed to load modules")?;


    let mut spec = Spec::load(cd.join("config.json")).unwrap(); 
    let ctx = WasiContext {
        spec: &spec,
        wasm_layers: &modules,
        platform: &platform,
    };


        e.run_wasi(&ctx, wasi_stdio)?;
    Ok(())
}
