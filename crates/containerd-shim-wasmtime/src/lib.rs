pub mod instance;

pub use instance::WasmtimeInstance;

#[cfg(unix)]
#[cfg(test)]
#[path = "tests_container.rs"]
mod wasmtime_container_tests;

#[cfg(test)]
#[path = "tests_oci.rs"]
mod wasmtime_oci_tests;
