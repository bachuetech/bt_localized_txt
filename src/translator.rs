use std::borrow::Cow;

use bt_any_error::any_err::AnyErr;
use bt_logger::get_error;
use rustc_hash::FxHashMap;

use crate::languages::Languages;
use crate::localizer::{Localizer, StringValues};

/// A helper struct for managing translations across multiple languages.
///
/// `TraslatorHelper` provides a centralized interface for adding languages,
/// importing translations from TOML sources, and retrieving localized strings
/// organized by sections.
///
/// # Structure
///
/// - `languages`: Manages registered languages and their unique identifiers
/// - `holders`: Maps translation section names to their respective [`Localizer`] instances
///
/// # Example
///
/// ```no_run
/// use bt_localized_txt::translator::TranslatorHelper;
/// let mut translator = TranslatorHelper::default();
/// translator.add_language("en", "English");
/// translator.add_translation("en", "menu", "[menu]\nfile = \"File\"");
/// ```
#[derive(Clone)]
pub struct TranslatorHelper {
    languages: Languages,
    holders: FxHashMap<String, Localizer>,
}

impl TranslatorHelper {
    /// Creates a new `TraslatorHelper` with default settings.
    ///
    /// Initializes an empty translation holder and a default [`Languages`] instance.
    ///
    /// # Returns
    ///
    /// A new `TraslatorHelper` instance with no languages or translations configured.
    ///
    /// # Example
    ///```
    /// use bt_localized_txt::translator::TranslatorHelper;
    /// let translator = TranslatorHelper::default();
    /// ```
    pub fn default() -> TranslatorHelper {
        TranslatorHelper { 
            holders: FxHashMap::default(), 
            languages: Languages::new() 
        }
    }

    /// Creates a new `TraslatorHelper` with pre-configured languages.
    ///
    /// Use this constructor when you have an existing [`Languages`] instance
    /// that you want to use.
    ///
    /// # Arguments
    ///
    /// * `languages` - A [`Languages`] instance containing pre-registered languages
    ///
    /// # Returns
    ///
    /// A new `TraslatorHelper` instance with the provided languages configuration.
    pub fn init(languages: Languages) -> TranslatorHelper {
        TranslatorHelper { 
            holders: FxHashMap::default(), 
            languages 
        }
    }

    /// Adds a new language to the translation system.
    ///
    /// Registers a language with its code and human-readable name.
    /// Each language is assigned a unique identifier for efficient lookup.
    ///
    /// # Arguments
    ///
    /// * `language_code` - The ISO language code (e.g., "en", "es", "fr", "de")
    /// * `language_name` - The human-readable name (e.g., "English", "Spanish")
    ///
    /// # Returns
    ///
    /// A unique `u16` identifier for the newly added language.
    ///
    /// # Example
    ///
    /// ```
    /// use bt_localized_txt::translator::TranslatorHelper;
    /// let mut translator = TranslatorHelper::default();
    /// let lang_id = translator.add_language("en", "English");
    /// assert_eq!(lang_id, 0); // First language gets ID 0
    /// ```
    pub fn add_language(&mut self, language_code: &str, language_name: &str) -> u16 {
        self.languages.add_language(language_code, language_name)
    }

    /// Retrieves the unique identifier for a language by its code.
    ///
    /// # Arguments
    ///
    /// * `language_code` - The language code to look up (e.g., "en")
    ///
    /// # Returns
    ///
    /// The `u16` identifier associated with the given language code.
    /// Return default langugage if not found.
    ///
    pub fn get_lang_id_be(&self, language_code: &str) -> u16 {
        self.languages.get_lang_id_be(language_code)
    }

    /// Retrieves a list of all registered languages.
    ///
    /// # Returns
    ///
    /// A vector of tuples containing:
    /// - Language ID (`u16`)
    /// - Language name (`&str`)
    ///
    pub fn get_list_of_languages(&self) -> Vec<(u16, &str)> {
        self.languages.get_list_of_languages()
    }

