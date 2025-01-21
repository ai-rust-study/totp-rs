use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct I18nConfig {
    totp: TotpMessages,
}

#[derive(Debug, Deserialize)]
struct TotpMessages {
    validation_failed: String,
    errors: TotpErrors,
}

#[derive(Debug, Deserialize)]
struct TotpErrors {
    base32_decode: String,
    invalid_key_length: String,
    invalid_digits: String,
    invalid_time_step: String,
    invalid_timestamp: String,
}

pub struct I18n {
    messages: HashMap<String, I18nConfig>,
    current_locale: String,
}

impl I18n {
    pub fn new() -> Self {
        let mut i18n = I18n {
            messages: HashMap::new(),
            current_locale: String::from("en"),
        };
        i18n.load_default_locales();
        i18n.set_locale_from_env();
        i18n
    }

    fn load_default_locales(&mut self) {
        let locales_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/utils/i18n/locales");
        for entry in fs::read_dir(locales_dir).expect("Failed to read locales directory") {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "yml") {
                    let locale = path.file_stem().unwrap().to_str().unwrap();
                    self.load_locale(locale, &path);
                }
            }
        }
    }

    fn load_locale(&mut self, locale: &str, path: &Path) {
        if let Ok(content) = fs::read_to_string(path) {
            if let Ok(config) = serde_yaml::from_str(&content) {
                self.messages.insert(locale.to_string(), config);
            }
        }
    }

    fn set_locale_from_env(&mut self) {
        if let Ok(lang) = env::var("LANG") {
            let locale = if lang.starts_with("zh") { "zh" } else { "en" };
            self.set_locale(locale);
        }
    }

    pub fn set_locale(&mut self, locale: &str) {
        if self.messages.contains_key(locale) {
            self.current_locale = locale.to_string();
        }
    }

    pub fn get_message(&self, key: &str) -> Option<&String> {
        let config = self.messages.get(&self.current_locale)?;
        match key {
            "totp.validation_failed" => Some(&config.totp.validation_failed),
            "totp.errors.base32_decode" => Some(&config.totp.errors.base32_decode),
            "totp.errors.invalid_key_length" => Some(&config.totp.errors.invalid_key_length),
            "totp.errors.invalid_digits" => Some(&config.totp.errors.invalid_digits),
            "totp.errors.invalid_time_step" => Some(&config.totp.errors.invalid_time_step),
            "totp.errors.invalid_timestamp" => Some(&config.totp.errors.invalid_timestamp),
            _ => None,
        }
    }

    pub fn load_custom_locale(&mut self, locale: &str, path: &Path) {
        self.load_locale(locale, path);
    }
}
