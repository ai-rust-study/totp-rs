use crate::utils::i18n::I18n;
use base32::Alphabet::Rfc4648;
use chrono::Utc;
use hmac::{Hmac, Mac};
use sha1::Sha1;
use sha2::{Sha256, Sha512};
use sm3::Sm3;
use std::fmt;

/// CN: HMAC-SHA1 类型别名
/// EN: Type alias for HMAC-SHA1
type HmacSha1 = Hmac<Sha1>;

/// CN: HMAC-SHA256 类型别名
/// EN: Type alias for HMAC-SHA256
type HmacSha256 = Hmac<Sha256>;

/// CN: HMAC-SHA512 类型别名
/// EN: Type alias for HMAC-SHA512
type HmacSha512 = Hmac<Sha512>;

/// CN: HMAC-SM3 类型别名
/// EN: Type alias for HMAC-SM3
type HmacSm3 = Hmac<Sm3>;

/// CN: 哈希算法枚举，支持多种哈希算法实现
/// EN: Hash algorithm enumeration, supporting multiple hash algorithm implementations
#[derive(Debug, Clone, Copy)]
pub enum HashAlgorithm {
    /// CN: SHA1 哈希算法（RFC 4226标准）
    /// EN: SHA1 hash algorithm (RFC 4226 standard)
    SHA1,
    /// CN: SHA256 哈希算法（增强安全性）
    /// EN: SHA256 hash algorithm (enhanced security)
    SHA256,
    /// CN: SHA512 哈希算法（最高安全性）
    /// EN: SHA512 hash algorithm (maximum security)
    SHA512,
    /// CN: 国密SM3哈希算法
    /// EN: Chinese SM3 hash algorithm
    SM3,
}

impl fmt::Display for HashAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HashAlgorithm::SHA1 => write!(f, "SHA1"),
            HashAlgorithm::SHA256 => write!(f, "SHA256"),
            HashAlgorithm::SHA512 => write!(f, "SHA512"),
            HashAlgorithm::SM3 => write!(f, "SM3"),
        }
    }
}

/// CN: 为HashAlgorithm实现默认值特征，默认使用SHA1算法
/// EN: Implement Default trait for HashAlgorithm, using SHA1 as default
impl Default for HashAlgorithm {
    fn default() -> Self {
        Self::SHA1
    }
}

/// CN: TOTP配置结构体，用于自定义TOTP生成参数
/// EN: TOTP configuration struct for customizing TOTP generation parameters
#[derive(Debug, Copy, Clone)]
pub struct TotpConfig {
    /// CN: 验证码位数（4至10位）
    /// EN: Number of digits in the verification code （4 to 10 digits）
    pub digits: u8,
    /// CN: 时间步长（秒）
    /// EN: Time step in seconds
    pub time_step: u64,
    /// CN: 可选的自定义时间戳
    /// EN: Optional custom timestamp
    pub timestamp: Option<u64>,
    /// CN: 可选的时区偏移（秒）
    /// EN: Optional timezone offset in seconds
    pub timezone_offset: Option<i32>,
    /// CN: 使用的哈希算法
    /// EN: Hash algorithm to use
    pub hash_algorithm: HashAlgorithm,
}

/// CN: 为TotpConfig实现默认值特征
/// EN: Implement Default trait for TotpConfig
impl Default for TotpConfig {
    fn default() -> Self {
        Self {
            digits: 6,
            time_step: 30,
            timestamp: None,
            timezone_offset: None,
            hash_algorithm: HashAlgorithm::default(),
        }
    }
}

/// CN: TOTP错误类型枚举
/// EN: TOTP error type enumeration
#[derive(Debug)]
pub enum TotpError {
    /// CN: Base32解码错误
    /// EN: Base32 decoding error
    Base32DecodeError,
    /// CN: 密钥长度无效
    /// EN: Invalid key length
    InvalidKeyLength,
}

/// CN: 为TotpError实现显示特征，提供错误信息的国际化支持
/// EN: Implement Display trait for TotpError with internationalization support
impl fmt::Display for TotpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let i18n = I18n::new();
        match self {
            TotpError::Base32DecodeError => write!(
                f,
                "{}",
                i18n.get_message("totp.errors.base32_decode").unwrap()
            ),
            TotpError::InvalidKeyLength => write!(
                f,
                "{}",
                i18n.get_message("totp.errors.invalid_key_length").unwrap()
            ),
        }
    }
}

