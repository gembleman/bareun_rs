fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Compile all proto files together so they share common types
    tonic_prost_build::configure()
        .build_server(false)
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile_protos(
            &[
                "proto/dict_common.proto",
                "proto/lang_common.proto",
                "proto/language_service.proto",
                "proto/revision_service.proto",
                "proto/custom_dict.proto",
            ],
            &["proto"],
        )?;

    Ok(())
}
