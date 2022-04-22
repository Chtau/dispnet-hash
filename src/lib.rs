pub mod hash;

#[cfg(test)]
mod tests {
    use crate::hash::{DispnetHash, HashType, HashConfig};

    #[test]
    fn new_hash() {
        let dispnet_hash = DispnetHash::new("test".as_bytes());
        let display_hash = format!("{}", dispnet_hash);
        assert_eq!(display_hash, "010324878ca0425c739fa427f7eda20fe845f6b2e46ba5fe2a14df5b1e32f50603215");
    }

    #[test]
    fn create_crc32_hash() {
        let dispnet_hash = DispnetHash::create(HashType::CRC, "test".as_bytes(), None);
        let display_hash = format!("{}", dispnet_hash);
        assert_eq!(display_hash, "0201032323538363632303830");
    }

    #[test]
    fn create_argon2_hash() {
        let dispnet_hash = DispnetHash::create(HashType::Argon2, "test".as_bytes(), None);
        let display_hash = format!("{}", dispnet_hash);
        assert_eq!(display_hash, "03121246172676f6e326924763d3139246d3d343039362c743d332c703d31245154687556586f785547746a4d456c614d48564b5531704f626b3173646d524d656a42554d3246734e5568716147637924464d4f7a6f46647754464676397a31435a485751684b7a2f63696f754c55427571494a54756a574d375338");
    }

    #[test]
    fn create_argon2_salt_hash() {
        let dispnet_hash = DispnetHash::create(HashType::Argon2, "test".as_bytes(), Some(HashConfig { salt: Some(Box::new(b"12345678".to_vec())) }));
        let display_hash = format!("{}", dispnet_hash);
        assert_eq!(display_hash, "03084246172676f6e326924763d3139246d3d343039362c743d332c703d31244d54497a4e4455324e7a6724686f56354d494638596a39746b39356c467365546279554a6e393336484944586754685533637065643151");
    }

    #[test]
    fn parse_hash() {
        let dispnet_hash = "010324878ca0425c739fa427f7eda20fe845f6b2e46ba5fe2a14df5b1e32f50603215".parse::<DispnetHash>().unwrap();
        assert_eq!(dispnet_hash.hash_type, HashType::Blake3);
        assert_eq!(dispnet_hash.digest_length, 32);
        assert_eq!(dispnet_hash.digest_value.len(), 32);
    }

    #[test]
    fn parse_crc32_hash() {
        let dispnet_hash = "0201032323538363632303830".parse::<DispnetHash>().unwrap();
        assert_eq!(dispnet_hash.hash_type, HashType::CRC);
        assert_eq!(dispnet_hash.digest_length, 10);
        assert_eq!(dispnet_hash.digest_value.len(), 10);
    }

    #[test]
    fn parse_argon2_hash() {
        let dispnet_hash = "03121246172676f6e326924763d3139246d3d343039362c743d332c703d31245154687556586f785547746a4d456c614d48564b5531704f626b3173646d524d656a42554d3246734e5568716147637924464d4f7a6f46647754464676397a31435a485751684b7a2f63696f754c55427571494a54756a574d375338".parse::<DispnetHash>().unwrap();
        assert_eq!(dispnet_hash.hash_type, HashType::Argon2);
        assert_eq!(dispnet_hash.digest_length, 121);
        assert_eq!(dispnet_hash.digest_value.len(), 121);
    }

    #[test]
    fn parse_argon2_salt_hash() {
        let dispnet_hash = "03084246172676f6e326924763d3139246d3d343039362c743d332c703d31244d54497a4e4455324e7a6724686f56354d494638596a39746b39356c467365546279554a6e393336484944586754685533637065643151".parse::<DispnetHash>().unwrap();
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
        let dispnet_hash_2 = "010324878ca0425c739fa427f7eda20fe845f6b2e46ba5fe2a14df5b1e32f50603215".parse::<DispnetHash>().unwrap();
        assert_eq!(dispnet_hash_1, dispnet_hash_2);
    }

    #[test]
    fn compare_crc32_hash_instance_and_prase() {
        let dispnet_hash_1 = DispnetHash::create(HashType::CRC, "test".as_bytes(), None);
        let dispnet_hash_2 = "0201032323538363632303830".parse::<DispnetHash>().unwrap();
        assert_eq!(dispnet_hash_1, dispnet_hash_2);
    }

    #[test]
    fn compare_argon2_hash_instance_and_prase() {
        let dispnet_hash_1 = DispnetHash::create(HashType::Argon2, "test".as_bytes(), None);
        let dispnet_hash_2 = "03121246172676f6e326924763d3139246d3d343039362c743d332c703d31245154687556586f785547746a4d456c614d48564b5531704f626b3173646d524d656a42554d3246734e5568716147637924464d4f7a6f46647754464676397a31435a485751684b7a2f63696f754c55427571494a54756a574d375338".parse::<DispnetHash>().unwrap();
        assert_eq!(dispnet_hash_1, dispnet_hash_2);
    }

    #[test]
    fn compare_argon2_salt_hash_instance_and_prase() {
        let dispnet_hash_1 = DispnetHash::create(HashType::Argon2, "test".as_bytes(), Some(HashConfig { salt: Some(Box::new(b"12345678".to_vec())) }));
        let dispnet_hash_2 = "03084246172676f6e326924763d3139246d3d343039362c743d332c703d31244d54497a4e4455324e7a6724686f56354d494638596a39746b39356c467365546279554a6e393336484944586754685533637065643151".parse::<DispnetHash>().unwrap();
        assert_eq!(dispnet_hash_1, dispnet_hash_2);
    }

    #[test]
    fn compare_hash_instance_and_string() {
        let dispnet_hash_1 = DispnetHash::new("test".as_bytes());
        assert_eq!(dispnet_hash_1, "010324878ca0425c739fa427f7eda20fe845f6b2e46ba5fe2a14df5b1e32f50603215".to_owned());
    }

    #[test]
    fn compare_crc32_hash_instance_and_string() {
        let dispnet_hash_1 = DispnetHash::create(HashType::CRC, "test".as_bytes(), None);
        assert_eq!(dispnet_hash_1, "0201032323538363632303830".to_owned());
    }
}
