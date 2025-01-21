# Changelog

## [0.1.0] - 2024-01-21

### Added
- Implemented TOTP generation based on RFC 6238 standard
- Support for SHA-1, SHA-256, SHA-512, and SM3 hash algorithms
- Support for 6-digit and 8-digit verification codes
- Support for custom time step and timezone offset

### Features
- Complete RFC 6238 test vector validation
- Base32 key encoding validation
- Key length validation
- Time window validation

### Technical Details
- Implemented using Rust standard library
- Support for custom configuration parameters
- Detailed API documentation
- Complete unit test coverage