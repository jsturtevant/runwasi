pub mod container;
pub mod metrics;
pub mod networking;
pub mod signals;
pub mod stdio;

pub const DEFAULT_CONTAINERD_PIPE: &str = "/run/containerd/containerd.sock";
