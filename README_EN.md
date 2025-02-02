# TOTP-RS

[English](README_EN.md) | [中文](README.md)

A Rust implementation of Time-based One-Time Password (TOTP) algorithm according to RFC 6238, with support for multiple hash algorithms including SHA1, SHA256, SHA512, and Chinese SM3.

### Features

- Multiple hash algorithm support (SHA1, SHA256, SHA512, SM3)
- Configurable code length (6 or 8 digits)
- Customizable time step
- Timezone offset support
- Internationalization support
- RFC 6238 compliant

### Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
totp-sm-rs = "0.1.4"
```

### Usage

Example of invoking a key with the recommended key length of 16 bits as per RFC 6238 standard.

```rust
use totp_sm_rs::utils::totp::totp::{generate_totp_code, TotpConfig, HashAlgorithm};

// 使用默认配置生成TOTP（SHA1算法，6位验证码）
let secret = "GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ";
let code = generate_totp_code(secret, None).unwrap();

// 使用自定义配置生成TOTP
let config = TotpConfig {
    digits: 8,
    time_step: 30,
    timestamp: Some(1234567890),
    timezone_offset: None,
    hash_algorithm: HashAlgorithm::SHA1,
    is_check_security: true

};
let code = generate_totp_code(secret, Some(config)).unwrap();
```

An example of invoking a key that does not comply with the recommended key length of 16 bits as per RFC 6238 standard, such as GitHub's TOTP.

```rust
use totp_sm_rs::utils::totp::totp::{generate_totp_code, TotpConfig, HashAlgorithm};

// 使用默认配置生成TOTP（SHA1算法，6位验证码）
let secret = "GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ";
let code = generate_totp_code(secret, None).unwrap();

// 使用自定义配置生成TOTP
let config = TotpConfig {
    digits: 8,
    time_step: 30,
    timestamp: Some(1234567890),
    timezone_offset: None,
    hash_algorithm: HashAlgorithm::SHA1,
    is_check_security: false

};
let code = generate_totp_code(secret, Some(config)).unwrap();
```

### Performance

Here's a performance comparison of different hash algorithms generating 1000 TOTP codes:

```
Performance Results (shorter is better)
SHA1    481ns  ▌
SHA256  970ns  █
SHA512  1119ns █▏
SM3     914ns  ▉
```

> Note: Tested on Apple M1 Pro, results are for reference only.

### License

This project is dual-licensed - you can choose to use either [Apache License 2.0](LICENSE-APACHE) or [MIT License](LICENSE-MIT).
