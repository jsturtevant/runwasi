cargo build --bin containerd-shim-wasmtime-v1
cp .\target\debug\containerd-shim-wasmtime-v1.exe ..\containerd\bin\
ls ..\containerd\bin\containerd-shim-wasmtime-v1.exe