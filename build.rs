fn main() -> Result<(), Box<dyn std::error::Error>> {
  tonic_build::configure()
      .build_server(false)
      .out_dir("src/lnd_client")
      .compile(
          &["lnd/lnrpc/lightning.proto"],
          &["lnd/lnrpc"],
      )?;

  Ok(())
}
