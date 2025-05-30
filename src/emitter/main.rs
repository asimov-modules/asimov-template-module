// This is free and unencumbered software released into the public domain.

#[cfg(feature = "std")]
fn main() -> Result<asimov_module::SysexitsError, Box<dyn std::error::Error>> {
    use asimov_module::SysexitsError::*;

    // Load environment variables from `.env`:
    asimov_module::dotenv().ok();

    // Expand wildcards and @argfiles:
    let _args = asimov_module::args_os()?;

    // Configure logging & tracing:
    #[cfg(feature = "tracing")]
    asimov_module::tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(asimov_module::tracing_subscriber::filter::LevelFilter::WARN)
        .init();

    println!("asimov-template-emitter"); // TODO

    Ok(EX_UNAVAILABLE)
}

#[cfg(not(feature = "std"))]
fn main() {
    unimplemented!("asimov-template-emitter requires the 'std' feature")
}
