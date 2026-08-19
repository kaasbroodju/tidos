use figment::providers::{Format, Toml};
use figment::{Error, Figment, Metadata, Profile, Provider};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs;
use std::path::PathBuf;

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

/// Errors raised while loading or interpreting a [`TidosI18nConfig`].
///
/// All variants carry the resolved absolute path or value involved, since
/// these errors are only ever surfaced via a `panic!` at startup or first
/// use (see [`crate::enable_i18n!`]) and a bare `unwrap()` message previously
/// gave no indication of *which* file or value was the problem.
#[derive(Debug)]
pub enum TidosI18nError {
	/// `Tidos.toml` could not be loaded/parsed by figment.
	Config { path: PathBuf, source: Box<Error> },
	/// The process's current working directory could not be determined.
	CurrentDir(std::io::Error),
	/// `default_locale` in `Tidos.toml` is not a valid language identifier.
	InvalidDefaultLocale {
		value: String,
		source: unic_langid::LanguageIdentifierError,
	},
	/// The `resource_location` directory does not exist or could not be read.
	ResourceDirUnreadable {
		path: PathBuf,
		source: std::io::Error,
	},
	/// A subdirectory of `resource_location` is not a valid language identifier.
	InvalidLocaleDirName {
		name: String,
		path: PathBuf,
		source: unic_langid::LanguageIdentifierError,
	},
}

impl fmt::Display for TidosI18nError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			TidosI18nError::Config { path, source } => write!(
				f,
				"Tidos i18n: could not load configuration from '{}': {source}\n\
				hint: this path is resolved relative to the process's current working directory, not the crate root -- \
				make sure the process is started with the crate directory as its working directory.",
				path.display()
			),
			TidosI18nError::CurrentDir(source) => {
				write!(f, "Tidos i18n: could not determine the current working directory: {source}")
			}
			TidosI18nError::InvalidDefaultLocale { value, source } => write!(
				f,
				"Tidos i18n: `default_locale = \"{value}\"` in Tidos.toml is not a valid language identifier: {source}"
			),
			TidosI18nError::ResourceDirUnreadable { path, source } => write!(
				f,
				"Tidos i18n: could not read resource directory '{}': {source}\n\
				hint: `resource_location` in Tidos.toml is resolved relative to the process's current working directory.",
				path.display()
			),
			TidosI18nError::InvalidLocaleDirName { name, path, source } => write!(
				f,
				"Tidos i18n: '{}' is not a valid language identifier for a locale directory ('{name}'): {source}",
				path.display()
			),
		}
	}
}

impl std::error::Error for TidosI18nError {}

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

	/// Loads the configuration via [`Self::figment`], wrapping a failure in a
	/// [`TidosI18nError`] that names the (CWD-relative) path that was tried.
	pub fn load() -> Result<TidosI18nConfig, TidosI18nError> {
		let path = std::env::current_dir()
			.map_err(TidosI18nError::CurrentDir)?
			.join("Tidos.toml");

		Self::figment()
			.extract()
			.map_err(|source| TidosI18nError::Config {
				path,
				source: Box::new(source),
			})
	}

	pub fn get_default_locale(&self) -> Result<LanguageIdentifier, TidosI18nError> {
		self.default_locale
			.parse()
			.map_err(|source| TidosI18nError::InvalidDefaultLocale {
				value: self.default_locale.clone(),
				source,
			})
	}

	pub fn get_available_locales(&self) -> Result<Vec<LanguageIdentifier>, TidosI18nError> {
		let mut locales = vec![];

		let res_path = std::env::current_dir()
			.map_err(TidosI18nError::CurrentDir)?
			.join(&self.resource_location);

		let res_dir =
			fs::read_dir(&res_path).map_err(|source| TidosI18nError::ResourceDirUnreadable {
				path: res_path.clone(),
				source,
			})?;

		for entry in res_dir.flatten() {
			let path = entry.path();
			if path.is_dir() {
				if let Some(name) = path.file_name() {
					if let Some(name) = name.to_str() {
						let langid: LanguageIdentifier = name.parse().map_err(|source| {
							TidosI18nError::InvalidLocaleDirName {
								name: name.to_string(),
								path: path.clone(),
								source,
							}
						})?;
						locales.push(langid);
					}
				}
			}
		}
		Ok(locales)
	}
}

use figment::value::{Dict, Map};
use unic_langid::LanguageIdentifier;

// Make `Config` a provider itself for composability.
impl Provider for TidosI18nConfig {
	fn metadata(&self) -> Metadata {
		Metadata::named("Tidos i18n Config")
	}

	fn data(&self) -> Result<Map<Profile, Dict>, Error> {
		figment::providers::Serialized::defaults(TidosI18nConfig::default()).data()
	}

	fn profile(&self) -> Option<Profile> {
		Some(Profile::Default)
	}
}
