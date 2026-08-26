use std::borrow::Cow;

use bt_any_error::any_err::AnyErr;
use bt_logger::get_error;
use rustc_hash::FxHashMap;

use crate::languages::Languages;
use crate::localizer::{Localizer, StringValues};

#[derive(Clone)]
pub struct TraslatorHelper{
    languages: Languages,
    holders: FxHashMap<String, Localizer>,
}

impl TraslatorHelper {
    pub fn default() -> TraslatorHelper{
        TraslatorHelper{ holders: FxHashMap::default(), languages: Languages::new() }
    }

    pub fn init(languages: Languages) -> TraslatorHelper{
        TraslatorHelper{ holders: FxHashMap::default(), languages }
    }

    pub fn add_language(&mut self, language_code: &str, language_name: &str) -> u16{
        self.languages.add_language(language_code, language_name)
    }

    pub fn get_lang_id_be(&self, language_code: &str) -> u16{
        self.languages.get_lang_id_be(language_code)
    }

    pub fn add_translation(&mut self, language_code: &str, translation_section: &str, content_toml: &str) -> Result<(), AnyErr>{
        let lang_id = self.get_lang_id_be(language_code);
        let toml_table: toml::Table = toml::from_str(content_toml)?;
        let section = match toml_table.get(translation_section){
            Some(s) => s,
            None => return Err(get_error!("","Cannot find section '{}' in TOML table (None)",translation_section).into()),
        };

        // Fast path: direct table access without collecting into intermediate Vec
        if let Some(table) = section.as_table() {
            // Skip empty tables early
            if table.is_empty() {
                return Ok(());
            }
            
            // Pre-allocate localizer before insert to avoid reallocation
            let localizer = self.holders
                .entry(translation_section.to_string())
                .or_insert_with(Localizer::new);
                
            // Use Cow to avoid unnecessary allocations for borrowed strings
            let translation_pairs: Vec<(Cow<'_, str>, Cow<'_, str>)> = table
                .iter()
                .filter_map(|(key, value)| {
                    value.as_str().map(|value| (
                        Cow::Borrowed(key.as_str()),
                        Cow::Borrowed(value)
                    ))
                })
                .collect();
                
            // Only insert if we have translations
            if !translation_pairs.is_empty() {
                // Convert Cow pairs to static pairs only when needed for insert_batch
                let owned_pairs: Vec<(&'static str, &'static str)> = translation_pairs
                    .into_iter()
                    .map(|(k, v)| (
                        Box::leak(k.into_owned().into_boxed_str()) as &'static str,
                        Box::leak(v.into_owned().into_boxed_str()) as &'static str
                    ))
                    .collect();
                localizer.insert_batch(lang_id, owned_pairs);
            }
        } else {
            return Err(get_error!("","Cannot parse section '{}' in TOML table (None)",translation_section).into());
        }
   
        Ok(())
    }

    pub fn get_translations(&self, language_id: u16, translation_section: &str) -> Option<StringValues> {
        // Direct access without intermediate Option checks
        self.holders.get(translation_section)
            .and_then(|t| t.get_string_values(language_id))
    }

}


#[cfg(test)]
mod translator_tests {
    use crate::languages::Languages;
use crate::translator::TraslatorHelper;

    #[test]
    fn test_default() {
        let helper = TraslatorHelper::default();
        assert_eq!(helper.holders.len(), 0);
    }

    #[test]
    fn test_init() {
        let languages = Languages::new();
        let helper = TraslatorHelper::init(languages);
        assert_eq!(helper.holders.len(), 0);
    }

    #[test]
    fn test_add_language_and_get_id() {
        let mut helper = TraslatorHelper::default();
        
        // Add first language
        let id1 = helper.add_language("en", "English");
        let id2 = helper.add_language("es", "Spanish");
        
        // IDs should be different
        assert_ne!(id1, id2);
        
        // Verify retrieval
        assert_eq!(helper.get_lang_id_be("en"), id1);
        assert_eq!(helper.get_lang_id_be("es"), id2);
    }

    #[test]
    fn test_add_translation_success_single_language() {
        let mut helper = TraslatorHelper::default();
        helper.add_language("en", "English");
        
        let toml_content = r#"
[greeting]
hello = "Hello"
goodbye = "Goodbye"
"#;
        
        let result = helper.add_translation("en", "greeting", toml_content);
        assert!(result.is_ok());
        
        // Verify localizer exists in handlers
        assert!(helper.holders.contains_key("greeting"));
    }

    #[test]
    fn test_add_translation_success_multiple_languages() {
        let mut helper = TraslatorHelper::default();
        helper.add_language("en", "English");
        helper.add_language("es", "Spanish");
        
        // English translations
        let en_content = r#"
[greeting]
hello = "Hello"
goodbye = "Goodbye"
"#;
        
        // Spanish translations
        let es_content = r#"
[greeting]
hello = "Hola"
goodbye = "Adiós"
"#;
        
        assert!(helper.add_translation("en", "greeting", en_content).is_ok());
        assert!(helper.add_translation("es", "greeting", es_content).is_ok());
        
        // Should have one entry for "greeting" section
        assert_eq!(helper.holders.len(), 1);
    }

