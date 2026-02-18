#[macro_export]
macro_rules! i18n {
	($input:expr) => {
		{
			use fluent_langneg::{negotiate_languages, NegotiationStrategy};
			use fluent_resmgr::resource_manager::ResourceManager;
			use std::{fs, io, vec};
			use std::path::PathBuf;
			use unic_langid::LanguageIdentifier;

			let res_path = PathBuf::from(std::env!("CARGO_MANIFEST_DIR"))
				.join("resources")
				.join("{locale}")
				.join("{res_id}");
			// ResourceManager should be static
			let mgr = ResourceManager::new(res_path.to_str().unwrap().to_string());


			// dynamic part per request
			let resolved_locales = negotiate_languages(
					&[LanguageIdentifier::from_bytes("nl-BE".as_bytes()).unwrap()], // This needs to be detemined once upon request of a page
					&crate::AVAILABLE_LOCALES, // could be static
					Some(&crate::DEFAULT_LOCALE), // could be determined during compile time
					NegotiationStrategy::Filtering,
				)
				.into_iter()
				.map(|s| s.to_owned())
				.collect(); // This needs to be detemined once upon request of a page

			let bundle = mgr
				.get_bundle(resolved_locales, crate::AVAILABLE_RESOURCES.clone())
				.expect("Could not get bundle");

			let msg = bundle.get_message($input)
				.expect("Message doesn't exist.");

			let mut errors = vec![];
			let pattern = msg.value()
				.expect("Message has no value.");

			let mut output = String::new();

			bundle.write_pattern(&mut output, &pattern, None, &mut errors)
				.expect("Could not write.");

			output
		}
	};
	($input:expr, $(($arg_key:expr, $arg_value:expr)),+) => {
		{
			use fluent::{FluentArgs};
			use fluent_langneg::{negotiate_languages, NegotiationStrategy};
			use fluent_resmgr::resource_manager::ResourceManager;
			use std::{fs, io, vec};
			use std::path::PathBuf;
			use unic_langid::LanguageIdentifier;

			let res_path = PathBuf::from(std::env!("CARGO_MANIFEST_DIR"))
				.join("resources")
				.join("{locale}")
				.join("{res_id}");
			// ResourceManager should be static
			let mgr = ResourceManager::new(res_path.to_str().unwrap().to_string());


			// dynamic part per request
			let resolved_locales = negotiate_languages(
					&[LanguageIdentifier::from_bytes("nl-BE".as_bytes()).unwrap()], // This needs to be detemined once upon request of a page
					&crate::AVAILABLE_LOCALES, // could be static
					Some(&crate::DEFAULT_LOCALE), // could be determined during compile time
					NegotiationStrategy::Filtering,
				)
				.into_iter()
				.map(|s| s.to_owned())
				.collect(); // This needs to be detemined once upon request of a page

			let bundle = mgr
				.get_bundle(resolved_locales, crate::AVAILABLE_RESOURCES.clone())
				.expect("Could not get bundle");

			let msg = bundle.get_message($input)
				.expect("Message doesn't exist.");

			let mut args = FluentArgs::new();
			$(args.set($arg_key, $arg_value);)+

			let mut errors = vec![];
			let pattern = msg.value()
				.expect("Message has no value.");

			let mut output = String::new();

			bundle.write_pattern(&mut output, &pattern, Some(&args), &mut errors)
				.expect("Could not write.");

			output
		}
	};
}

#[macro_export]
macro_rules! enable_i18n {
	($resource_location:literal, $default_locale:literal, $($resources:literal),*) => {
		lazy_static! {
			static ref AVAILABLE_LOCALES: Vec<LanguageIdentifier> = {
				let mut locales = vec![];

				let res_path = PathBuf::from(std::env!("CARGO_MANIFEST_DIR")).join($resource_location);
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
			};
			static ref DEFAULT_LOCALE: LanguageIdentifier = LanguageIdentifier::from_bytes($default_locale.as_bytes()).expect("Parsing failed.");
			static ref AVAILABLE_RESOURCES: Vec<String> = vec![$(String::from($resources)),*];
		}
	}
}