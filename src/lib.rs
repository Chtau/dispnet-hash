pub mod hash;

#[cfg(test)]
mod tests {
    use crate::hash::{ForwardHash, HashType};

    #[test]
    fn new_hash() {
        let forward_hash = ForwardHash::new("test".as_bytes());
        let display_hash = format!("{}", forward_hash);
        assert_eq!(display_hash, "010324878ca0425c739fa427f7eda20fe845f6b2e46ba5fe2a14df5b1e32f50603215");
    }

    #[test]
    fn parse_hash() {
        let forward_hash = ForwardHash::parse("010324878ca0425c739fa427f7eda20fe845f6b2e46ba5fe2a14df5b1e32f50603215".to_owned()).unwrap();
        assert_eq!(forward_hash.hash_type, HashType::Blake3);
        assert_eq!(forward_hash.digest_length, 32);
        assert_eq!(forward_hash.digest_value.len(), 32);
    }

    #[test]
    fn compare_hash_instances() {
        let forward_hash_1 = ForwardHash::new("test".as_bytes());
        let forward_hash_2 = ForwardHash::new("test".as_bytes());
        assert_eq!(forward_hash_1, forward_hash_2);
    }

    #[test]
    fn compare_hash_instance_and_prase() {
        let forward_hash_1 = ForwardHash::new("test".as_bytes());
        let forward_hash_2 = ForwardHash::parse("010324878ca0425c739fa427f7eda20fe845f6b2e46ba5fe2a14df5b1e32f50603215".to_owned()).unwrap();
        assert_eq!(forward_hash_1, forward_hash_2);
    }

    #[test]
    fn compare_hash_instance_and_string() {
        let forward_hash_1 = ForwardHash::new("test".as_bytes());
        assert_eq!(forward_hash_1, "010324878ca0425c739fa427f7eda20fe845f6b2e46ba5fe2a14df5b1e32f50603215".to_owned());
    }
}