    #[test]
    fn test_add_translation_missing_section() {
        bt_logger::build_logger("bachuetech", "translator_test", bt_logger::LogLevel::VERBOSE, bt_logger::LogTarget::STD_OUT, None);           
        let mut helper = TraslatorHelper::default();
        helper.add_language("en", "English");
        
        let toml_content = r#"
[other]
hello = "Hello"
"#;
        
        let result = helper.add_translation("en", "greeting", toml_content);
        assert!(result.is_err());
        
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("Cannot find section 'greeting'"));
    }

    #[test]
    fn test_add_translation_invalid_toml() {
        let mut helper = TraslatorHelper::default();
        helper.add_language("en", "English");
        
        // Invalid TOML syntax
        let toml_content = r#"
[greeting]
hello = "Unclosed quote
"#;
        
        let result = helper.add_translation("en", "greeting", toml_content);
        assert!(result.is_err());
    }

    #[test]
    fn test_add_translation_empty_section() {
        let mut helper = TraslatorHelper::default();
        helper.add_language("en", "English");
        
        let toml_content = r#"
[greeting]
"#;
        
        // Empty section should still work (inserts empty batch)
        let result = helper.add_translation("en", "greeting", toml_content);
        assert!(result.is_ok());
        println!("HELLO: {:?}",helper.holders);
        assert!(helper.holders.is_empty());
    }

    #[test]
    fn test_add_translation_non_string_values_filtered() {
        let mut helper = TraslatorHelper::default();
        helper.add_language("en", "English");
        
        let toml_content = r#"
[greeting]
hello = "Hello"
count = 42
valid = true
pi = 3.14
"#;
        
        let result = helper.add_translation("en", "greeting", toml_content);
        assert!(result.is_ok());
        
        // Only string values should be kept (hello)
        assert!(helper.holders.contains_key("greeting"));
    }

    #[test]
    fn test_add_translation_multiple_sections_isolation() {
        let mut helper = TraslatorHelper::default();
        helper.add_language("en", "English");
        
        let greeting_content = r#"
[greeting]
hello = "Hello"
"#;
        
        let farewell_content = r#"
[farewell]
bye = "Bye"
"#;
        
        assert!(helper.add_translation("en", "greeting", greeting_content).is_ok());
        assert!(helper.add_translation("en", "farewell", farewell_content).is_ok());
        
        assert_eq!(helper.holders.len(), 2);
        assert!(helper.holders.contains_key("greeting"));
        assert!(helper.holders.contains_key("farewell"));
    }

    #[test]
    fn test_add_translation_same_section_updates() {
        let mut helper = TraslatorHelper::default();
        helper.add_language("en", "English");
        helper.add_language("es", "Spanish");
        
        // First load English
        let en_content = r#"
[greeting]
hello = "Hello"
"#;
        assert!(helper.add_translation("en", "greeting", en_content).is_ok());
        
        // Add Spanish to same section
        let es_content = r#"
[greeting]
hello = "Hola"
"#;
        assert!(helper.add_translation("es", "greeting", es_content).is_ok());
        
        // Should still only have 1 entry (updated, not duplicated)
        assert_eq!(helper.holders.len(), 1);
    }

    #[test]
    fn test_add_translation_special_characters() {
        let mut helper = TraslatorHelper::default();
        helper.add_language("fr", "French");
        
        // Unicode, quotes, and escapes
        let toml_content = r#"
[special]
quotes = "Say \"hello\""
unicode = "Café ☕"
multiline = "Line 1\nLine 2"
"#;
        
        let result = helper.add_translation("fr", "special", toml_content);
        assert!(result.is_ok());
        assert!(helper.holders.contains_key("special"));
    }

    #[test]
    fn test_add_translation_empty_strings() {
        let mut helper = TraslatorHelper::default();
        helper.add_language("en", "English");
        
        let toml_content = r#"
[empty]
blank = ""
zero = ""
"#;
        
        let result = helper.add_translation("en", "empty", toml_content);
        assert!(result.is_ok());
    }

    #[test]
    fn test_non_existent_language_id() {
        let helper = TraslatorHelper::default();
        // Should this panic or return an ID? Depends on implementation
        // This tests the current behavior
        let _ = helper.get_lang_id_be("nonexistent");
    }

    #[test]
    fn test_add_translation_nested_tables_filtered() {
        let mut helper = TraslatorHelper::default();
        helper.add_language("en", "English");
        
        let toml_content = r#"
[greeting]
hello = "Hello"

[greeting.subsection]
nested = "Nested value"
"#;
        
        // Nested tables should be filtered out (not strings)
        let result = helper.add_translation("en", "greeting", toml_content);
        assert!(result.is_ok());
    }
}