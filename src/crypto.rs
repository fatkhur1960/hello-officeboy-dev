//! Utilitas berkaitan dengan kriptografi.
//! Misalnya untuk mendapatkan passhash dari plain password,
//! menggenerasikan pasangan kunci (keypair) asimetris,
//! melakukan signing pada data, dll.

pub(crate) use crate::crypto_impl::crypto::sign::ed25519 as ds;
use crate::crypto_impl::crypto::{self, hash::sha256};
use hex;

/// Number of bytes in a public key.
pub const PUBLIC_KEY_LENGTH: usize = ds::PUBLICKEYBYTES;
/// Number of bytes in a secret key.
pub const SECRET_KEY_LENGTH: usize = ds::SECRETKEYBYTES;
/// Number of bytes in a `Hash`.
pub const HASH_SIZE: usize = sha256::DIGESTBYTES;
/// Number of bytes in a signature.
pub const SIGNATURE_LENGTH: usize = ds::SIGNATUREBYTES;

// Buatkan wrapper untuk object-object internal dari crypto_impl
// agar lebih flexibel kita bisa menambahkan implementasi.

implement_crypto_wrapper!(
    struct PublicKey, PUBLIC_KEY_LENGTH
);
implement_crypto_wrapper!(
    struct SecretKey, SECRET_KEY_LENGTH
);
implement_crypto_wrapper!(
    struct Signature, SIGNATURE_LENGTH
);
implement_crypto_wrapper!(
    struct Hash, crate::crypto::sha256::Digest, Digest, HASH_SIZE
);

/// Mendapatkan passhash dari sebuah password.
/// Kalkulasi passhash ini menggunakan sha256 yang diproses
/// sebanyak 9 kali.
pub fn get_passhash(password: &str) -> String {
    let mut hash = sha256::hash(password.as_bytes());
    for i in 0..9 {
        hash = sha256::hash(hash.as_ref());
    }
    hex::encode(hash)
}

/// Generate key pair
pub fn gen_keypair() -> (PublicKey, SecretKey) {
    let (p, s) = ds::gen_keypair();
    (PublicKey::new(p.0), SecretKey::new(s.0))
}

/// Hash some str text
pub fn hash_str(text: &str) -> Hash {
    hash_bytes(&text.as_bytes())
}

/// Hash byte data
pub fn hash_bytes(bytes: &[u8]) -> Hash {
    Hash(sha256::hash(bytes))
}

/// Sign a data in bytes, return Signature.
pub fn sign(bytes: &[u8], secret_key: &SecretKey) -> Signature {
    let signature = ds::sign_detached(bytes, &secret_key.0);
    Signature(signature)
}

/// Memverifikasi digital signature apakah cocok dengan data dan public key-nya.
pub fn verify(bytes: &[u8], signature: &Signature, pub_key: &PublicKey) -> bool {
    ds::verify_detached(&signature.0, bytes, &pub_key.0)
}

#[cfg(test)]
mod tests {
    use super::{PublicKey, SecretKey, Signature};

    #[test]
    fn test_get_pass_hash() {
        assert_eq!(
            super::get_passhash("123"),
            "c4f79e6453e740fadae0e333a48888529f5cc10e7769491430fdcddff94d2f8f"
        );
    }

    #[test]
    fn test_gen_keyppair() {
        let (p, s) = super::gen_keypair();
        println!("{} -> {}", p.to_hex(), s.to_hex());
        assert_ne!(p.to_hex(), s.to_hex());
        assert_ne!(p.to_hex(), "".to_string());
        assert_ne!(s.to_hex(), "".to_string());
    }

    #[test]
    fn test_hash() {
        let h = super::hash_str("Zufar");
        assert_eq!(
            h.to_hex(),
            "96d301802cf09936d0aa746c5de12b2f2085bd878f2c1e43ebad6074650f218a".to_string()
        );
    }

    const DATA: &'static [u8] = b"Zufar";

    fn get_preset_keypair() -> (PublicKey, SecretKey) {
        (
            "db70a045a13645e1c0e227f0c4097e58880d5c4b227fb4d5ff448425ebf7b90d".parse::<PublicKey>().unwrap(),
            "fa1c6ed8bd8d3e88a84561fc60ae3f205a3c6538f9d2883597524a374f1aa969\
             db70a045a13645e1c0e227f0c4097e58880d5c4b227fb4d5ff448425ebf7b90d".parse::<SecretKey>().unwrap()
        )
    }

    fn create_signature() -> Signature {
        let (p, s) = get_preset_keypair();
        super::sign(DATA, &s)
    }

    #[test]
    fn test_create_signature() {
        let signature = create_signature();

        println!("signature: {}", signature.to_hex());

        assert_eq!(signature.to_hex(), "e5628fac8dfd4d61da9bdca8c63e1ba81447d6151d0017daee4b35146df688f1\
                                        889f7ee7b06fe87bb1a385bbe1f6437aa3463566fbf32d31c267e1f6717c7f0d");
    }

    #[test]
    fn test_verify_signature() {
        let (p, _) = get_preset_keypair();
        let signature = create_signature();

        assert!(super::verify(DATA, &signature, &p));
    }
}
