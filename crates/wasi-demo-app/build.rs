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
    let layer_path = out_dir.join("layer.tar");
    let mut tar_builder = tar::Builder::new(File::create(&layer_path).unwrap());

    #[cfg(unix)]
    let app_file = "wasi-demo-app.wasm";
    #[cfg(windows)]
    let app_file = "Files\\wasi-demo-app.wasm";
    
    #[cfg(windows)]
    {
        // Containerd Snapshotter on Windows requires container files to be in "Files" directory
        
        let windows_file = PathBuf::from("blank.txt");

        // need a blank file to populate the config files for windows.
        // these are required files for the containerd snapshotter to work
        tar_builder.append_dir("Files", ".").unwrap();
        tar_builder.append_dir("Files\\Windows", ".").unwrap();
        tar_builder
            .append_dir("Files\\Windows\\System32", ".")
            .unwrap();
        tar_builder
            .append_dir("Files\\Windows\\System32\\config", ".")
            .unwrap();
        tar_builder
            .append_path_with_name(&windows_file, "Files\\Windows\\System32\\config\\DEFAULT")
            .unwrap();
        tar_builder
            .append_path_with_name(&windows_file, "Files\\Windows\\System32\\config\\SAM")
            .unwrap();
        tar_builder
            .append_path_with_name(&windows_file, "Files\\Windows\\System32\\config\\SECURITY")
            .unwrap();
        tar_builder
            .append_path_with_name(&windows_file, "Files\\Windows\\System32\\config\\SOFTWARE")
            .unwrap();
        tar_builder
            .append_path_with_name(&windows_file, "Files\\Windows\\System32\\config\\SYSTEM")
            .unwrap();
    }

    tar_builder
        .append_path_with_name(&app_path, app_file)
        .unwrap();

    let mut builder = Builder::default();

    builder.add_layer(&layer_path);

    let config = spec::ConfigBuilder::default()
        .entrypoint(vec!["/wasi-demo-app.wasm".to_owned()])
        .build()
        .unwrap();

    let layer_digest = try_digest(layer_path.as_path()).unwrap();
    let img = spec::ImageConfigurationBuilder::default()
        .config(config)
        .os("wasi")
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
