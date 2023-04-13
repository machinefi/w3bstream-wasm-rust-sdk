use anyhow::{bail, Context, Ok, Result};
use k256::{
    ecdsa::{
        signature::Signer, signature::Verifier, DerSignature, Signature, SigningKey, VerifyingKey,
    },
    EncodedPoint,
};

pub fn sign(prikey_hex: &str, data_bytes: &[u8]) -> Result<String> {
    let sig = sign_the_message(prikey_hex, data_bytes)?;
    Ok(sig.to_string())
}

pub fn sign_der(prikey_hex: &str, data_bytes: &[u8]) -> Result<String> {
    let sig = sign_the_message(prikey_hex, data_bytes)?;
    Ok(hex::encode(sig.to_der()))
}

pub fn verify(pubkey_hex: &str, data_bytes: &[u8], sig_hex: &str) -> Result<()> {
    let sig = get_raw_signature(&hex::decode(sig_hex)?)?;

    let pbk = get_verifyingkey(&hex::decode(pubkey_hex)?)?;

    match pbk.verify(data_bytes, &sig) {
        Err(err) => bail!(err.to_string()),
        _ => return Ok(()),
    }
}

pub fn verify_der(pubkey_hex: &str, data_bytes: &[u8], sig_hex: &str) -> Result<()> {
    let sig = get_der_signature(&hex::decode(sig_hex)?)?;

    let pbk = get_verifyingkey(&hex::decode(pubkey_hex)?)?;

    match pbk.verify(data_bytes, &sig) {
        Err(err) => bail!(err.to_string()),
        _ => return Ok(()),
    }
}

pub fn pubkey(prikey_hex: &str) -> Result<Vec<u8>> {
    let signer =
        SigningKey::from_slice(&hex::decode(prikey_hex)?).context("fail to get the private key")?;
    let pbk = signer.verifying_key().to_encoded_point(false);
    Ok(pbk.to_bytes().to_vec())
}

fn sign_the_message(prikey_hex: &str, data_bytes: &[u8]) -> Result<Signature> {
    let signer =
        SigningKey::from_slice(&hex::decode(prikey_hex)?).context("fail to get the private key")?;
    Ok(signer.sign(data_bytes))
}

fn get_der_signature(sig_bytes: &[u8]) -> Result<DerSignature> {
    DerSignature::from_bytes(&sig_bytes).context("fail to decode the signature")
}

fn get_raw_signature(sig_bytes: &[u8]) -> Result<Signature> {
    Signature::from_slice(sig_bytes).context("fail to decode the signature")
}

fn get_verifyingkey(pubkey_bytes: &[u8]) -> Result<VerifyingKey> {
    let pubkey = EncodedPoint::from_bytes(&pubkey_bytes).context("fail to get encodedpoint")?;
    VerifyingKey::from_encoded_point(&pubkey).context("fail to get verifying key")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p256() {
        let pvk_hex = "4582b2bf2611f8fe5f7d4e22e20ff19dda42ca630344b33831695c02b616c819";
        let message = "sample";
        let pubkey_hex: &str = "04437203fefbba6922efdfd3b60611f47bbfc7d1472c16506a4ec7f27cec5b3357ec17e87add178dbe6e6eaf3707b2e73c5fa94ed0fb59553ed8ed485e1e6ba3fb";

        assert_eq!(hex::encode(&pubkey(pvk_hex).unwrap()), pubkey_hex);

        let sig_der = sign_der(pvk_hex, message.as_bytes()).unwrap();
        assert!(verify_der(pubkey_hex, message.as_bytes(), &sig_der).is_ok());

        let sig = sign(pvk_hex, message.as_bytes()).unwrap();
        assert!(verify(pubkey_hex, message.as_bytes(), &sig).is_ok())
    }
}
