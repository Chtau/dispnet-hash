use std::{
    fmt,
    str::{from_utf8, FromStr},
};

#[derive(Debug)]
pub enum HashError {
    Undefined,
    InvalidDigest { hex_digest: String },
    DigestLength { raw_digest_length: String },
    DigestLengthMissmatch { length: usize, digest: Vec<u8> },
}

#[derive(Debug)]
pub struct HashConfig {
    pub salt: Option<Box<Vec<u8>>>,
}

#[derive(Debug, PartialEq)]
pub enum HashType {
    Blake3,
    CRC,
    Argon2,
}

impl fmt::Display for HashType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            HashType::Argon2 => {
                write!(f, "{:02}", 3)
            }
            HashType::CRC => {
                write!(f, "{:02}", 2)
            }
            _ => {
                write!(f, "{:02}", 1)
            }
        }
    }
}

/// Dispnet hash is as self descriping hash format.
///
/// # Display format is structured as followed:
///
/// * First 2 characters are the hash type as integer with a leading 0 (Default is 01 which is Blake3 hash).
/// * Then come 4 characters as integer with leading 0 which is the length of the bytes from the digest.
/// * Digest value as hex.
///
/// # Examles
/// ```
/// fn new_hash() {
///     let dispnet_hash = dispnet_hash::DispnetHash::new("test".as_bytes());
///     let display_hash = format!("{}", dispnet_hash);
///     assert_eq!(display_hash, "010324878ca0425c739fa427f7eda20fe845f6b2e46ba5fe2a14df5b1e32f50603215");
/// }
/// ```
#[derive(Debug)]
pub struct DispnetHash {
    pub hash_type: HashType,
    pub digest_length: usize,
    pub digest_value: Vec<u8>,
    pub digest_encoded: u64,
    value: String,
}

trait Hash {
    fn equal(hash: DispnetHash) -> bool;
    fn upgrade();
}

impl DispnetHash {
    /// Create a hash with the default typ (Blake3).
    pub fn new(value: &[u8]) -> Self {
        DispnetHash::create(HashType::Blake3, value, None)
    }

    /// Create a new dispnet hash.
    /// 
    /// # Usage
    /// ```
    /// use dispnet_hash::{DispnetHash, HashType, HashConfig};
    /// 
    /// fn create_hashes() {
    ///     let dispnet_hash_Balke3 = DispnetHash::create(HashType::Blake3, "test".as_bytes(), None);
    ///     let dispnet_hash_CRC = DispnetHash::create(HashType::CRC, "test".as_bytes(), None);
    ///     let dispnet_hash_Argon2 = DispnetHash::create(HashType::Argon2, "test".as_bytes(), None);
    ///     let dispnet_hash_Argon2_slat = DispnetHash::create(HashType::Argon2, "test".as_bytes(), Some(HashConfig { salt: Some(Box::new(b"12345678".to_vec())) }));
    /// }
    /// ```
    pub fn create(hash_type: HashType, value: &[u8], config: Option<HashConfig>) -> Self {
        let internal_hash = InternalDispnetHash::new(hash_type, value, config);
        let internal_hash_value = format!("{}", internal_hash);
        let encoded: u64 = DispnetHash::encoded_u64(&internal_hash.digest_value);
        Self {
            hash_type: internal_hash.hash_type,
            digest_length: internal_hash.digest_length,
            digest_value: internal_hash.digest_value,
            digest_encoded: encoded,
            value: internal_hash_value,
        }
    }

    /// Verify a dispnet hash string with raw value.
    /// The hash must be created with the Argon2 type
    /// # Usage
    /// ```
    /// use dispnet_hash::{DispnetHash, HashType};
    /// 
    /// fn verify_hash() {
    ///     let dispnet_hash = DispnetHash::create(HashType::Argon2, "test".as_bytes(), None);
    ///     
    ///     DispnetHash::verify(&dispnet_hash.to_string(), "test".as_bytes());
    /// }
    /// ```
    pub fn verify(hash: &str, value: &[u8]) -> bool {
        let dispnet_hash = hash.parse::<DispnetHash>();
        if let Ok(hash) = dispnet_hash {
            return DispnetHash::verify_instance(&hash, value);
        }
        false
    }

