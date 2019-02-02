use chrono::Utc;

use crate::models::AccessToken;

/// Trait untuk memastikan apakah suatu object
/// bisa divalidasi atau tidak.
pub trait Validable {
    /// Periksa kevalidan object.
    fn valid(&self) -> bool;
}

impl Validable for AccessToken {
    fn valid(&self) -> bool {
        let now = Utc::now().naive_utc();
        now < self.valid_thru
    }
}

#[cfg(test)]
mod tests {
    use super::Validable;
    use crate::models::AccessToken;
    use chrono::{Duration, Utc};
    use std::ops::Add;
    use std::{thread::sleep, time};

    #[test]
    fn test_access_token_valid() {
        let access_token = AccessToken {
            token: "".to_owned(),
            account_id: 1,
            created: Utc::now().naive_utc(),
            valid_thru: Utc::now().naive_utc().add(Duration::days(1)),
        };
        sleep(time::Duration::from_millis(1000));
        assert!(access_token.valid());
    }
}
