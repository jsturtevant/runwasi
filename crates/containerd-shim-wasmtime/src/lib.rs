pub mod instance;

pub use instance::WasmtimeInstance;

#[cfg(test)]
#[path = "tests.rs"]
mod wasmtime_tests;