    /// Verify a dispnet hash instance with raw value.
    /// The hash must be created with the Argon2 type
    /// # Usage
    /// ```
    /// use dispnet_hash::{DispnetHash, HashType};
    /// 
    /// fn verify_hash_instance() {
    ///     let dispnet_hash = DispnetHash::create(HashType::Argon2, "test".as_bytes(), None);
    ///     
    ///     DispnetHash::verify_instance(&dispnet_hash, "test".as_bytes());
    /// }
    /// ```
    pub fn verify_instance(hash: &DispnetHash, value: &[u8]) -> bool {
        let str_hash = from_utf8(&hash.digest_value).unwrap();
        let matches_result = argon2::verify_encoded(str_hash, value);
        if let Ok(matches) = matches_result {
            return matches;
        }
        false
    }

    fn parse(hash_value: &str) -> Result<Self, HashError> {
        let internal_hash_result = InternalDispnetHash::parse(hash_value);
        if let Ok(internal_hash) = internal_hash_result {
            let internal_hash_value = format!("{}", internal_hash);
            let encoded: u64 = DispnetHash::encoded_u64(&internal_hash.digest_value);
            return Ok(Self {
                hash_type: internal_hash.hash_type,
                digest_length: internal_hash.digest_length,
                digest_value: internal_hash.digest_value,
                digest_encoded: encoded,
                value: internal_hash_value,
            });
        }
        Err(internal_hash_result.err().unwrap())
    }

    /// Convert a hexadecimal string to a vector of bytes.
    /// Returns `None` if the input string has an odd length which makes it an invalid hex string.
    /// # Usage
    /// ```
    /// use dispnet_hash::DispnetHash;
    ///
    /// fn hex_to_bytes() {
    ///     let hex_string = "74657374";
    ///     let bytes = DispnetHash::hex_to_bytes(hex_string).unwrap();
    ///     assert_eq!(bytes, vec![116, 101, 115, 116]);
    /// }
    /// ```
    pub fn hex_to_bytes(s: &str) -> Option<Vec<u8>> {
        if s.len() % 2 == 0 {
            (0..s.len())
                .step_by(2)
                .map(|i| {
                    s.get(i..i + 2)
                        .and_then(|sub| u8::from_str_radix(sub, 16).ok())
                })
                .collect()
        } else {
            None
        }
    }
    
    /// Convert a slice of bytes to a hexadecimal string.
    /// # Usage
    /// ```
    /// use dispnet_hash::DispnetHash;
    ///
    /// fn bytes_to_hex() {
    ///     let bytes = vec![116, 101, 115, 116];
    ///     let hex_string = DispnetHash::bytes_to_hex(&bytes);
    ///     assert_eq!(hex_string, "74657374");
    /// }
    /// ```
    pub fn bytes_to_hex(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{:02x}", b)).collect()
    }