    /// Adds translations from a TOML-formatted string for a specific language and section.
    ///
    /// Parses TOML content and extracts key-value translation pairs from the
    /// specified section, storing them in the appropriate [`Localizer`].
    ///
    /// # Arguments
    ///
    /// * `language_code` - The language code to associate translations with (e.g., "en")
    /// * `translation_section` - The section name within the TOML to extract (e.g., "menu", "messages")
    /// * `content_toml` - TOML-formatted string containing translation pairs
    ///
    /// # Returns
    ///
    /// - `Ok(())` - Translations were successfully added
    /// - `Err(AnyErr)` - An error occurred during parsing or extraction
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The TOML content cannot be parsed
    /// - The specified section does not exist in the TOML
    /// - The section exists but is not a valid TOML table
    ///
    /// # Example TOML Format
    ///
    /// ```toml
    /// [menu]
    /// file = "File"
    /// edit = "Edit"
    /// view = "View"
    ///
    /// [messages]
    /// welcome = "Welcome!"
    /// goodbye = "Goodbye!"
    /// ```
    ///
    /// # Example Usage
    ///
    /// ```
    /// use bt_localized_txt::translator::TranslatorHelper;
    /// let mut translator = TranslatorHelper::default();
    /// translator.add_language("en", "English");
    /// 
    /// let toml = r#"
    /// [menu]
    /// file = "File"
    /// edit = "Edit"
    /// "#;
    /// 
    /// let _ = translator.add_translation("en", "menu", toml);
    /// ```
    ///
    /// # Performance Notes
    ///
    /// - Uses `Cow<'_, str>` to avoid unnecessary string allocations during parsing
    /// - Pre-allocates the localizer entry before inserting translation pairs
    /// - Skips empty tables and empty translation sets early
    ///
    /// # Memory Note
    ///
    /// This method uses `Box::leak` to create `'static` string references for
    /// efficient storage. These strings will persist for the lifetime of the
    /// program and will not be deallocated. This is intentional for translation
    /// data which is typically loaded once and never unloaded.
    pub fn add_translation(
        &mut self, 
        language_code: &str, 
        translation_section: &str, 
        content_toml: &str
    ) -> Result<(), AnyErr> {
        let lang_id = self.get_lang_id_be(language_code);
        self.add_translation_with_lang_id(lang_id,translation_section,content_toml)
    }

