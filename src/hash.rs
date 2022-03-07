use std::{fmt, str::FromStr};

#[derive(Debug)]
pub enum HashError {
    Undefined,
    InvalidDigest { hex_digest: String },
    DigestLength { raw_digest_length: String },
    DigestLengthMissmatch { length: usize, digest: Vec<u8> },
}

#[derive(Debug, PartialEq)]
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

/// Dispnet hash is as self descriping hash format.
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
///     let dispnet_hash = DispnetHash::new("test".as_bytes());
///     let display_hash = format!("{}", dispnet_hash);
///     assert_eq!(display_hash, "010324878ca0425c739fa427f7eda20fe845f6b2e46ba5fe2a14df5b1e32f50603215");
/// }
/// ```
#[derive(Debug)]
pub struct DispnetHash {
    pub hash_type: HashType,
    pub digest_length: usize,
    pub digest_value: Vec<u8>,
    value: String,
}

trait Hash {
    fn equal(hash: DispnetHash) -> bool;
    fn upgrade();
}

impl DispnetHash {
    pub fn new(value: &[u8]) -> Self {
        let internal_hash = InternalDispnetHash::new(value);
        let internal_hash_value = format!("{}", internal_hash);
        Self {
            hash_type: internal_hash.hash_type,
            digest_length: internal_hash.digest_length,
            digest_value: internal_hash.digest_value,
            value: internal_hash_value,
        }
    }

    fn parse(hash_value: &str) -> Result<Self, HashError> {
        let internal_hash_result = InternalDispnetHash::parse(hash_value);
        if internal_hash_result.is_ok() {
            let internal_hash = internal_hash_result.unwrap();
            let internal_hash_value = format!("{}", internal_hash);
            return Ok(Self {
                hash_type: internal_hash.hash_type,
                digest_length: internal_hash.digest_length,
                digest_value: internal_hash.digest_value,
                value: internal_hash_value,
            });
        }
        return Err(internal_hash_result.err().unwrap());
    }
}

impl fmt::Display for DispnetHash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl PartialEq for DispnetHash {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl PartialEq<String> for DispnetHash {
    fn eq(&self, other: &String) -> bool {
        self.value == *other
    }
}

impl FromStr for DispnetHash {
    type Err = HashError;

    fn from_str(s: &str) -> Result<Self, HashError> {
        DispnetHash::parse(s)
    }
}

#[derive(Debug)]
struct InternalDispnetHash {
    pub hash_type: HashType,
    pub digest_length: usize,
    pub digest_value: Vec<u8>,
}

impl InternalDispnetHash {
    fn new(value: &[u8]) -> Self {
        let hash = blake3::hash(&value);
        let hash_bytes = hash.as_bytes();
        Self {
            hash_type: HashType::Blake3,
            digest_length: hash_bytes.len(),
            digest_value: hash_bytes.to_vec(),
        }
    }

    fn parse(hash_value: &str) -> Result<Self, HashError> {
        let (raw_type,  raw_digest_len_value) = hash_value.split_at(2);
        let (raw_digest_len, raw_digest_value) = raw_digest_len_value.split_at(3);
        let mut type_result = HashType::Blake3;
        let raw_type_result = raw_type.parse::<u8>();
        if raw_type_result.is_ok() {
            match raw_type_result.unwrap() {
                _ => {
                    type_result = HashType::Blake3;
                }
            }
        } else {
            println!("Invalid hash type raw value:{}. Use Blake3 as fallback!", raw_type);
        }

        let hex_result = hex_to_bytes(raw_digest_value);
        if hex_result.is_some() {
            let hash_bytes = hex_result.unwrap();
            let digest_len_result = raw_digest_len.parse::<usize>();
            if digest_len_result.is_ok() {
                let hash_bytes_len = digest_len_result.unwrap();
                if hash_bytes_len == hash_bytes.len() {
                    return Ok(Self {
                        hash_type: type_result,
                        digest_length: hash_bytes_len,
                        digest_value: hash_bytes,
                    });
                } else {
                    println!("Length missmatch for digest. Length:{} Digest:{}", hash_bytes_len, hash_bytes.len());
                    return Err(HashError::DigestLengthMissmatch { length: hash_bytes_len, digest: hash_bytes });
                }
            } else {
                println!("Digest length is not a valid usize:{}", raw_digest_len);
                return Err(HashError::DigestLength { raw_digest_length: raw_digest_len.to_owned() });
            }
        } else {
            println!("Invalid digest hex value:{}", raw_digest_value);
            return Err(HashError::InvalidDigest { hex_digest: raw_digest_value.to_owned() });
        }
    }
}

fn hex_to_bytes(s: &str) -> Option<Vec<u8>> {
    if s.len() % 2 == 0 {
        (0..s.len())
            .step_by(2)
            .map(|i| s.get(i..i + 2).and_then(|sub| u8::from_str_radix(sub, 16).ok()))
            .collect()
    } else {
        None
    }
}

impl fmt::Display for InternalDispnetHash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{:03}{}", self.hash_type, self.digest_length, self.digest_value.iter().map(|x| format!("{:02x}", x)).collect::<String>())
    }
}