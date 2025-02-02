# TOTP-RS

[English](README_EN.md) | [中文](README.md)

## 中文

基于 RFC 6238 标准的 Rust TOTP（基于时间的一次性密码）算法实现，支持多种哈希算法，包括 SHA1、SHA256、SHA512 和国密 SM3。

### 特性

- 支持多种哈希算法（SHA1、SHA256、SHA512、SM3）
- 可配置验证码长度（6 位或 8 位）
- 可自定义时间步长
- 支持时区偏移
- 支持国际化
- 符合 RFC 6238 标准

### 安装

在 `Cargo.toml` 中添加：

```toml
[dependencies]
totp-sm-rs = "0.1.4"
```

### 使用方法

符合 RFC 6238 标准中推荐密钥长度为 16 位的密钥调用示例

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

不符合 RFC 6238 标准中推荐密钥长度为 16 位的密钥调用示例，如 GitHub 的 TOTP

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

### 性能测试

以下是不同哈希算法生成 1000 个验证码的性能对比：

```
性能测试结果 (越短越好)
SHA1    481ns  ▌
SHA256  970ns  █
SHA512  1119ns █▏
SM3     914ns  ▉
```

> 注：测试环境为 Apple M1 Pro，测试数据仅供参考。

### 许可证

本项目采用双重许可证 - 你可以选择使用 [Apache License 2.0](LICENSE-APACHE) 或 [MIT License](LICENSE-MIT) 中的任意一个。
