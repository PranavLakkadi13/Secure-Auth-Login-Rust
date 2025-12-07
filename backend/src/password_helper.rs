use sha2::{Digest, Sha256};

// we are usually supposed to add salt to the password to avoid hash collision
pub fn hash_password(password: String) -> String {
    let mut hasher = Sha256::new();

    hasher.update(password);
    let hash_pass = hasher.finalize();

    hex::encode(hash_pass)
}

#[cfg(test)]
mod test {
    use super::hash_password;

    #[test]
    fn test_hash() {
        assert_eq!(
            hash_password("pranav".to_string()).as_bytes(),
            "a260da9a7a61d18c5404948e4b1f54973f8356a303237b52ffc4aac3f26017fd".as_bytes()
        )
    }
}