    /// Convert a slice of bytes to a u64 integer.
    /// If the length of the slice is less than 8, it is converted to a u64 integer using little-endian byte order.
    /// Otherwise, the last 8 bytes of the slice are converted to a u64 integer using little-endian byte order.
    /// # Usage
    /// ```
    /// use dispnet_hash::DispnetHash;
    ///
    /// fn encoded_u64() {
    ///     let bytes = vec![0, 0, 0, 0, 0, 0, 0, 1];
    ///     let encoded = DispnetHash::encoded_u64(&bytes);
    ///     assert_eq!(encoded, 72057594037927936);
    /// }
    /// ```
    pub fn encoded_u64(bytes: &[u8]) -> u64 {
        if bytes.len() < 8 {
            let mut b = [0; 8];
            b[..bytes.len()].copy_from_slice(bytes);
            return u64::from_le_bytes(b);
        }
        u64::from_le_bytes(bytes[(bytes.len() - 8)..].try_into().unwrap())
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
    fn new(hash_type: HashType, value: &[u8], config: Option<HashConfig>) -> Self {
        let mut _hash_config: HashConfig = HashConfig { salt: None };
        let mut config_hash_salt: Box<Vec<u8>> =
            Box::new("A8nUz1Pkc0IZ0uJSZNnMlvdLz0T3al5Hjhg2".as_bytes().to_owned());
        let salt: &[u8];

        if let Some(_hash_config) = config {
            if let Some(config_hash_salt_value) = _hash_config.salt {
                config_hash_salt = config_hash_salt_value;
                salt = &(*config_hash_salt);
            } else {
                salt = &(*config_hash_salt);
            }
        } else {
            salt = &(*config_hash_salt);
        }
        match hash_type {
            HashType::Argon2 => {
                let argon2_config = argon2::Config::default();
                let hash = argon2::hash_encoded(value, salt, &argon2_config).unwrap();
                Self {
                    hash_type: HashType::Argon2,
                    digest_length: hash.len(),
                    digest_value: hash.into_bytes().to_vec(),
                }
            }
            HashType::CRC => {
                let crc32 = crc::Crc::<u32>::new(&crc::CRC_32_ISCSI);
                let hash = crc32.checksum(value).to_string();
                Self {
                    hash_type: HashType::CRC,
                    digest_length: hash.len(),
                    digest_value: hash.into_bytes().to_vec(),
                }
            }
            _ => {
                let hash = blake3::hash(value);
                let hash_bytes = hash.as_bytes();
                Self {
                    hash_type: HashType::Blake3,
                    digest_length: hash_bytes.len(),
                    digest_value: hash_bytes.to_vec(),
                }
            }
        }
    }

    fn parse(hash_value: &str) -> Result<Self, HashError> {
        let (raw_type, raw_digest_len_value) = hash_value.split_at(2);
        let (raw_digest_len, raw_digest_value) = raw_digest_len_value.split_at(4);
        let mut type_result = HashType::Blake3;
        let raw_type_result = raw_type.parse::<u8>();
        if let Ok(raw_type) = raw_type_result {
            match raw_type {
                3 => {
                    type_result = HashType::Argon2;
                }
                2 => {
                    type_result = HashType::CRC;
                }
                _ => {
                    type_result = HashType::Blake3;
                }
            }
        } else {
            println!(
                "Invalid hash type raw value:{}. Use Blake3 as fallback!",
                raw_type
            );
        }

        let hex_result = DispnetHash::hex_to_bytes(raw_digest_value);
        if let Some(hash_bytes) = hex_result {
            let digest_len_result = raw_digest_len.parse::<usize>();
            if let Ok(hash_bytes_len) = digest_len_result {
                if hash_bytes_len == hash_bytes.len() {
                    Ok(Self {
                        hash_type: type_result,
                        digest_length: hash_bytes_len,
                        digest_value: hash_bytes,
                    })
                } else {
                    println!(
                        "Length missmatch for digest. Length:{} Digest:{}",
                        hash_bytes_len,
                        hash_bytes.len()
                    );
                    Err(HashError::DigestLengthMissmatch {
                        length: hash_bytes_len,
                        digest: hash_bytes,
                    })
                }
            } else {
                println!("Digest length is not a valid usize:{}", raw_digest_len);
                Err(HashError::DigestLength {
                    raw_digest_length: raw_digest_len.to_owned(),
                })
            }
        } else {
            println!("Invalid digest hex value:{}", raw_digest_value);
            Err(HashError::InvalidDigest {
                hex_digest: raw_digest_value.to_owned(),
            })
        }
    }
}

impl fmt::Display for InternalDispnetHash {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}{:04}{}",
            self.hash_type,
            self.digest_length,
            self.digest_value
                .iter()
                .map(|x| format!("{:02x}", x))
                .collect::<String>()
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{DispnetHash, HashType, HashConfig};

    #[test]
    fn new_hash() {
        let dispnet_hash = DispnetHash::new("test".as_bytes());
        let display_hash = format!("{}", dispnet_hash);
        assert_eq!(display_hash, "0100324878ca0425c739fa427f7eda20fe845f6b2e46ba5fe2a14df5b1e32f50603215");
    }

    #[test]
    fn create_blake3_hash() {
        let dispnet_hash = DispnetHash::create(HashType::Blake3, "test".as_bytes(), None);
        let display_hash = format!("{}", dispnet_hash);
        assert_eq!(display_hash, "0100324878ca0425c739fa427f7eda20fe845f6b2e46ba5fe2a14df5b1e32f50603215");
        assert_eq!(dispnet_hash.digest_encoded, 1527389121149121013);
    }

    #[test]
    fn create_crc32_hash() {
        let dispnet_hash = DispnetHash::create(HashType::CRC, "test".as_bytes(), None);
        let display_hash = format!("{}", dispnet_hash);
        assert_eq!(display_hash, "02001032323538363632303830");
        assert_eq!(dispnet_hash.digest_encoded, 3474580104732358709);
    }

