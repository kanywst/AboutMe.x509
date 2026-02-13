use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug, Clone)]
#[command(name = "aboutme-x509")]
#[command(author = "Developed by kanywst")]
#[command(version = "1.0")]
#[command(about = "AboutMe.x509: Create your personal profile as a digital certificate", long_about = None)]
pub struct Args {
    /// Your full name
    #[arg(short, long)]
    pub name: Option<String>,

    /// Your hometown or current location
    #[arg(short = 'H', long)]
    pub hometown: Option<String>,

    /// Your age
    #[arg(short, long)]
    pub age: Option<String>,

    /// Your occupation or role
    #[arg(short, long)]
    pub occupation: Option<String>,

    /// Your favorite hobby
    #[arg(short = 'b', long)]
    pub hobby: Option<String>,

    /// Your personal motto
    #[arg(short, long)]
    pub motto: Option<String>,

    /// Output certificate file path
    #[arg(long, default_value = "cert.pem")]
    pub cert_out: PathBuf,

    /// Output private key file path
    #[arg(long, default_value = "key.pem")]
    pub key_out: PathBuf,
}
