#[cfg(feature = "oci-v1-tar")]
use {
    anyhow::Context, oci_spec::image as spec, oci_tar_builder::Builder, sha256::try_digest,
    std::env, std::fs::File, std::path::PathBuf,
};

#[cfg(not(feature = "oci-v1-tar"))]
fn main() {}

#[cfg(feature = "oci-v1-tar")]
fn main() {
    env_logger::init();

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let p = out_dir.join("img.tar");
    let bin_output_dir = out_dir
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap();

    let app_path = bin_output_dir.join("wasi-demo-app.wasm");
    let windows_file = PathBuf::from("blank.txt");
    let layer_path = out_dir.join("layer.tar");
    let mut b = tar::Builder::new(File::create(&layer_path).unwrap());
    b.append_dir("Files", ".").unwrap();
    b.append_dir("Files\\Windows", ".").unwrap();
    b.append_dir("Files\\Windows\\System32", ".").unwrap();
    b.append_dir("Files\\Windows\\System32\\config", ".").unwrap();
    b.append_path_with_name(&app_path, "wasi-demo-app.wasm").unwrap();
    b.append_path_with_name(&windows_file, "Files\\Windows\\System32\\config\\DEFAULT").unwrap();
    b.append_path_with_name(&windows_file, "Files\\Windows\\System32\\config\\SAM").unwrap();
    b.append_path_with_name(&windows_file, "Files\\Windows\\System32\\config\\SECURITY").unwrap();
    b.append_path_with_name(&windows_file, "Files\\Windows\\System32\\config\\SOFTWARE").unwrap();
    b.append_path_with_name(&windows_file, "Files\\Windows\\System32\\config\\SYSTEM").unwrap();

    let mut builder = Builder::default();

    builder.add_layer(&layer_path);

    let config = spec::ConfigBuilder::default()
        .entrypoint(vec!["/wasi-demo-app.wasm".to_owned()])
        .build()
        .unwrap();

    let layer_digest = try_digest(layer_path.as_path()).unwrap();
    let img = spec::ImageConfigurationBuilder::default()
        .config(config)
        .os("windows")
        .architecture("wasm")
        .rootfs(
            spec::RootFsBuilder::default()
                .diff_ids(vec!["sha256:".to_owned() + &layer_digest])
                .build()
                .unwrap(),
        )
        .build()
        .context("failed to build image configuration")
        .unwrap();

    builder.add_config(
        img,
        "ghcr.io/containerd/runwasi/wasi-demo-app:latest".to_string(),
    );

    let f = File::create(&p).unwrap();
    builder.build(f).unwrap();
    std::fs::rename(&p, bin_output_dir.join("img.tar")).unwrap();
}
