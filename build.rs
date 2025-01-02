use tonic_build;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().build_server(false).compile(
        &[
            "proto/custom_dict.proto",
            "proto/language_service.proto",
            "proto/bareun/dict_common.proto",
        ],
        &["proto"],
    )?;

    Ok(())
}