    #[test]
    fn create_argon2_hash() {
        let dispnet_hash = DispnetHash::create(HashType::Argon2, "test".as_bytes(), None);
        let display_hash = format!("{}", dispnet_hash);
        assert_eq!(display_hash, "030121246172676f6e326924763d3139246d3d343039362c743d332c703d31245154687556586f785547746a4d456c614d48564b5531704f626b3173646d524d656a42554d3246734e5568716147637924464d4f7a6f46647754464676397a31435a485751684b7a2f63696f754c55427571494a54756a574d375338");
        assert_eq!(dispnet_hash.digest_encoded, 4058648494509552980);
    }

    #[test]
    fn create_argon2_salt_hash() {
        let dispnet_hash = DispnetHash::create(HashType::Argon2, "test".as_bytes(), Some(HashConfig { salt: Some(Box::new(b"12345678".to_vec())) }));
        let display_hash = format!("{}", dispnet_hash);
        assert_eq!(display_hash, "030084246172676f6e326924763d3139246d3d343039362c743d332c703d31244d54497a4e4455324e7a6724686f56354d494638596a39746b39356c467365546279554a6e393336484944586754685533637065643151");
        assert_eq!(dispnet_hash.digest_encoded, 5850567777771008853);
    }

    #[test]
    fn parse_hash() {
        let dispnet_hash = "0100324878ca0425c739fa427f7eda20fe845f6b2e46ba5fe2a14df5b1e32f50603215".parse::<DispnetHash>().unwrap();
        assert_eq!(dispnet_hash.hash_type, HashType::Blake3);
        assert_eq!(dispnet_hash.digest_length, 32);
        assert_eq!(dispnet_hash.digest_value.len(), 32);
    }

    #[test]
    fn parse_crc32_hash() {
        let dispnet_hash = "02001032323538363632303830".parse::<DispnetHash>().unwrap();
        assert_eq!(dispnet_hash.hash_type, HashType::CRC);
        assert_eq!(dispnet_hash.digest_length, 10);
        assert_eq!(dispnet_hash.digest_value.len(), 10);
    }

    #[test]
    fn parse_argon2_hash() {
        let dispnet_hash = "030121246172676f6e326924763d3139246d3d343039362c743d332c703d31245154687556586f785547746a4d456c614d48564b5531704f626b3173646d524d656a42554d3246734e5568716147637924464d4f7a6f46647754464676397a31435a485751684b7a2f63696f754c55427571494a54756a574d375338".parse::<DispnetHash>().unwrap();
        assert_eq!(dispnet_hash.hash_type, HashType::Argon2);
        assert_eq!(dispnet_hash.digest_length, 121);
        assert_eq!(dispnet_hash.digest_value.len(), 121);
    }

    #[test]
    fn parse_argon2_salt_hash() {
        let dispnet_hash = "030084246172676f6e326924763d3139246d3d343039362c743d332c703d31244d54497a4e4455324e7a6724686f56354d494638596a39746b39356c467365546279554a6e393336484944586754685533637065643151".parse::<DispnetHash>().unwrap();
        assert_eq!(dispnet_hash.hash_type, HashType::Argon2);
        assert_eq!(dispnet_hash.digest_length, 84);
        assert_eq!(dispnet_hash.digest_value.len(), 84);
    }

    #[test]
    fn compare_hash_instances() {
        let dispnet_hash_1 = DispnetHash::new("test".as_bytes());
        let dispnet_hash_2 = DispnetHash::new("test".as_bytes());
        assert_eq!(dispnet_hash_1, dispnet_hash_2);
    }

    #[test]
    fn compare_crc32_hash_instances() {
        let dispnet_hash_1 = DispnetHash::create(HashType::CRC, "test".as_bytes(), None);
        let dispnet_hash_2 = DispnetHash::create(HashType::CRC, "test".as_bytes(), None);
        assert_eq!(dispnet_hash_1, dispnet_hash_2);
    }

    #[test]
    fn compare_argon2_hash_instances() {
        let dispnet_hash_1 = DispnetHash::create(HashType::Argon2, "test".as_bytes(), None);
        let dispnet_hash_2 = DispnetHash::create(HashType::Argon2, "test".as_bytes(), None);
        assert_eq!(dispnet_hash_1, dispnet_hash_2);
    }

