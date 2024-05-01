ps *shim* | kill
ctr -n  default c rm testwasmoci
ctr -n  default snapshots rm testwasmoci
rm -recurse -force C:\ProgramData\containerd\state\io.containerd.runtime.v2.task\default\testwasmoci   # Windows   