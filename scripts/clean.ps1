# dev helper script incase something goes wrong

param(
    [string]$containerName = "testwasmoci",
    [string]$namespace = "default"
)

ps *shim* | kill
ctr -n  $namespace c rm $containerName
ctr -n  $namespace snapshots rm $containerName
rm -recurse -force C:\ProgramData\containerd\state\io.containerd.runtime.v2.task\$namespace\$containerName