    /// Adds translations from a TOML-formatted string using a language ID.
    ///
    /// This is a more efficient variant of [`add_translation`] when you already
    /// have the language ID, avoiding an extra lookup by language code.
    ///
    /// Parses TOML content and extracts key-value translation pairs from the
    /// specified section, storing them in the appropriate [`Localizer`].
    ///
    /// # Arguments
    ///
    /// * `language_id` - The unique identifier of the target language (from [`add_language`])
    /// * `translation_section` - The section name within the TOML to extract (e.g., "menu", "messages")
    /// * `content_toml` - TOML-formatted string containing translation pairs
    ///
    /// # Returns
    ///
    /// - `Ok(())` - Translations were successfully added, or the section was empty
    /// - `Err(AnyErr)` - An error occurred during parsing or extraction
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The TOML content cannot be parsed (invalid syntax)
    /// - The specified section does not exist in the TOML document
    /// - The section exists but is not a valid TOML table (e.g., it's a scalar or array)
    ///
    /// # Example
    ///
    /// ```
    /// use bt_localized_txt::translator::TranslatorHelper;
    /// use bt_localized_txt::languages::Languages;
    ///
    /// let mut translator = TranslatorHelper::default();
    /// let lang_id = translator.add_language("en", "English");
    ///
    /// let toml = r#"
    /// [menu]
    /// file = "File"
    /// edit = "Edit"
    /// "#;
    ///
    /// translator.add_translation_with_lang_id(lang_id, "menu", toml);
    /// 
    /// let translation = translator.get_translation(lang_id, "menu", "file");
    /// assert_eq!(translation, Some("File".to_string()));
    /// ```
    ///
    /// # Behavior Notes
    ///
    /// - **Empty sections**: Silently accepts empty tables without error
    /// - **Non-string values**: Keys with non-string values are skipped silently
    /// - **New sections**: Automatically creates the [`Localizer`] if the section is new
    /// - **Existing sections**: Merges new translations into the existing section
    ///
    /// # Memory Note
    ///
    /// This method uses `Box::leak` to create `'static` string references for
    /// efficient storage. These strings persist for the program's lifetime and
    /// are never deallocated. This is intentional for translation data that is
    /// typically loaded once at startup.
    ///
    /// # Performance
    ///
    /// - Uses `Cow<'_, str>` to minimize allocations during TOML parsing
    /// - Pre-allocates the localizer entry before inserting translation pairs
    /// - Skips empty tables early to avoid unnecessary work
    pub fn add_translation_with_lang_id(
        &mut self, 
        language_id: u16, 
        translation_section: &str, 
        content_toml: &str
    ) -> Result<(), AnyErr> {
        let toml_table: toml::Table = toml::from_str(content_toml)?;
        let section = match toml_table.get(translation_section) {
            Some(s) => s,
            None => return Err(
                get_error!("", "Cannot find section '{}' in TOML table (None)", translation_section).into()
            ),
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
                localizer.insert_batch(language_id, owned_pairs);
            }
        } else {
            return Err(
                get_error!("", "Cannot parse section '{}' in TOML table (None)", translation_section).into()
            );
        }

