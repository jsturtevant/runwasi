use std::env;
use std::path::PathBuf;
use std::{fs::OpenOptions, process::exit};

use clap::Parser;
use anyhow::{anyhow, Result};
use containerd_shim_wasm::sandbox::{oci, Stdio};
use oci_spec::runtime::Spec;
use wasmtime::{Engine, Linker, Module, Store};
use wasmtime_wasi::WasiCtxBuilder;

use containerd_shim_wasmtime::oci_wasmtime::wasi_dir;

fn main() -> Result<()> {
    let args = Args::parse();

    let module_name = args.module_name;
    let method = args.method.unwrap_or("_start".to_string());
    let args = args.args.unwrap_or_default();
    let engine = Engine::default();
    let env: Vec<_> = env::vars().collect();

    //let path = wasi_dir(".", OpenOptions::new().read(true))?;
    let wasi_builder = WasiCtxBuilder::new()
        .args(args.as_slice())?
        .envs(env.as_slice())?
        .inherit_stdio();
        //.preopened_dir(path, "/")?;

    log::info!("building wasi context");
    let wctx = wasi_builder.build();
    log::info!("wasi context ready");

    log::info!("loading module from file {} ", module_name);
    let module = Module::from_file(&engine, module_name)?;
    let mut linker = Linker::new(&engine);

    wasmtime_wasi::add_to_linker(&mut linker, |s| s)?;
    let mut store = Store::new(&engine, wctx);

    log::info!("instantiating instance");
    let instance = linker.instantiate(&mut store, &module)?;

    log::info!("getting start function");
    let start_func = instance
        .get_func(&mut store, &method)
        .ok_or_else(|| anyhow!("module does not have a WASI start function".to_string()))?;

    log::info!("calling start function");
    match start_func.call(&mut store, &[], &mut []) {
        Ok(_) => std::process::exit(0),
        Err(_) => std::process::exit(137),
    };
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    module_name: String,

    #[arg(short, long)]
    args: Option<Vec<String>>,

    #[arg(short='t', long, )]
    method : Option<String>,
}
