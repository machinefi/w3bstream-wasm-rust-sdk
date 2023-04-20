use anyhow::{bail, Context, Ok, Result};
use k256::{
    ecdsa::{
        signature::Signer, signature::Verifier, DerSignature, Signature, SigningKey, VerifyingKey,
    },
    EncodedPoint,
};

/// get the signature by signing the payload with the private key.
///
/// The signature is consist of `{r, s}` pair; the private key is in hex form; and the payload is in bytes array.
///
/// # Examples
///
/// ```no_run
/// use ws-sdk::crypto::secp256k1::sign;
/// let pvk_hex = "4582b2bf2611f8fe5f7d4e22e20ff19dda42ca630344b33831695c02b616c819";
/// let message = "sample";
/// let sig = sign(pvk_hex, message.as_bytes())?;
/// // sig: "C66DC6ECC0D24B5D0A8143E42B332BF8FC36DAE40D094C0C2967AAFDAC92C8130E8FA34CCB99DD001D7740B6EADA3892EA87741733911B32CE2F6AF25C3FD082";
/// ```
pub fn sign(prikey_hex: &str, data_bytes: &[u8]) -> Result<String> {
    let sig = sign_the_message(prikey_hex, data_bytes)?;
    Ok(sig.to_string())
}

/// get the DER-encoded signature by signing the payload with the private key.
///
/// The signature is [DER] encoded; the private key is in hex form; and the payload is in bytes array.
///
/// [DER]: https://en.wikipedia.org/wiki/X.690#DER_encoding
///
/// # Examples
///
/// ```no_run
/// use ws-sdk::crypto::secp256k1::sign_der;
/// let pvk_hex = "4582b2bf2611f8fe5f7d4e22e20ff19dda42ca630344b33831695c02b616c819";
/// let message = "sample";
/// let sig_der = sign_der(pvk_hex, message.as_bytes())?;
/// // sig_der: "3045022100c66dc6ecc0d24b5d0a8143e42b332bf8fc36dae40d094c0c2967aafdac92c81302200e8fa34ccb99dd001d7740b6eada3892ea87741733911b32ce2f6af25c3fd082";
/// ```
pub fn sign_der(prikey_hex: &str, data_bytes: &[u8]) -> Result<String> {
    let sig = sign_the_message(prikey_hex, data_bytes)?;
    Ok(hex::encode(sig.to_der()))
}

/// verify the signature with the public key and the payload.
///
/// The signature is consist of `{r, s}` pair; the public key is in hex form; and the payload is in bytes array.
///
/// # Examples
///
/// ```no_run
/// use ws-sdk::crypto::secp256k1::verify_der;
/// let pbk_hex = "04437203fefbba6922efdfd3b60611f47bbfc7d1472c16506a4ec7f27cec5b3357ec17e87add178dbe6e6eaf3707b2e73c5fa94ed0fb59553ed8ed485e1e6ba3fb";
/// let message = "sample";
/// let sig = "C66DC6ECC0D24B5D0A8143E42B332BF8FC36DAE40D094C0C2967AAFDAC92C8130E8FA34CCB99DD001D7740B6EADA3892EA87741733911B32CE2F6AF25C3FD082";
/// let res = verify(pbk_hex, message.as_bytes(), sig);
/// ```
pub fn verify(pubkey_hex: &str, data_bytes: &[u8], sig_hex: &str) -> Result<()> {
    let sig = get_raw_signature(&hex::decode(sig_hex)?)?;

    let pbk = get_verifyingkey(&hex::decode(pubkey_hex)?)?;

    match pbk.verify(data_bytes, &sig) {
        Err(err) => bail!(err.to_string()),
        _ => return Ok(()),
    }
}

/// verify the DER-encoded signature with the public key and the payload.
///
/// The signature is [DER] encoded; the public key is in hex form; and the payload is in bytes array.
///
/// [DER]: https://en.wikipedia.org/wiki/X.690#DER_encoding
///
/// # Examples
///
/// ```no_run
/// use ws-sdk::crypto::secp256k1::verify_der;
/// let pbk_hex = "04437203fefbba6922efdfd3b60611f47bbfc7d1472c16506a4ec7f27cec5b3357ec17e87add178dbe6e6eaf3707b2e73c5fa94ed0fb59553ed8ed485e1e6ba3fb";
/// let message = "sample";
/// let sig_der = "3045022100c66dc6ecc0d24b5d0a8143e42b332bf8fc36dae40d094c0c2967aafdac92c81302200e8fa34ccb99dd001d7740b6eada3892ea87741733911b32ce2f6af25c3fd082";
/// let res = verify_der(pbk_hex, message.as_bytes(), sig_der);
/// ```
pub fn verify_der(pubkey_hex: &str, data_bytes: &[u8], sig_hex: &str) -> Result<()> {
    let sig = get_der_signature(&hex::decode(sig_hex)?)?;

    let pbk = get_verifyingkey(&hex::decode(pubkey_hex)?)?;

    match pbk.verify(data_bytes, &sig) {
        Err(err) => bail!(err.to_string()),
        _ => return Ok(()),
    }
}

/// generate the public key from the private key.
///
/// Both public key and private key are in hex form
///
/// # Examples
///
/// ```no_run
/// use ws-sdk::crypto::secp256k1::pubkey;
/// let pvk_hex = "4582b2bf2611f8fe5f7d4e22e20ff19dda42ca630344b33831695c02b616c819";
/// let pbk_hex = pubkey(pvk_hex)?;
/// // pbk_hex: "04437203fefbba6922efdfd3b60611f47bbfc7d1472c16506a4ec7f27cec5b3357ec17e87add178dbe6e6eaf3707b2e73c5fa94ed0fb59553ed8ed485e1e6ba3fb";
/// ```
pub fn pubkey(prikey_hex: &str) -> Result<String> {
    let signer =
        SigningKey::from_slice(&hex::decode(prikey_hex)?).context("fail to get the private key")?;
    let pbk = signer.verifying_key().to_encoded_point(false);
    Ok(hex::encode(pbk.to_bytes().to_vec()))
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
    fn test_k256() {
        let pvk_hex = "4582b2bf2611f8fe5f7d4e22e20ff19dda42ca630344b33831695c02b616c819";
        let message = "sample";
        let pbk_hex: &str = "04437203fefbba6922efdfd3b60611f47bbfc7d1472c16506a4ec7f27cec5b3357ec17e87add178dbe6e6eaf3707b2e73c5fa94ed0fb59553ed8ed485e1e6ba3fb";

        assert_eq!(&pubkey(pvk_hex).unwrap(), pbk_hex);

        let sig_der = sign_der(pvk_hex, message.as_bytes()).unwrap();
        assert!(verify_der(pbk_hex, message.as_bytes(), &sig_der).is_ok());

        let sig = sign(pvk_hex, message.as_bytes()).unwrap();
        assert!(verify(pbk_hex, message.as_bytes(), &sig).is_ok())
    }
}
