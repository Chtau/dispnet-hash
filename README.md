# Dispnet Hash

Dispnet Hash produces a self describing hash for easy backwards compatibility.
The hash format follows the **TLV** (type-length-value) pattern.

* First 2 characters are the hash type as integer with a leading 0 (Default is 01 which is **Blake3** hash).
* Then come 4 characters as integer with leading 0 which is the length of the bytes from the digest.
* Digest value as hex.

## Supported hash algorithm

* [Blake3](https://github.com/BLAKE3-team/BLAKE3)
* [CRC32](https://github.com/mrhooray/crc-rs)
* [Argon2](https://github.com/sru-systems/rust-argon2)

## Abstract hash structur

```xml
<type><bytes-length><value>
```

## Hash value structur

```bash
01     0032           4878ca0425c739fa427f7eda20fe845f6b2e46ba5fe2a14df5b1e32f50603215
02     0010           32323538363632303830
03     0084           246172676f6e326924763d3139246d3d343039362c743d332c703d31244d54497a4e4455324e7a6724686f56354d494638596a39746b39356c467365546279554a6e393336484944586754685533637065643151
Type | Bytes length | Hash
```

## Usage

### Create a hash

```rust
let dispnet_hash = DispnetHash::new("test".as_bytes());
let display_hash = format!("{}", dispnet_hash);
assert_eq!(display_hash, "0100324878ca0425c739fa427f7eda20fe845f6b2e46ba5fe2a14df5b1e32f50603215");
```

### Create a hash with hash type

```rust
let dispnet_hash = DispnetHash::create(HashType::CRC, "test".as_bytes(), None);
let display_hash = format!("{}", dispnet_hash);
assert_eq!(display_hash, "02001032323538363632303830");
```

### Get hash parts

```rust
let dispnet_hash = "0100324878ca0425c739fa427f7eda20fe845f6b2e46ba5fe2a14df5b1e32f50603215".parse::<DispnetHash>().unwrap();
assert_eq!(dispnet_hash.hash_type, HashType::Blake3);
assert_eq!(dispnet_hash.digest_length, 32);
assert_eq!(dispnet_hash.digest_value.len(), 32);
```

### Verify Argon2 hash with value

```rust
assert!(DispnetHash::verify("030084246172676f6e326924763d3139246d3d343039362c743d332c703d31244d54497a4e4455324e7a6724686f56354d494638596a39746b39356c467365546279554a6e393336484944586754685533637065643151", "test".as_bytes()));
```

## Build instruction

### 1.) Install [Rust](https://www.rust-lang.org/tools/install)

### 2.) Build

```sh
cargo build
```

### .) Test

```sh
cargo test
```

### .) Benchmark

```sh
cargo bench
```

View Html report in target/criterion/report/index.html

Created with [Criterion.rs](https://bheisler.github.io/criterion.rs/book/criterion_rs.html)

### .) Publish

```sh
cargo publish --dry-run
```

---

## Made by Christoph Taucher

License [MIT](LICENSE)