    #[test]
    fn compare_argon2_salt_hash_instances() {
        let dispnet_hash_1 = DispnetHash::create(HashType::Argon2, "test".as_bytes(), Some(HashConfig { salt: Some(Box::new(b"12345678".to_vec())) }));
        let dispnet_hash_2 = DispnetHash::create(HashType::Argon2, "test".as_bytes(), Some(HashConfig { salt: Some(Box::new(b"12345678".to_vec())) }));
        assert_eq!(dispnet_hash_1, dispnet_hash_2);
    }

    #[test]
    fn compare_hash_instance_and_prase() {
        let dispnet_hash_1 = DispnetHash::new("test".as_bytes());
        let dispnet_hash_2 = "0100324878ca0425c739fa427f7eda20fe845f6b2e46ba5fe2a14df5b1e32f50603215".parse::<DispnetHash>().unwrap();
        assert_eq!(dispnet_hash_1, dispnet_hash_2);
    }

    #[test]
    fn compare_crc32_hash_instance_and_prase() {
        let dispnet_hash_1 = DispnetHash::create(HashType::CRC, "test".as_bytes(), None);
        let dispnet_hash_2 = "02001032323538363632303830".parse::<DispnetHash>().unwrap();
        assert_eq!(dispnet_hash_1, dispnet_hash_2);
    }

    #[test]
    fn compare_argon2_hash_instance_and_prase() {
        let dispnet_hash_1 = DispnetHash::create(HashType::Argon2, "test".as_bytes(), None);
        let dispnet_hash_2 = "030121246172676f6e326924763d3139246d3d343039362c743d332c703d31245154687556586f785547746a4d456c614d48564b5531704f626b3173646d524d656a42554d3246734e5568716147637924464d4f7a6f46647754464676397a31435a485751684b7a2f63696f754c55427571494a54756a574d375338".parse::<DispnetHash>().unwrap();
        assert_eq!(dispnet_hash_1, dispnet_hash_2);
    }

    #[test]
    fn compare_argon2_salt_hash_instance_and_prase() {
        let dispnet_hash_1 = DispnetHash::create(HashType::Argon2, "test".as_bytes(), Some(HashConfig { salt: Some(Box::new(b"12345678".to_vec())) }));
        let dispnet_hash_2 = "030084246172676f6e326924763d3139246d3d343039362c743d332c703d31244d54497a4e4455324e7a6724686f56354d494638596a39746b39356c467365546279554a6e393336484944586754685533637065643151".parse::<DispnetHash>().unwrap();
        assert_eq!(dispnet_hash_1, dispnet_hash_2);
    }

    #[test]
    fn compare_hash_instance_and_string() {
        let dispnet_hash_1 = DispnetHash::new("test".as_bytes());
        assert_eq!(dispnet_hash_1, "0100324878ca0425c739fa427f7eda20fe845f6b2e46ba5fe2a14df5b1e32f50603215".to_owned());
    }

    #[test]
    fn compare_crc32_hash_instance_and_string() {
        let dispnet_hash_1 = DispnetHash::create(HashType::CRC, "test".as_bytes(), None);
        assert_eq!(dispnet_hash_1, "02001032323538363632303830".to_owned());
    }

    #[test]
    fn verify_argon2_hash() {
        assert!(DispnetHash::verify("030084246172676f6e326924763d3139246d3d343039362c743d332c703d31244d54497a4e4455324e7a6724686f56354d494638596a39746b39356c467365546279554a6e393336484944586754685533637065643151", "test".as_bytes()));
        assert!(!DispnetHash::verify("030084246172676f6e326924763d3139246d3d343039362c743d332c703d31244d54497a4e4455324e7a6724686f56354d494638596a39746b39356c467365546279554a6e393336484944586754685533637065644262", "test".as_bytes()));
    }

    #[test]
    fn hex() {
        assert_eq!(DispnetHash::bytes_to_hex("test".as_bytes()), "74657374");
        assert_eq!(DispnetHash::hex_to_bytes("74657374").unwrap(), "test".as_bytes());
    }

    #[test]
    fn encoded_u64() {
        assert_eq!(DispnetHash::encoded_u64("test".as_bytes()), 1953719668);
        assert_eq!(DispnetHash::encoded_u64("a".as_bytes()), 97);
        assert_eq!(DispnetHash::encoded_u64("aasdsakdljaslfhaksjhuahwiuewasdfgs4354sg".as_bytes()), 7454359211325289319);
    }
}
