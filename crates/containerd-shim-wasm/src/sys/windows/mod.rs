pub mod container;
pub mod job;
pub mod metrics;
pub mod networking;
pub mod signals;
pub mod stdio;

pub const DEFAULT_CONTAINERD_PIPE: &str = r"\\.\pipe\containerd-containerd";
