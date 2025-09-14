use fluent::{FluentBundle, FluentResource, FluentValue};
use std::collections::HashMap;
use unic_langid::{langid, LanguageIdentifier};

/// Supported languages
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Language {
    Italian,
    English,
}

impl Language {
    pub fn language_id(&self) -> LanguageIdentifier {
        match self {
            Language::Italian => langid!("it"),
            Language::English => langid!("en"),
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            Language::Italian => "it",
            Language::English => "en",
        }
    }
}

/// Internationalization manager
pub struct I18nManager {
    bundles: HashMap<Language, FluentBundle<FluentResource>>,
    current_language: Language,
}

impl I18nManager {
    pub fn new() -> Self {
        let mut manager = Self {
            bundles: HashMap::new(),
            current_language: Language::Italian,
        };
        manager.load_languages();
        manager
    }

    fn load_languages(&mut self) {
        // Load Italian strings
        let it_ftl = include_str!("../locales/it.ftl");
        let it_resource =
            FluentResource::try_new(it_ftl.to_string()).expect("Failed to parse Italian locale");
        let mut it_bundle = FluentBundle::new(vec![langid!("it")]);
        it_bundle
            .add_resource(it_resource)
            .expect("Failed to add Italian resource");
        self.bundles.insert(Language::Italian, it_bundle);

        // Load English strings
        let en_ftl = include_str!("../locales/en.ftl");
        let en_resource =
            FluentResource::try_new(en_ftl.to_string()).expect("Failed to parse English locale");
        let mut en_bundle = FluentBundle::new(vec![langid!("en")]);
        en_bundle
            .add_resource(en_resource)
            .expect("Failed to add English resource");
        self.bundles.insert(Language::English, en_bundle);
    }

    pub fn set_language(&mut self, language: Language) {
        self.current_language = language;
    }

    pub fn current_language(&self) -> &Language {
        &self.current_language
    }

    /// Get localized string by message ID
    pub fn get(&self, message_id: &str) -> String {
        self.get_with_args(message_id, None)
    }

    /// Get localized string with arguments
    pub fn get_with_args(
        &self,
        message_id: &str,
        args: Option<&HashMap<String, String>>,
    ) -> String {
        if let Some(bundle) = self.bundles.get(&self.current_language) {
            if let Some(message) = bundle.get_message(message_id) {
                if let Some(pattern) = message.value() {
                    let mut errors = vec![];
                    let fluent_args = args.map(|a| {
                        a.iter()
                            .map(|(k, v)| (k.as_str(), FluentValue::from(v.as_str())))
                            .collect()
                    });
                    let formatted =
                        bundle.format_pattern(pattern, fluent_args.as_ref(), &mut errors);
                    return formatted.to_string();
                }
            }
        }

        // Fallback to message ID if translation not found
        format!("[{}]", message_id)
    }
}

impl Default for I18nManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_switching() {
        let mut i18n = I18nManager::new();

        assert_eq!(i18n.current_language(), &Language::Italian);

        i18n.set_language(Language::English);
        assert_eq!(i18n.current_language(), &Language::English);
    }

    #[test]
    fn test_basic_translation() {
        let i18n = I18nManager::new();

        // Test Italian (default)
        let app_name = i18n.get("app-name");
        assert_ne!(app_name, "[app-name]"); // Should not be fallback
    }

    #[test]
    fn test_fallback() {
        let i18n = I18nManager::new();

        // Test non-existent key
        let missing = i18n.get("non-existent-key");
        assert_eq!(missing, "[non-existent-key]");
    }
}
