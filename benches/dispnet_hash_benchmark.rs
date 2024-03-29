use criterion::{criterion_group, criterion_main, Criterion};
use dispnet_hash::{DispnetHash, HashType, HashConfig};


fn new_hash() {
    let dispnet_hash = DispnetHash::new("test".as_bytes());
    let display_hash = format!("{}", dispnet_hash);
    assert_eq!(display_hash, "0100324878ca0425c739fa427f7eda20fe845f6b2e46ba5fe2a14df5b1e32f50603215");
}

fn new_crc32_hash() {
    let dispnet_hash = DispnetHash::create(HashType::CRC, "test".as_bytes(), None);
    let display_hash = format!("{}", dispnet_hash);
    assert_eq!(display_hash, "02001032323538363632303830");
}

fn new_argon2_hash() {
    let dispnet_hash = DispnetHash::create(HashType::Argon2, "test".as_bytes(), None);
    let display_hash = format!("{}", dispnet_hash);
    assert_eq!(display_hash, "030121246172676f6e326924763d3139246d3d343039362c743d332c703d31245154687556586f785547746a4d456c614d48564b5531704f626b3173646d524d656a42554d3246734e5568716147637924464d4f7a6f46647754464676397a31435a485751684b7a2f63696f754c55427571494a54756a574d375338");
}

fn new_argon2_salt_hash() {
    let dispnet_hash = DispnetHash::create(HashType::Argon2, "test".as_bytes(), Some(HashConfig { salt: Some(Box::new(b"12345678".to_vec())) }));
    let display_hash = format!("{}", dispnet_hash);
    assert_eq!(display_hash, "030084246172676f6e326924763d3139246d3d343039362c743d332c703d31244d54497a4e4455324e7a6724686f56354d494638596a39746b39356c467365546279554a6e393336484944586754685533637065643151");
}

fn parse_hash() {
    let dispnet_hash = "0100324878ca0425c739fa427f7eda20fe845f6b2e46ba5fe2a14df5b1e32f50603215".parse::<DispnetHash>().unwrap();
    assert_eq!(dispnet_hash.hash_type, HashType::Blake3);
    assert_eq!(dispnet_hash.digest_length, 32);
    assert_eq!(dispnet_hash.digest_value.len(), 32);
}

fn parse_crc32_hash() {
    let dispnet_hash = "02001032323538363632303830".parse::<DispnetHash>().unwrap();
    assert_eq!(dispnet_hash.hash_type, HashType::CRC);
    assert_eq!(dispnet_hash.digest_length, 10);
    assert_eq!(dispnet_hash.digest_value.len(), 10);
}

fn parse_argon2_hash() {
    let dispnet_hash = "030121246172676f6e326924763d3139246d3d343039362c743d332c703d31245154687556586f785547746a4d456c614d48564b5531704f626b3173646d524d656a42554d3246734e5568716147637924464d4f7a6f46647754464676397a31435a485751684b7a2f63696f754c55427571494a54756a574d375338".parse::<DispnetHash>().unwrap();
    assert_eq!(dispnet_hash.hash_type, HashType::Argon2);
    assert_eq!(dispnet_hash.digest_length, 121);
    assert_eq!(dispnet_hash.digest_value.len(), 121);
}

fn parse_argon2_salt_hash() {
    let dispnet_hash = "030084246172676f6e326924763d3139246d3d343039362c743d332c703d31244d54497a4e4455324e7a6724686f56354d494638596a39746b39356c467365546279554a6e393336484944586754685533637065643151".parse::<DispnetHash>().unwrap();
    assert_eq!(dispnet_hash.hash_type, HashType::Argon2);
    assert_eq!(dispnet_hash.digest_length, 84);
    assert_eq!(dispnet_hash.digest_value.len(), 84);
}

fn verify_argon2_hash() {
    assert!(DispnetHash::verify("030084246172676f6e326924763d3139246d3d343039362c743d332c703d31244d54497a4e4455324e7a6724686f56354d494638596a39746b39356c467365546279554a6e393336484944586754685533637065643151", "test".as_bytes()));
}

fn hex_from_to_bytes() {
    assert_eq!(DispnetHash::bytes_to_hex("test".as_bytes()), "74657374");
    assert_eq!(DispnetHash::hex_to_bytes("74657374").unwrap(), "test".as_bytes());
}

fn encoded_u64() {
    assert_eq!(DispnetHash::encoded_u64("test".as_bytes()), 1953719668);
    assert_eq!(DispnetHash::encoded_u64("a".as_bytes()), 97);
    assert_eq!(DispnetHash::encoded_u64("aasdsakdljaslfhaksjhuahwiuewasdfgs4354sg".as_bytes()), 7454359211325289319);
}

fn compare_hash_instances(instance_1: &DispnetHash, instance_2: &DispnetHash) {
    assert_eq!(instance_1, instance_2);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("new hash", |b| b.iter(new_hash));
    c.bench_function("parse hash", |b| b.iter(parse_hash));

    c.bench_function("new CRC32 hash", |b| b.iter(new_crc32_hash));
    c.bench_function("parse CRC32 hash", |b| b.iter(parse_crc32_hash));

    c.bench_function("new Argon2 hash", |b| b.iter(new_argon2_hash));
    c.bench_function("parse Argon2 hash", |b| b.iter(parse_argon2_hash));

    c.bench_function("new Argon2 salt hash", |b| b.iter(new_argon2_salt_hash));
    c.bench_function("parse Argon2 salt hash", |b| b.iter(parse_argon2_salt_hash));

    let dispnet_hash_1 = DispnetHash::new("test".as_bytes());
    let dispnet_hash_2 = DispnetHash::new("test".as_bytes());
    c.bench_function("compare hash instances", |b| b.iter(|| compare_hash_instances(&dispnet_hash_1, &dispnet_hash_2)));

    let dispnet_crc32_hash_1 = DispnetHash::create(HashType::CRC, "test".as_bytes(), None);
    let dispnet_crc32_hash_2 = DispnetHash::create(HashType::CRC, "test".as_bytes(), None);
    c.bench_function("compare CRC32 hash instances", |b| b.iter(|| compare_hash_instances(&dispnet_crc32_hash_1, &dispnet_crc32_hash_2)));

    c.bench_function("verify Argon2 hash", |b| b.iter(verify_argon2_hash));

    c.bench_function("Hex convert from/to byte", |b| b.iter(hex_from_to_bytes));

    c.bench_function("encoded u64", |b| b.iter(encoded_u64));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);