/// CN: 生成TOTP验证码
/// CN: 参数:
/// CN: - secret: Base32编码的密钥
/// CN: - config: TOTP配置选项，包含验证码位数、时间步长等参数
/// CN: 返回:
/// CN: - Ok(String): 成功生成的TOTP验证码
/// CN: - Err(TotpError): 生成过程中的错误
///
/// EN: Generate TOTP verification code
/// EN: Parameters:
/// EN: - secret: Base32 encoded secret key
/// EN: - config: TOTP configuration options, including code digits, time step, etc.
/// EN: Returns:
/// EN: - Ok(String): Successfully generated TOTP code
/// EN: - Err(TotpError): Errors during generation
pub fn generate_totp_code(secret: &str, config: Option<TotpConfig>) -> Result<String, TotpError> {
    let config = config.unwrap_or_default();
    if config.digits < 4 && config.digits > 10 {
        return Err(TotpError::InvalidKeyLength);
    }
    // CN: Base32密钥解码和有效性验证
    // EN: Decode Base32 secret and validate
    let secret_bytes = match base32::decode(Rfc4648 { padding: false }, secret) {
        Some(bytes) if !bytes.is_empty() => bytes,
        _ => return Err(TotpError::Base32DecodeError),
    };

    // CN: 获取时间戳并应用时区偏移
    // EN: Get timestamp and apply timezone offset
    let mut timestamp = match config.timestamp {
        Some(ts) => ts,
        None => Utc::now().timestamp() as u64,
    };
    if let Some(offset) = config.timezone_offset {
        timestamp = timestamp.saturating_add_signed(offset as i64);
    }
    let time = timestamp / config.time_step;

    // CN: 将时间戳转换为大端字节数组
    // EN: Convert timestamp to big-endian byte array
    let time_bytes = time.to_be_bytes();

    // CN: 根据选择的算法创建HMAC实例并计算
    // EN: Create and calculate HMAC instance based on selected algorithm
    let result = match config.hash_algorithm {
        HashAlgorithm::SHA1 => {
            let mut mac =
                HmacSha1::new_from_slice(&secret_bytes).map_err(|_| TotpError::InvalidKeyLength)?;
            mac.update(&time_bytes);
            mac.finalize().into_bytes().to_vec()
        }
        HashAlgorithm::SHA256 => {
            let mut mac = HmacSha256::new_from_slice(&secret_bytes)
                .map_err(|_| TotpError::InvalidKeyLength)?;
            mac.update(&time_bytes);
            mac.finalize().into_bytes().to_vec()
        }
        HashAlgorithm::SHA512 => {
            let mut mac = HmacSha512::new_from_slice(&secret_bytes)
                .map_err(|_| TotpError::InvalidKeyLength)?;
            mac.update(&time_bytes);
            mac.finalize().into_bytes().to_vec()
        }
        HashAlgorithm::SM3 => {
            let mut mac =
                HmacSm3::new_from_slice(&secret_bytes).map_err(|_| TotpError::InvalidKeyLength)?;
            mac.update(&time_bytes);
            mac.finalize().into_bytes().to_vec()
        }
    };

    // CN: 动态截取偏移量（根据RFC 6238标准）
    // EN: Dynamic truncation offset (according to RFC 6238)
    let offset = (result[result.len() - 1] & 0xf) as usize;

    // CN: 验证密钥长度（至少16字节，符合安全要求）
    // EN: Validate key length (minimum 16 bytes for security)
    if secret_bytes.len() < 16 {
        return Err(TotpError::InvalidKeyLength);
    }

    // CN: 根据RFC 6238标准计算最终的验证码
    // EN: Calculate final code according to RFC 6238
    let code = ((result[offset] as u64 & 0x7f) << 24
        | (result[offset + 1] as u64 & 0xff) << 16
        | (result[offset + 2] as u64 & 0xff) << 8
        | (result[offset + 3] as u64 & 0xff))
        % 10u64.pow(config.digits as u32);

    Ok(format!(
        "{:0digits$}",
        code,
        digits = config.digits as usize
    ))
}
