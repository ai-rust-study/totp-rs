use totp_sm_rs::utils::totp::totp::{generate_totp_code, HashAlgorithm, TotpConfig, TotpError};
const RFC_TEST_VECTORS: [(&str, i64, &str, HashAlgorithm); 24] = [
    // SHA-1 测试向量
    ("GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ", 59, "94287082", HashAlgorithm::SHA1),
    ("GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ", 1111111109, "07081804", HashAlgorithm::SHA1),
    ("GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ", 1111111111, "14050471", HashAlgorithm::SHA1),
    ("GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ", 1234567890, "89005924", HashAlgorithm::SHA1),
    ("GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ", 2000000000, "69279037", HashAlgorithm::SHA1),
    ("GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ", 20000000000, "65353130", HashAlgorithm::SHA1),

    // SHA-256 测试向量
    ("GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZA", 59, "46119246", HashAlgorithm::SHA256),
    ("GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZA", 1111111109, "68084774", HashAlgorithm::SHA256),
    ("GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZA", 1111111111, "67062674", HashAlgorithm::SHA256),
    ("GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZA", 1234567890, "91819424", HashAlgorithm::SHA256),
    ("GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZA", 2000000000, "90698825", HashAlgorithm::SHA256),
    ("GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZA", 20000000000, "77737706", HashAlgorithm::SHA256),

    // SHA-512 测试向量
    ("GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZDGNA", 59, "90693936", HashAlgorithm::SHA512),
    ("GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZDGNA", 1111111109, "25091201", HashAlgorithm::SHA512),
    ("GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZDGNA", 1111111111, "99943326", HashAlgorithm::SHA512),
    ("GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZDGNA", 1234567890, "93441116", HashAlgorithm::SHA512),
    ("GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZDGNA", 2000000000, "38618901", HashAlgorithm::SHA512),
    ("GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZDGNA", 20000000000, "47863826", HashAlgorithm::SHA512),

    // SM3 测试向量
    ("GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZA", 59, "70252643", HashAlgorithm::SM3),
    ("GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZA", 1111111109, "03087309", HashAlgorithm::SM3),
    ("GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZA", 1111111111, "68947295", HashAlgorithm::SM3),
    ("GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZA", 1234567890, "39678463", HashAlgorithm::SM3),
    ("GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZA", 2000000000, "78318009", HashAlgorithm::SM3),
    ("GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQGEZA", 20000000000, "53139997", HashAlgorithm::SM3),
];

/// 测试生成TOTP代码是否符合RFC 6238标准
#[test]
fn test_rfc_test_vectors() {
    for (secret, timestamp, expected, hash_algorithm) in RFC_TEST_VECTORS {
        let config = TotpConfig {
            digits: 8,
            time_step: 30,
            timestamp: Some(timestamp),
            timezone_offset: None,
            hash_algorithm, // 使用字段简写语法
        };

        let result = generate_totp_code(secret, Some(config));

        assert_eq!(
            result.unwrap(),
            expected.to_string(),
            "TOTP validation failed for timestamp {}, expected {}, algorithm {}",
            timestamp,
            expected,
            hash_algorithm
        );
    }
}

/// 测试无效的Base32编码
#[test]
fn test_invalid_base32() {
    let invalid_secrets = [
        "12345",      // 无效字符
        "GEZDG=====", // 无效填充
        "",           // 空字符串
    ];

    for secret in invalid_secrets {
        let config = TotpConfig {
            digits: 8,
            time_step: 30,
            timestamp: None,
            timezone_offset: None,
            hash_algorithm: HashAlgorithm::SHA1,
        };
        let result = generate_totp_code(secret, Some(config));
        assert!(
            matches!(result, Err(TotpError::Base32DecodeError)),
            "Expected Base32 decode error for secret: {}",
            secret
        );
    }
}

/// 测试密钥长度验证
#[test]
fn test_key_length_validation() {
    let short_secret = "GEZDG"; // 过短
    let config = TotpConfig {
        digits: 8,
        time_step: 30,
        timestamp: None,
        timezone_offset: None,
        hash_algorithm: HashAlgorithm::SHA1,
    };
    let result = generate_totp_code(short_secret, Some(config));
    assert!(matches!(result, Err(TotpError::InvalidKeyLength)));
}

/// 测试代码长度始终为8位
#[test]
fn test_code_length() {
    let secret = "GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ";
    let config = TotpConfig {
        digits: 8,
        time_step: 30,
        timestamp: Some(1234567890),
        timezone_offset: None,
        hash_algorithm: HashAlgorithm::SHA1,
    };
    let code = generate_totp_code(secret, Some(config)).unwrap();
    assert_eq!(code.len(), 8);
}

/// 测试代码范围在0-99999999之间
#[test]
fn test_code_range() {
    let secret = "GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ";
    let config = TotpConfig {
        digits: 8,
        time_step: 30,
        timestamp: Some(1234567890),
        timezone_offset: None,
        hash_algorithm: HashAlgorithm::SHA1,
    };
    let code = generate_totp_code(secret, Some(config)).unwrap();
    let code_num: u32 = code.parse().unwrap();
    assert!(code_num < 100_000_000);
}

