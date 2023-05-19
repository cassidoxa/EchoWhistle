fn main() -> Result<(), Box<dyn std::error::Error>> {
    // I've statically generated and checked in the sni.rs file that tonic-build produces.
    // Leaving this module here for posterity.

    //tonic_build::configure()
    //    .build_server(false)
    //    .out_dir("./src/sni")
    //    .compile(&["proto/sni.proto"], &["proto"])?;
    //tonic_build::compile_protos("proto/sni.proto")?;

    Ok(())
}
