use std::time::Duration;

use containerd_shim_wasm::container::Instance;
use containerd_shim_wasm::testing::modules::*;
use containerd_shim_wasm::testing::{oci_helpers, WasiTest};
use serial_test::serial;
use wasmtime::Config;
use WasmtimeTestInstance as WasiInstance;

use crate::instance::{WasiConfig, WasmtimeEngine};

// use test configuration to avoid dead locks when running tests that use precompiled content
// https://github.com/containerd/runwasi/issues/357
pub type WasmtimeTestInstance = Instance<WasmtimeEngine<WasiTestConfig>>;

#[derive(Clone)]
pub struct WasiTestConfig {}

impl WasiConfig for WasiTestConfig {
    fn new_config() -> Config {
        let mut config = wasmtime::Config::new();
        // Disable Wasmtime parallel compilation for the tests
        // see https://github.com/containerd/runwasi/pull/405#issuecomment-1928468714 for details
        config.parallel_compilation(false);
        config.wasm_component_model(true); // enable component linking
        config
    }
}

#[test]
#[serial]
fn test_hello_world_oci() -> anyhow::Result<()> {
    let (builder, _oci_cleanup) = WasiTest::<WasiInstance>::builder()?
        .with_wasm(HELLO_WORLD)?
        .as_oci_image(None, None)?;

    let (exit_code, stdout, _) = builder.build()?.start()?.wait(Duration::from_secs(10))?;

    assert_eq!(exit_code, 0);
    assert_eq!(stdout, "hello world\n");

    Ok(())
}

#[test]
#[serial]
fn test_hello_world_oci_uses_precompiled() -> anyhow::Result<()> {
    let (builder, _oci_cleanup1) = WasiTest::<WasiInstance>::builder()?
        .with_wasm(HELLO_WORLD)?
        .as_oci_image(
            Some("localhost/hello:latest".to_string()),
            Some("c1".to_string()),
        )?;

    let (exit_code, stdout, _) = builder.build()?.start()?.wait(Duration::from_secs(10))?;

    assert_eq!(exit_code, 0);
    assert_eq!(stdout, "hello world\n");

    let (label, _id) = oci_helpers::get_content_label()?;
    assert!(
        label.starts_with("runwasi.io/precompiled/wasmtime/"),
        "was {}",
        label
    );

    // run second time, it should succeed without recompiling
    let (builder, _oci_cleanup2) = WasiTest::<WasiInstance>::builder()?
        .with_wasm(HELLO_WORLD)?
        .as_oci_image(
            Some("localhost/hello:latest".to_string()),
            Some("c2".to_string()),
        )?;

    let (exit_code, stdout, _) = builder.build()?.start()?.wait(Duration::from_secs(10))?;

    assert_eq!(exit_code, 0);
    assert_eq!(stdout, "hello world\n");

    Ok(())
}

#[test]
#[serial]
fn test_hello_world_oci_uses_precompiled_when_content_removed() -> anyhow::Result<()> {
    let (builder, _oci_cleanup1) = WasiTest::<WasiInstance>::builder()?
        .with_wasm(HELLO_WORLD)?
        .as_oci_image(
            Some("localhost/hello:latest".to_string()),
            Some("c1".to_string()),
        )?;

    let (exit_code, stdout, _) = builder.build()?.start()?.wait(Duration::from_secs(10))?;

    assert_eq!(exit_code, 0);
    assert_eq!(stdout, "hello world\n");

    // remove the compiled content from the cache
    let (label, id) = oci_helpers::get_content_label()?;
    assert!(
        label.starts_with("runwasi.io/precompiled/wasmtime/"),
        "was {}",
        label
    );
    oci_helpers::remove_content(id)?;

    // run second time, it should succeed
    let (builder, _oci_cleanup2) = WasiTest::<WasiInstance>::builder()?
        .with_wasm(HELLO_WORLD)?
        .as_oci_image(
            Some("localhost/hello:latest".to_string()),
            Some("c2".to_string()),
        )?;

    let (exit_code, stdout, _) = builder.build()?.start()?.wait(Duration::from_secs(10))?;

    assert_eq!(exit_code, 0);
    assert_eq!(stdout, "hello world\n");

    Ok(())
}