/// 测试时间窗口变化
#[test]
fn test_time_window_change() {
    let secret = "GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ";

    // 测试相同时间窗口
    let config1 = TotpConfig {
        digits: 8,
        time_step: 30,
        timestamp: Some(1234567890),
        timezone_offset: None,
        hash_algorithm: HashAlgorithm::SHA1,
    };
    let code1 = generate_totp_code(secret, Some(config1)).unwrap();

    let config2 = TotpConfig {
        digits: 8,
        time_step: 30,
        timestamp: Some(1234567895),
        timezone_offset: None,
        hash_algorithm: HashAlgorithm::SHA1,
    };
    let code2 = generate_totp_code(secret, Some(config2)).unwrap();
    assert_eq!(code1, code2);

    // 测试不同时间窗口
    let config3 = TotpConfig {
        digits: 8,
        time_step: 30,
        timestamp: Some(1234567920),
        timezone_offset: None,
        hash_algorithm: HashAlgorithm::SHA1,
    };
    let code3 = generate_totp_code(secret, Some(config3)).unwrap();
    assert_ne!(code1, code3);
}

/// 测试边界条件
#[test]
fn test_edge_cases() {
    let secret = "GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ";

    // 测试时间戳为0
    let config1 = TotpConfig {
        digits: 8,
        time_step: 30,
        timestamp: Some(0),
        timezone_offset: None,
        hash_algorithm: HashAlgorithm::SHA1,
    };
    let code = generate_totp_code(secret, Some(config1)).unwrap();
    assert_eq!(code.len(), 8);

    // 测试最大时间戳
    let config2 = TotpConfig {
        digits: 8,
        time_step: 30,
        timestamp: Some(i64::MAX),
        timezone_offset: None,
        hash_algorithm: HashAlgorithm::SHA1,
    };
    let code = generate_totp_code(secret, Some(config2)).unwrap();
    assert_eq!(code.len(), 8);
}

/// 测试不同位数验证码（4-10位）
#[test]
fn test_different_digits() {
    let secret = "GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ";
    let hash_algorithms = [
        HashAlgorithm::SHA1,
        HashAlgorithm::SHA256,
        HashAlgorithm::SHA512,
        HashAlgorithm::SM3,
    ];

    for digits in 4..=10 {
        for &algorithm in &hash_algorithms {
            let config = TotpConfig {
                digits,
                time_step: 30,
                timestamp: Some(1234567890),
                timezone_offset: None,
                hash_algorithm: algorithm,
            };
            let code = generate_totp_code(secret, Some(config)).unwrap();
            assert_eq!(
                code.len() as u8,
                digits,
                "验证码长度不符合预期，算法：{}",
                algorithm
            );

            let code_num: u64 = code.parse().unwrap();
            assert!(
                code_num < 10u64.pow(digits as u32),
                "验证码超出范围，算法：{}",
                algorithm
            );
        }
    }
}

/// 性能测试：测试不同哈希算法的性能表现
#[test]
fn test_performance() {
    use std::time::Instant;
    let secret = "GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ";
    let hash_algorithms = [
        HashAlgorithm::SHA1,
        HashAlgorithm::SHA256,
        HashAlgorithm::SHA512,
        HashAlgorithm::SM3,
    ];
    let iterations = 1000;

    for &algorithm in &hash_algorithms {
        let config = TotpConfig {
            digits: 6,
            time_step: 30,
            timestamp: Some(1234567890),
            timezone_offset: None,
            hash_algorithm: algorithm,
        };

        let start = Instant::now();
        for _ in 0..iterations {
            let _ = generate_totp_code(secret, Some(config));
        }
        let duration = start.elapsed();
        println!(
            "算法 {} 生成 {} 个验证码耗时：{:?}，平均每个耗时：{:?}",
            algorithm,
            iterations,
            duration,
            duration / iterations as u32
        );
    }
}

/// 测试默认参数
#[test]
fn test_default_parameters() {
    let secret = "GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ";

    // 测试默认digits (6)
    let config = TotpConfig {
        digits: 6,
        time_step: 30,
        timestamp: Some(1234567890),
        timezone_offset: None,
        hash_algorithm: HashAlgorithm::SHA1,
    };
    let code = generate_totp_code(secret, Some(config)).unwrap();
    assert_eq!(code.len(), 6);

    // 测试默认time_step (30)
    let config = TotpConfig {
        digits: 8,
        time_step: 30,
        timestamp: Some(1234567890),
        timezone_offset: None,
        hash_algorithm: HashAlgorithm::SHA1,
    };
    let code = generate_totp_code(secret, Some(config)).unwrap();
    assert_eq!(code.len(), 8);

    // 测试两个默认参数
    let code = generate_totp_code(secret, None).unwrap();
    assert_eq!(code.len(), 6);
}
