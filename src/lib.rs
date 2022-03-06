pub mod hash;

#[cfg(test)]
mod tests {
    use crate::hash::ForwardHash;

    #[test]
    fn new_hash() {
        let forward_hash = ForwardHash::new("test".as_bytes());
        let display_hash = format!("{}", forward_hash);
        assert_eq!(display_hash, "010324878ca0425c739fa427f7eda20fe845f6b2e46ba5fe2a14df5b1e32f50603215");
    }
}