        Ok(())
    }

    /// Retrieves all translations for a specific language and section.
    ///
    /// Looks up the [`Localizer`] for the given section and returns
    /// the translation values associated with the specified language.
    ///
    /// # Arguments
    ///
    /// * `language_id` - The unique identifier of the language (from [`add_language`])
    /// * `translation_section` - The section name to retrieve translations from
    ///
    /// # Returns
    ///
    /// - `Some(StringValues)` - Container with translations if found
    /// - `None` - If the section doesn't exist or has no translations for the language
    pub fn get_translations(&self, language_id: u16, translation_section: &str) -> Option<StringValues> {
        // Direct access without intermediate Option checks
        self.holders
            .get(translation_section)
            .and_then(|t| t.get_string_values(language_id))
    }

    /// Retrieves a single translation string by its key.
    ///
    /// Looks up the translation for a specific key within a section and language.
    /// This is the primary method for accessing individual localized strings.
    ///
    /// # Arguments
    ///
    /// * `language_id` - The unique identifier of the language (from [`add_language`])
    /// * `translation_section` - The section name containing the translation (e.g., "menu")
    /// * `translation_id` - The translation key to look up (e.g., "file", "edit")
    ///
    /// # Returns
    ///
    /// - `Some(String)` - The translated string if found
    /// - `None` - If any of the following are true:
    ///   - The section doesn't exist
    ///   - The language has no translations in the section
    ///   - The translation key doesn't exist
    ///
    /// # Example
    ///
    /// ```
    /// use bt_localized_txt::translator::TranslatorHelper;
    ///
    /// let mut translator = TranslatorHelper::default();
    /// let en_id = translator.add_language("en", "English");
    /// let es_id = translator.add_language("es", "Spanish");
    ///
    /// let en_toml = r#"
    /// [menu]
    /// file = "File"
    /// "#;
    ///
    /// let es_toml = r#"
    /// [menu]
    /// file = "Archivo"
    /// "#;
    ///
    /// translator.add_translation("en", "menu", en_toml).unwrap();
    /// translator.add_translation("es", "menu", es_toml).unwrap();
    ///
    /// // Retrieve English translation
    /// let en_file = translator.get_translation(en_id, "menu", "file");
    /// assert_eq!(en_file, Some("File".to_string()));
    ///
    /// // Retrieve Spanish translation
    /// let es_file = translator.get_translation(es_id, "menu", "file");
    /// assert_eq!(es_file, Some("Archivo".to_string()));
    ///
    /// // Non-existent key returns None
    /// let missing = translator.get_translation(en_id, "menu", "nonexistent");
    /// assert_eq!(missing, None);
    /// ```
    ///
    /// # Performance
    ///
    /// - Uses direct hash map lookups without intermediate Option checks
    /// - Returns an owned `String` to avoid lifetime issues with internal storage   
    pub fn get_translation(&self, language_id: u16, translation_section: &str, translation_id: &str) -> Option<String>{
        // Direct access without intermediate Option checks
        self.holders
            .get(translation_section)
            .and_then(|t| t.get_string_value(language_id, translation_id))
    }

    /// Retrieves the human-readable name of a language by its unique identifier.
    ///
    /// Looks up the language name associated with the given `language_id`.
    /// The name returned is the one provided during [`add_language`] registration
    /// (e.g., "English", "Spanish", "French").
    ///
    /// # Arguments
    ///
    /// * `language_id` - The unique `u16` identifier returned by [`add_language`]
    ///
    /// # Returns
    ///
    /// - `Some(String)` - The human-readable language name if the ID exists
    /// - `None` - If the `language_id` is out of bounds (not registered)
    ///
    /// # Example
    ///
    /// ```
    /// use bt_localized_txt::translator::TranslatorHelper;
    ///
    /// let mut translator = TranslatorHelper::default();
    /// let lang_id = translator.add_language("en", "English");
    ///
    /// let name = translator.get_language_name(lang_id);
    /// assert_eq!(name, Some("English".to_string()));
    ///
    /// // Invalid ID returns None
    /// let invalid = translator.get_language_name(999);
    /// assert_eq!(invalid, None);
    /// ```
    ///
    /// # Implementation Notes
    ///
    /// - Uses bounds checking against the internal languages list before lookup
    /// - Returns an owned `String` to avoid lifetime dependencies on internal storage
    /// - IDs are 0-indexed, the last valid index is `len - 1`    
    pub fn get_language_name(&self, language_id: u16) -> Option<String>{
        if language_id as usize >= self.languages.get_list_of_languages().len() { return None}
        Some(self.languages.get_lang_name(language_id).to_owned())
    }    
}


#[cfg(test)]
mod translator_tests {
    use crate::languages::Languages;
use crate::translator::TranslatorHelper;

    #[test]
    fn test_default() {
        let helper = TranslatorHelper::default();
        assert_eq!(helper.holders.len(), 0);
    }

    #[test]
    fn test_init() {
        let languages = Languages::new();
        let helper = TranslatorHelper::init(languages);
        assert_eq!(helper.holders.len(), 0);
    }

    #[test]
    fn test_add_language_and_get_id() {
        let mut helper = TranslatorHelper::default();
        
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
        let mut helper = TranslatorHelper::default();
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
        let mut helper = TranslatorHelper::default();
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
        let mut helper = TranslatorHelper::default();
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
        let mut helper = TranslatorHelper::default();
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
        let mut helper = TranslatorHelper::default();
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
        let mut helper = TranslatorHelper::default();
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
        let mut helper = TranslatorHelper::default();
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
        let mut helper = TranslatorHelper::default();
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
        let mut helper = TranslatorHelper::default();
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
        let mut helper = TranslatorHelper::default();
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
        let helper = TranslatorHelper::default();
        // Should this panic or return an ID? Depends on implementation
        // This tests the current behavior
        let _ = helper.get_lang_id_be("nonexistent");
    }

    #[test]
    fn test_add_translation_nested_tables_filtered() {
        let mut helper = TranslatorHelper::default();
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