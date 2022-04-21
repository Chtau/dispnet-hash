use criterion::{criterion_group, criterion_main, Criterion};
use dispnet_hash::hash::{DispnetHash, HashType};


fn new_hash() {
    let dispnet_hash = DispnetHash::new("test".as_bytes());
    let display_hash = format!("{}", dispnet_hash);
    assert_eq!(display_hash, "010324878ca0425c739fa427f7eda20fe845f6b2e46ba5fe2a14df5b1e32f50603215");
}


fn new_crc32_hash() {
    let dispnet_hash = DispnetHash::create(HashType::CRC, "test".as_bytes());
    let display_hash = format!("{}", dispnet_hash);
    assert_eq!(display_hash, "0201032323538363632303830");
}

fn parse_hash() {
    let dispnet_hash = "010324878ca0425c739fa427f7eda20fe845f6b2e46ba5fe2a14df5b1e32f50603215".parse::<DispnetHash>().unwrap();
    assert_eq!(dispnet_hash.hash_type, HashType::Blake3);
    assert_eq!(dispnet_hash.digest_length, 32);
    assert_eq!(dispnet_hash.digest_value.len(), 32);
}

fn parse_crc32_hash() {
    let dispnet_hash = "0201032323538363632303830".parse::<DispnetHash>().unwrap();
    assert_eq!(dispnet_hash.hash_type, HashType::CRC);
    assert_eq!(dispnet_hash.digest_length, 10);
    assert_eq!(dispnet_hash.digest_value.len(), 10);
}

fn compare_hash_instances(instance_1: &DispnetHash, instance_2: &DispnetHash) {
    assert_eq!(instance_1, instance_2);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("new hash", |b| b.iter(|| new_hash()));
    c.bench_function("parse hash", |b| b.iter(|| parse_hash()));

    c.bench_function("new CRC32 hash", |b| b.iter(|| new_crc32_hash()));
    c.bench_function("parse CRC32 hash", |b| b.iter(|| parse_crc32_hash()));

    let dispnet_hash_1 = DispnetHash::new("test".as_bytes());
    let dispnet_hash_2 = DispnetHash::new("test".as_bytes());
    c.bench_function("compare hash instances", |b| b.iter(|| compare_hash_instances(&dispnet_hash_1, &dispnet_hash_2)));

    let dispnet_crc32_hash_1 = DispnetHash::create(HashType::CRC, "test".as_bytes());
    let dispnet_crc32_hash_2 = DispnetHash::create(HashType::CRC, "test".as_bytes());
    c.bench_function("compare CRC32 hash instances", |b| b.iter(|| compare_hash_instances(&dispnet_crc32_hash_1, &dispnet_crc32_hash_2)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);