// This is free and unencumbered software released into the public domain.

#[cfg(feature = "std")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure logging & tracing:
    #[cfg(feature = "tracing")]
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(tracing_subscriber::filter::LevelFilter::WARN)
        .init();

    println!("asimov-template-emitter"); // TODO

    Ok(())
}

#[cfg(not(feature = "std"))]
fn main() {
    unimplemented!("asimov-template-emitter requires the 'std' feature")
}
