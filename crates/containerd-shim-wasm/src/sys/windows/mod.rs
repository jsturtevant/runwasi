pub mod container;
pub mod metrics;
pub mod networking;
pub mod signals;
pub mod stdio;
pub mod job;

pub const DEFAULT_CONTAINERD_PIPE: &str = r"\\.\pipe\containerd-containerd";