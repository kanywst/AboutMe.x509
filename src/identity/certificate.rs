use crate::terminal::args::Args;
use anyhow::{Context, Result};
use rcgen::{CertificateParams, DistinguishedName, DnType, KeyPair};

/// Core logic for minting a persona certificate.
pub fn generate_profile_cert(args: &Args) -> Result<(String, String)> {
    let mut params = CertificateParams::default();
    let mut dn = DistinguishedName::new();

    let name = args.name.as_ref().context("Name is required to mint a persona")?;
    dn.push(DnType::CommonName, name);

    if let Some(h) = args.hometown.as_ref().filter(|s| !s.is_empty()) {
        dn.push(DnType::LocalityName, h);
    }
    if let Some(a) = args.age.as_ref().filter(|s| !s.is_empty()) {
        dn.push(DnType::OrganizationalUnitName, format!("Age: {}", a));
    }
    if let Some(o) = args.occupation.as_ref().filter(|s| !s.is_empty()) {
        dn.push(DnType::OrganizationalUnitName, format!("Occupation: {}", o));
    }
    if let Some(b) = args.hobby.as_ref().filter(|s| !s.is_empty()) {
        dn.push(DnType::OrganizationalUnitName, format!("Hobby: {}", b));
    }
    if let Some(m) = args.motto.as_ref().filter(|s| !s.is_empty()) {
        dn.push(DnType::OrganizationalUnitName, format!("Motto: {}", m));
    }

    dn.push(DnType::OrganizationName, "AboutMe.x509 Personal CA");

    params.distinguished_name = dn;

    let key_pair = KeyPair::generate().context("Failed to generate cryptographic key pair")?;
    let cert = params.self_signed(&key_pair).context("Failed to sign the identity certificate")?;

    Ok((cert.pem(), key_pair.serialize_pem()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::terminal::args::Args;
    use std::path::PathBuf;

    #[test]
    fn test_generate_minimal_cert() {
        let args = Args {
            name: Some("Test User".to_string()),
            hometown: None,
            age: None,
            occupation: None,
            hobby: None,
            motto: None,
            cert_out: PathBuf::from("test_cert.pem"),
            key_out: PathBuf::from("test_key.pem"),
        };

        let result = generate_profile_cert(&args);
        assert!(result.is_ok());
    }
}
