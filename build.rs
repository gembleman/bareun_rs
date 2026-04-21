fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var_os("PROTOC").is_none() {
        unsafe {
            std::env::set_var("PROTOC", protoc_bin_vendored::protoc_bin_path()?);
        }
    }

    tonic_prost_build::configure()
        .build_server(false)
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile_protos(
            &[
                "proto/bareun/dict_common.proto",
                "proto/bareun/lang_common.proto",
                "proto/bareun/language_service.proto",
                "proto/bareun/revision_service.proto",
                "proto/bareun/custom_dict.proto",
            ],
            &["proto"],
        )?;

    Ok(())
}
