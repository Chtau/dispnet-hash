use std::fmt;

#[derive(Debug)]
pub enum HashType {
    Blake3
}

impl fmt::Display for HashType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            _ => {
                return write!(f, "{:02}", 1);
            }
        }
    }
}

/// Forward hash is as self descriping hash format.
/// 
/// # Display format is structured as followed:
/// 
/// * First 2 characters are the hash type as integer with a leading 0 (Default is 01 which is Blake3 hash).
/// * Then come 3 characters as integer with leading 0 which is the length of the bytes from the digest.
/// * Digest value as hex.
/// 
/// # Examles
/// ```
/// fn new_hash() {
///     let forward_hash = ForwardHash::new("test".as_bytes());
///     let display_hash = format!("{}", forward_hash);
///     assert_eq!(display_hash, "010324878ca0425c739fa427f7eda20fe845f6b2e46ba5fe2a14df5b1e32f50603215");
/// }
/// ```
#[derive(Debug)]
pub struct ForwardHash {
    pub hash_type: HashType,
    pub digest_length: usize,
    pub digest_value: Vec<u8>,
    value: String,
}

impl ForwardHash {
    pub fn new(value: &[u8]) -> Self {
        let internal_hash = InternalForwardHash::new(value);
        let internal_hash_value = format!("{}", internal_hash);
        Self {
            hash_type: internal_hash.hash_type,
            digest_length: internal_hash.digest_length,
            digest_value: internal_hash.digest_value,
            value: internal_hash_value,
        }
    }
}

impl fmt::Display for ForwardHash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug)]
struct InternalForwardHash {
    pub hash_type: HashType,
    pub digest_length: usize,
    pub digest_value: Vec<u8>,
}

impl InternalForwardHash {
    fn new(value: &[u8]) -> Self {
        let hash = blake3::hash(&value);
        let hash_bytes = hash.as_bytes();
        Self {
            hash_type: HashType::Blake3,
            digest_length: hash_bytes.len(),
            digest_value: hash_bytes.to_vec(),
        }
    }
}

impl fmt::Display for InternalForwardHash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{:03}{}", self.hash_type, self.digest_length, self.digest_value.iter().map(|x| format!("{:02x}", x)).collect::<String>())
    }
}