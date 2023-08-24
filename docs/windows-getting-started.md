# Windows: Getting Started

Currently, **runwasi** depends on a Linux environment (i.e., because it has to wire up networking and rootfs mounts). Therefore, to run it on Windows, we recommend utilizing the Windows Subsystem for Linux (WSL).

To get started with WSL, see [this](https://docs.microsoft.com/en-us/windows/wsl/install).

Once you have your WSL environment set and you have cloned the **runwasi** repository, you will need to install Docker and the Docker Buildx plugin.

To install Docker and the Docker Buildx Plugin, see [this](https://docs.docker.com/engine/install/) to find specific installation instructions for your WSL distro.

Before proceeding, it's also recommended to install Docker Desktop on Windows and run it once.

To finish off installing pre-requisites, install Rust following [this](https://www.rust-lang.org/tools/install).

After following these steps and navigating to the runwasi directory in your terminal:
- run `make build`,
- run `make install`,
- run `make test/out/img.tar`,
- open a secondary terminal and run `containerd`, and
- run `make load`.

After this, you can execute an example, like: `ctr run --rm --runtime=io.containerd.wasmtime.v1 docker.io/library/wasmtest:latest testwasm`.

> To kill the process from the example, you can run: `ctr task kill -s SIGKILL testwasm`.

## Building and developing on Windows

You need to install `wasmedge`, `llvm` and `make`. This can be done using `winget`, `choco` or manually. (note as of writing this `winget` doesn't have the latest package and will builds will fail).  See `.github/scripts/build-windows.sh` for an example.

Once you have those dependencies you will need to set env:

```
$env:WASMEDGE_LIB_DIR="C:\Program Files\WasmEdge\lib"
$env:WASMEDGE_INCLUDE_DIR="C:\Program Files\WasmEdge\include"    
```

Then you can run:

```
make build
```

Make sure `ctr.exe` is on your path, if you install [following containerd's instructions](https://github.com/containerd/containerd/blob/main/docs/getting-started.md#installing-containerd-on-windows) it is typically in `$Env:ProgramFiles\containerd`.

### Using VS code
If you are using VS Code for development you can use the following `settings.json` in the `.vscode` folder of the project:

```
{
    "rust-analyzer.cargo.noDefaultFeatures": true,
    "rust-analyzer.cargo.extraEnv": {
        "WASMEDGE_LIB_DIR": "C:\\Program Files\\WasmEdge\\lib",
        "WASMEDGE_INCLUDE_DIR": "C:\\Program Files\\WasmEdge\\include"
    }
}

### Debugging
It can be handy to attach a debugger. The shim comes with the [ability to pause](https://github.com/containerd/rust-extensions/blob/27a503a474c68cdba7eff5b9f1b439d1846c39e0/crates/shim/src/synchronous/mod.rs#L249) the shim after its startup sequence.  

The general process is:

1. Set the flag `$env:SHIM_DEBUGGER="true"` before starting containerd
1. Run the shim with `ctr run -rm --runtime=io.containerd.wasmtime.v1 ghcr.io/containerd/runwasi/wasi-demo-app:latest testwasm`. At this point in the containerd logs you will see output like 
1. Attach the debugger and set a break point
1. Use [docker-signal](https://github.com/moby/docker-signal) the program to continue `go run .\docker-signal.go --debugger -pid 34344`

#### VS Code debugger configuration

Install [C/C++ Tools extension](https://marketplace.visualstudio.com/items?itemName=ms-vscode.cpptools).

Start VS Code as an Admin and use the following configuration file. This will prompt you for a process to attach to (select either by choosing PID or name). There will be two processes due to the shim activation process in Windows, select the one that doesn't have `start` as part of is command line arguments.

```
{
    "name": "(runwasi) Attach",
    "type": "cppvsdbg",
    "request": "attach",
    "processId": "${command:pickProcess}",
    "requireExactSource": false
}
```
