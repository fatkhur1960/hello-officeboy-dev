//! Utilitas berkaitan dengan kriptografi.
//! Misalnya untuk mendapatkan passhash dari plain password,
//! menggenerasikan pasangan kunci (keypair) asimetris,
//! melakukan signing pada data, dll.

// pub(crate) use crate::crypto_impl::crypto::sign::ed25519 as ds;
// use crate::crypto_impl::crypto::{self, hash::sha256};
use ed25519_dalek::Keypair;
use hex;
use rand::thread_rng;
use sha2::{Digest, Sha256, Sha512};

/// Number of bytes in a public key.
pub const PUBLIC_KEY_LENGTH: usize = ed25519_dalek::PUBLIC_KEY_LENGTH;
/// Number of bytes in a secret key.
pub const SECRET_KEY_LENGTH: usize = ed25519_dalek::SECRET_KEY_LENGTH; //ds::SECRETKEYBYTES;
/// Number of bytes in a `Hash`.
pub const HASH_SIZE: usize = 32; //sha256::DIGESTBYTES;
/// Number of bytes in a signature.
pub const SIGNATURE_LENGTH: usize = ed25519_dalek::SIGNATURE_LENGTH;

// Buatkan wrapper untuk object-object internal dari crypto_impl
// agar lebih flexibel kita bisa menambahkan implementasi sendiri.

implement_crypto_wrapper!(
    struct PublicKey, ed25519_dalek::PublicKey, PublicKey, PUBLIC_KEY_LENGTH
);
implement_crypto_wrapper!(
    struct SecretKey, ed25519_dalek::SecretKey, SecretKey, SECRET_KEY_LENGTH
);
implement_crypto_wrapper!(
    struct Signature, ed25519_dalek::Signature, Signature, SIGNATURE_LENGTH
);
// implement_crypto_wrapper!(
//     struct Hash, sha2::Digest, Digest, HASH_SIZE
// );

/// Hash
pub struct Hash([u8; HASH_SIZE]);

impl Hash {
    /// Encode to hex string
    pub fn to_hex(&self) -> String {
        hex::encode(self.0)
    }
}

impl AsRef<[u8; HASH_SIZE]> for Hash {
    fn as_ref(&self) -> &[u8; HASH_SIZE] {
        &self.0
    }
}

impl PublicKey {
    /// Memastikan signature valid untuk message dengan cara memverifikasi
    /// digital signature menggunakan public-key ini.
    pub fn valid(&self, message: &[u8], signature: &Signature) -> bool {
        let raw_pubkey =
            ed25519_dalek::PublicKey::from_bytes(&self.0).expect("Cannot parse bytes for public key");
        raw_pubkey.verify::<Sha512>(message, &signature.into()).is_ok()
    }
}

impl<'a> std::convert::Into<ed25519_dalek::Signature> for &'a Signature {
    fn into(self) -> ed25519_dalek::Signature {
        ed25519_dalek::Signature::from_bytes(&self.0).unwrap()
    }
}

/// Mendapatkan passhash dari sebuah password.
/// Kalkulasi passhash ini menggunakan sha256 yang diproses
/// sebanyak 9 kali.
pub fn get_passhash(password: &str) -> String {
    let mut hash = sha256_hash(password.as_bytes());
    for i in 0..9 {
        hash = sha256_hash(hash.as_ref());
    }
    hex::encode(hash.as_ref())
}

/// Generate key pair
pub fn gen_keypair() -> (PublicKey, SecretKey) {
    let mut csprng = thread_rng();
    let keypair = Keypair::generate::<Sha512, _>(&mut csprng);

    (
        PublicKey::new(keypair.public.to_bytes()),
        SecretKey::new(keypair.secret.to_bytes()),
    )
}

/// Hash some str text
pub fn hash_str(text: &str) -> Hash {
    sha256_hash(&text.as_bytes())
}

/// Get hash sha256 from bytes
pub fn sha256_hash(bytes: &[u8]) -> Hash {
    let mut hasher = Sha256::new();
    hasher.input(bytes);
    let hash = hasher.result().to_vec();

    let mut fixed: [u8; HASH_SIZE] = Default::default();
    fixed.copy_from_slice(hash.as_slice());
    Hash(fixed)
}

/// Get hash sha256 from bytes
pub fn sha256_hash_raw(bytes: &[u8]) -> [u8; HASH_SIZE] {
    let mut hasher = Sha256::new();
    hasher.input(bytes);
    let hash = hasher.result().to_vec();

    let mut fixed: [u8; HASH_SIZE] = Default::default();
    fixed.copy_from_slice(hash.as_slice());
    fixed
}

/// Get hash sha512 from bytes
pub fn sha512_hash_raw(bytes: &[u8]) -> [u8; 64] {
    let mut hasher = Sha512::new();
    hasher.input(bytes);
    let hash = hasher.result().to_vec();

    let mut fixed: [u8; 64] = [0u8; 64];
    fixed.copy_from_slice(hash.as_slice());
    fixed
}

/// Sign a data in bytes, return Signature.
pub fn sign(bytes: &[u8], secret_key: &SecretKey) -> Signature {
    let keypair = get_raw_keypair_from_secret(secret_key);
    let raw_signature = keypair.sign::<Sha512>(bytes);

    Signature(raw_signature.to_bytes())
}

fn get_raw_keypair_from_secret(secret_key: &SecretKey) -> ed25519_dalek::Keypair {
    let raw_secret_key = ed25519_dalek::SecretKey::from_bytes(&secret_key.0).expect("to raw secret key");
    let public_key = ed25519_dalek::PublicKey::from_secret::<Sha512>(&raw_secret_key);
    ed25519_dalek::Keypair {
        secret: raw_secret_key,
        public: public_key,
    }
}

/// Memverifikasi digital signature apakah cocok dengan data dan public key-nya.
pub fn is_verified(message: &[u8], signature: &Signature, pub_key: &PublicKey) -> bool {
    // ds::verify_detached(&signature.0, bytes, &pub_key.0)
    // @TODO(robin): Code here
    // unimplemented!();
    pub_key.valid(message, signature)
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
            "db70a045a13645e1c0e227f0c4097e58880d5c4b227fb4d5ff448425ebf7b90d"
                .parse::<PublicKey>()
                .unwrap(),
            "fa1c6ed8bd8d3e88a84561fc60ae3f205a3c6538f9d2883597524a374f1aa969"
                .parse::<SecretKey>()
                .unwrap(),
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

        assert!(super::is_verified(DATA, &signature, &p));
    }
}
