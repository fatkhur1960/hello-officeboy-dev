//! Utilitas berkaitan dengan kriptografi.
//! Misalnya untuk mendapatkan passhash dari plain password.

use hex;
use sodiumoxide::crypto::hash;

/// Mendapatkan passhash dari sebuah password.
pub fn get_passhash(password: &str) -> String {
    hex::encode(hash::hash(password.as_bytes()))
}
