use anyhow::{bail, Context, Ok, Result};
use p256::{
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
/// use ws-sdk::crypto::secp256r1::sign;
/// let pvk_hex = "C9AFA9D845BA75166B5C215767B1D6934E50C3DB36E89B127B8A622B120F6721";
/// let message = "sample";
/// let sig = sign(pvk_hex, message.as_bytes())?;
/// // sig: "EFD48B2AACB6A8FD1140DD9CD45E81D69D2C877B56AAF991C34D0EA84EAF3716F7CB1C942D657C41D436C7A1B6E29F65F3E900DBB9AFF4064DC4AB2F843ACDA8";
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
/// use ws-sdk::crypto::secp256r1::sign_der;
/// let pvk_hex = "C9AFA9D845BA75166B5C215767B1D6934E50C3DB36E89B127B8A622B120F6721";
/// let message = "sample";
/// let sig_der = sign_der(pvk_hex, message.as_bytes())?;
/// // sig_der: "3046022100efd48b2aacb6a8fd1140dd9cd45e81d69d2c877b56aaf991c34d0ea84eaf3716022100f7cb1c942d657c41d436c7a1b6e29f65f3e900dbb9aff4064dc4ab2f843acda8";
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
/// use ws-sdk::crypto::secp256r1::verify_der;
/// let pbk_hex = "0460fed4ba255a9d31c961eb74c6356d68c049b8923b61fa6ce669622e60f29fb67903fe1008b8bc99a41ae9e95628bc64f2f1b20c2d7e9f5177a3c294d4462299";
/// let message = "sample";
/// let sig = "EFD48B2AACB6A8FD1140DD9CD45E81D69D2C877B56AAF991C34D0EA84EAF3716F7CB1C942D657C41D436C7A1B6E29F65F3E900DBB9AFF4064DC4AB2F843ACDA8";
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
/// use ws-sdk::crypto::secp256r1::verify_der;
/// let pbk_hex = "0460fed4ba255a9d31c961eb74c6356d68c049b8923b61fa6ce669622e60f29fb67903fe1008b8bc99a41ae9e95628bc64f2f1b20c2d7e9f5177a3c294d4462299";
/// let message = "sample";
/// let sig_der = "3046022100efd48b2aacb6a8fd1140dd9cd45e81d69d2c877b56aaf991c34d0ea84eaf3716022100f7cb1c942d657c41d436c7a1b6e29f65f3e900dbb9aff4064dc4ab2f843acda8";
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
/// use ws-sdk::crypto::secp256r1::pubkey;
/// let pvk_hex = "C9AFA9D845BA75166B5C215767B1D6934E50C3DB36E89B127B8A622B120F6721";
/// let pbk_hex = pubkey(pvk_hex)?;
/// // pbk_hex: "0460fed4ba255a9d31c961eb74c6356d68c049b8923b61fa6ce669622e60f29fb67903fe1008b8bc99a41ae9e95628bc64f2f1b20c2d7e9f5177a3c294d4462299";
/// ```
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
        // https://www.rfc-editor.org/rfc/rfc6979#appendix-A.2.5
        let pvk_hex = "C9AFA9D845BA75166B5C215767B1D6934E50C3DB36E89B127B8A622B120F6721";
        let message = "sample";
        let pubkey_hex: &str = "0460fed4ba255a9d31c961eb74c6356d68c049b8923b61fa6ce669622e60f29fb67903fe1008b8bc99a41ae9e95628bc64f2f1b20c2d7e9f5177a3c294d4462299";

        assert_eq!(hex::encode(&pubkey(pvk_hex).unwrap()), pubkey_hex);

        let sig_der = sign_der(pvk_hex, message.as_bytes()).unwrap();
        assert!(verify_der(pubkey_hex, message.as_bytes(), &sig_der).is_ok());

        let sig = sign(pvk_hex, message.as_bytes()).unwrap();
        assert!(verify(pubkey_hex, message.as_bytes(), &sig).is_ok())
    }
}
