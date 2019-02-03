//! Utilitas berkaitan dengan kriptografi.
//! Misalnya untuk mendapatkan passhash dari plain password.

use hex;
use sodiumoxide::crypto::hash::sha256;

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

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_pass_hash() {
        assert_eq!(
            super::get_passhash("123"),
            "c4f79e6453e740fadae0e333a48888529f5cc10e7769491430fdcddff94d2f8f"
        );
    }
}
