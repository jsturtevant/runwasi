# Windows: Getting Started

Windows support is a work in progress. See https://github.com/containerd/runwasi/issues/49 for latest. Of note, it currently requires some additional work to support networking and volumes.

The Windows shims only supports [Wasm OCI images](../README.md#demo-2-using-oci-images-with-custom-wasm-layers).  

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

You may need to add the following to your path to access some of the binaries used in the `makefile`:

```
C:\Program Files\Git\usr\local\bin\ 
```

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

## Developing Linux shims Using Wsl

Follow the Windows instructions to [get started with WSL](https://github.com/containerd/runwasi/blob/main/docs/windows-getting-started.md#:~:text=To%20get%20started,Docker%20Buildx%20plugin.).

Once you have your WSL environment set and you have cloned the runwasi repository, can follow the same instructions on the [readme](../README.md#contributing).