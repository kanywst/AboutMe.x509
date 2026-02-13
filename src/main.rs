use anyhow::{Context, Result};
use clap::Parser;
use std::fs::File;
use std::io::Write;
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

use aboutme_x509::terminal::wizard::run_wizard;
use aboutme_x509::{Args, generate_profile_cert};

fn main() -> Result<()> {
    // 1. Initialize diagnostics
    let subscriber = FmtSubscriber::builder().with_max_level(Level::INFO).finish();
    tracing::subscriber::set_global_default(subscriber)
        .context("Setting default tracing subscriber failed")?;

    // 2. Parse CLI arguments
    let mut args = Args::parse();

    println!("👋 Welcome to AboutMe.x509!");
    println!("Let's package your personal profile into a secure X.509 certificate.\n");

    // 3. Interactive enrichment if needed
    run_wizard(&mut args).context("Wizard interaction failed")?;

    // 4. Core domain logic: Certificate Generation
    let (cert_pem, key_pem) =
        generate_profile_cert(&args).context("Failed to generate identity certificate")?;

    // 5. Persistence
    save_to_disk(&args.cert_out, &cert_pem)?;
    save_to_disk(&args.key_out, &key_pem)?;

    info!("Minting complete!");
    println!("\n✨ Your 'About Me' certificate has been created!");
    println!("📄 Certificate: {:?}", args.cert_out);
    println!("🔑 Private Key: {:?}", args.key_out);

    Ok(())
}

fn save_to_disk(path: &std::path::Path, data: &str) -> Result<()> {
    let mut file =
        File::create(path).with_context(|| format!("Failed to create file: {:?}", path))?;
    file.write_all(data.as_bytes())
        .with_context(|| format!("Failed to write data to: {:?}", path))?;
    Ok(())
}
