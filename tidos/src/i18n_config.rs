use std::fs;
use std::path::PathBuf;
use figment::{Error, Figment, Metadata, Profile, Provider};
use figment::providers::{Format, Toml};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TidosI18nConfig {
    pub resource_location: String,
    pub default_locale: String,
    pub resources: Vec<String>,
}

impl Default for TidosI18nConfig {
    fn default() -> Self {
        TidosI18nConfig {
            resource_location: "translations".to_string(),
            default_locale: "en-US".to_string(), // todo this should be a option
            resources: vec![],
        }
    }
}

impl TidosI18nConfig {
    // Allow the configuration to be extracted from any `Provider`.
    pub fn from<T: Provider>(provider: T) -> Result<TidosI18nConfig, Error> {
        Figment::from(provider).extract()
    }

    // Provide a default provider, a `Figment`.
    pub fn figment() -> Figment {
        use figment::providers::Env;

        Figment::from(TidosI18nConfig::default())
            .merge(Toml::file_exact("Tidos.toml").nested())
            .merge(Env::prefixed("TIDOS_I18N_"))
    }

    pub fn get_default_locale(&self) -> LanguageIdentifier {
        self.default_locale.parse().unwrap()
    }

    pub fn get_available_locales(&self) -> Vec<LanguageIdentifier> {
        let mut locales = vec![];

        let res_path = std::env::current_dir().unwrap().join(&self.resource_location);
        let res_dir = fs::read_dir(res_path).expect("Unable to open resources");
        for entry in res_dir.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if let Some(name) = path.file_name() {
                    if let Some(name) = name.to_str() {
                        let langid: LanguageIdentifier = name.parse().expect("Parsing failed.");
                        locales.push(langid);
                    }
                }
            }
        }
        locales
    }
}

use figment::value::{Map, Dict};
use unic_langid::LanguageIdentifier;

// Make `Config` a provider itself for composability.
impl Provider for TidosI18nConfig {
    fn metadata(&self) -> Metadata {
        Metadata::named("Tidos i18n Config")
    }

    fn data(&self) -> Result<Map<Profile, Dict>, Error>  {
        figment::providers::Serialized::defaults(TidosI18nConfig::default()).data()
    }

    fn profile(&self) -> Option<Profile> {
        Some(Profile::Default)
    }
}