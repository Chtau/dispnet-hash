# Dispnet Hash

Dispnet Hash produces a self descripting hash for easy backwards compatibility.
The hash format follows the **TLV** (type-length-value) pattern.

* First 2 characters are the hash type as integer with a leading 0 (Default is 01 which is **Blake3** hash).
* Then come 3 characters as integer with leading 0 which is the length of the bytes from the digest.
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
01     032            4878ca0425c739fa427f7eda20fe845f6b2e46ba5fe2a14df5b1e32f50603215
02     010            32323538363632303830
Type | Bytes length | Hash
```

## Usage

### Create a hash

```rust
let dispnet_hash = DispnetHash::new("test".as_bytes());
let display_hash = format!("{}", dispnet_hash);
assert_eq!(display_hash, "010324878ca0425c739fa427f7eda20fe845f6b2e46ba5fe2a14df5b1e32f50603215");
```

### Create a hash with hash type

```rust
let dispnet_hash = DispnetHash::create(HashType::CRC, "test".as_bytes(), None);
let display_hash = format!("{}", dispnet_hash);
assert_eq!(display_hash, "0201032323538363632303830");
```

### Get hash parts

```rust
let dispnet_hash = "010324878ca0425c739fa427f7eda20fe845f6b2e46ba5fe2a14df5b1e32f50603215".parse::<DispnetHash>().unwrap();
assert_eq!(dispnet_hash.hash_type, HashType::Blake3);
assert_eq!(dispnet_hash.digest_length, 32);
assert_eq!(dispnet_hash.digest_value.len(), 32);
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

---

## Made by Christoph Taucher

License [MIT](LICENSE)
