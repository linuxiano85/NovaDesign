use fluent_bundle::{FluentBundle, FluentResource};
use std::collections::HashMap;
use unic_langid::LanguageIdentifier;

pub mod locales;

pub use locales::*;

/// Supported languages
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    Italian,
    English,
}

impl Language {
    pub fn code(&self) -> &'static str {
        match self {
            Language::Italian => "it",
            Language::English => "en",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Language::Italian => "Italiano",
            Language::English => "English",
        }
    }

    pub fn from_code(code: &str) -> Option<Self> {
        match code {
            "it" => Some(Language::Italian),
            "en" => Some(Language::English),
            _ => None,
        }
    }
}

/// Localization manager using Fluent
pub struct Localizer {
    bundles: HashMap<Language, FluentBundle<FluentResource>>,
    current_language: Language,
}

impl Localizer {
    pub fn new() -> Self {
        let mut localizer = Self {
            bundles: HashMap::new(),
            current_language: Language::English,
        };

        // Load default translations
        localizer.load_language(Language::English, ENGLISH_TRANSLATIONS);
        localizer.load_language(Language::Italian, ITALIAN_TRANSLATIONS);

        localizer
    }

    pub fn load_language(&mut self, language: Language, translations: &str) {
        let langid: LanguageIdentifier = language.code().parse().expect("Invalid language ID");
        let mut bundle = FluentBundle::new(vec![langid]);

        if let Ok(resource) = FluentResource::try_new(translations.to_string()) {
            if let Err(errors) = bundle.add_resource(resource) {
                eprintln!("Failed to add resource for {:?}: {:?}", language, errors);
            }
        }

        self.bundles.insert(language, bundle);
    }

    pub fn set_language(&mut self, language: Language) {
        self.current_language = language;
    }

    pub fn current_language(&self) -> Language {
        self.current_language
    }

    pub fn get_message(&self, message_id: &str) -> String {
        self.get_message_with_args(message_id, None)
    }

    pub fn get_message_with_args(
        &self,
        message_id: &str,
        args: Option<&fluent_bundle::FluentArgs>,
    ) -> String {
        if let Some(bundle) = self.bundles.get(&self.current_language) {
            if let Some(message) = bundle.get_message(message_id) {
                if let Some(pattern) = message.value() {
                    let mut errors = vec![];
                    let formatted = bundle.format_pattern(pattern, args, &mut errors);
                    if !errors.is_empty() {
                        eprintln!("Formatting errors for {}: {:?}", message_id, errors);
                    }
                    return formatted.to_string();
                }
            }
        }

        // Fallback to message ID if not found
        format!("[{}]", message_id)
    }

    /// Get all available languages
    pub fn available_languages(&self) -> Vec<Language> {
        self.bundles.keys().cloned().collect()
    }
}

impl Default for Localizer {
    fn default() -> Self {
        Self::new()
    }
}

/// Localization error types
#[derive(Debug, thiserror::Error)]
pub enum LocalizationError {
    #[error("Language not supported: {language}")]
    LanguageNotSupported { language: String },
    #[error("Message not found: {message_id}")]
    MessageNotFound { message_id: String },
    #[error("Translation parsing error: {error}")]
    ParsingError { error: String },
}

pub type Result<T> = std::result::Result<T, LocalizationError>;
