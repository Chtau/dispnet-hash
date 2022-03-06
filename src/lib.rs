pub mod hash;

#[cfg(test)]
mod tests {
    use crate::hash::{DispnetHash, HashType};

    #[test]
    fn new_hash() {
        let dispnet_hash = DispnetHash::new("test".as_bytes());
        let display_hash = format!("{}", dispnet_hash);
        assert_eq!(display_hash, "010324878ca0425c739fa427f7eda20fe845f6b2e46ba5fe2a14df5b1e32f50603215");
    }

    #[test]
    fn parse_hash() {
        let dispnet_hash = DispnetHash::parse("010324878ca0425c739fa427f7eda20fe845f6b2e46ba5fe2a14df5b1e32f50603215".to_owned()).unwrap();
        assert_eq!(dispnet_hash.hash_type, HashType::Blake3);
        assert_eq!(dispnet_hash.digest_length, 32);
        assert_eq!(dispnet_hash.digest_value.len(), 32);
    }

    #[test]
    fn compare_hash_instances() {
        let dispnet_hash_1 = DispnetHash::new("test".as_bytes());
        let dispnet_hash_2 = DispnetHash::new("test".as_bytes());
        assert_eq!(dispnet_hash_1, dispnet_hash_2);
    }

    #[test]
    fn compare_hash_instance_and_prase() {
        let dispnet_hash_1 = DispnetHash::new("test".as_bytes());
        let dispnet_hash_2 = DispnetHash::parse("010324878ca0425c739fa427f7eda20fe845f6b2e46ba5fe2a14df5b1e32f50603215".to_owned()).unwrap();
        assert_eq!(dispnet_hash_1, dispnet_hash_2);
    }

    #[test]
    fn compare_hash_instance_and_string() {
        let dispnet_hash_1 = DispnetHash::new("test".as_bytes());
        assert_eq!(dispnet_hash_1, "010324878ca0425c739fa427f7eda20fe845f6b2e46ba5fe2a14df5b1e32f50603215".to_owned());
    }
}
