#[cfg(test)]
mod translator_sys_tests2 {
    use bt_localized_txt::translator::TranslatorHelper;


    // =========================================================================
    // Tests for add_translation_with_lang_id
    // =========================================================================

    #[test]
    fn test_add_translation_with_lang_id_basic() {
        let mut translator = TranslatorHelper::default();
        let lang_id = translator.add_language("en", "English");

        let toml = r#"
[menu]
file = "File"
edit = "Edit"
"#;

        let result = translator.add_translation_with_lang_id(lang_id, "menu", toml);
        assert!(result.is_ok());

        // Verify translations were added
        let file_translation = translator.get_translation(lang_id, "menu", "file");
        assert_eq!(file_translation, Some("File".to_string()));

        let edit_translation = translator.get_translation(lang_id, "menu", "edit");
        assert_eq!(edit_translation, Some("Edit".to_string()));
    }

    #[test]
    fn test_add_translation_with_lang_id_invalid_toml() {
        let mut translator = TranslatorHelper::default();
        let lang_id = translator.add_language("en", "English");

        let invalid_toml = r#"
[menu
file = "File"
"#;

        let result = translator.add_translation_with_lang_id(lang_id, "menu", invalid_toml);
        assert!(result.is_err());
    }

    #[test]
    fn test_add_translation_with_lang_id_missing_section() {
        let mut translator = TranslatorHelper::default();
        let lang_id = translator.add_language("en", "English");

        let toml = r#"
[other_section]
key = "value"
"#;

        let result = translator.add_translation_with_lang_id(lang_id, "menu", toml);
        assert!(result.is_err());
    }

    #[test]
    fn test_add_translation_with_lang_id_empty_section() {
        let mut translator = TranslatorHelper::default();
        let lang_id = translator.add_language("en", "English");

        let toml = r#"
[menu]
"#;

        // Empty sections should return Ok
        let result = translator.add_translation_with_lang_id(lang_id, "menu", toml);
        assert!(result.is_ok());

        // But no translations should be retrievable
        let translation = translator.get_translation(lang_id, "menu", "file");
        assert_eq!(translation, None);
    }

    #[test]
    fn test_add_translation_with_lang_id_section_not_table() {
        let mut translator = TranslatorHelper::default();
        let lang_id = translator.add_language("en", "English");

        let toml = r#"
menu = "not a table"
"#;

        let result = translator.add_translation_with_lang_id(lang_id, "menu", toml);
        assert!(result.is_err());
    }

    #[test]
    fn test_add_translation_with_lang_id_multiple_languages() {
        let mut translator = TranslatorHelper::default();
        let en_id = translator.add_language("en", "English");
        let es_id = translator.add_language("es", "Spanish");

        let en_toml = r#"
[menu]
file = "File"
"#;

        let es_toml = r#"
[menu]
file = "Archivo"
"#;

        let result_en = translator.add_translation_with_lang_id(en_id, "menu", en_toml);
        let result_es = translator.add_translation_with_lang_id(es_id, "menu", es_toml);

        assert!(result_en.is_ok());
        assert!(result_es.is_ok());

        // Verify both languages have their own translations
        assert_eq!(
            translator.get_translation(en_id, "menu", "file"),
            Some("File".to_string())
        );
        assert_eq!(
            translator.get_translation(es_id, "menu", "file"),
            Some("Archivo".to_string())
        );
    }

    #[test]
    fn test_add_translation_with_lang_id_merges_existing_section() {
        let mut translator = TranslatorHelper::default();
        let lang_id = translator.add_language("en", "English");

        let toml1 = r#"
[menu]
file = "File"
"#;

        let toml2 = r#"
[menu]
edit = "Edit"
"#;

        translator.add_translation_with_lang_id(lang_id, "menu", toml1).unwrap();
        translator.add_translation_with_lang_id(lang_id, "menu", toml2).unwrap();

        // Both translations should exist
        assert_eq!(
            translator.get_translation(lang_id, "menu", "file"),
            Some("File".to_string())
        );
        assert_eq!(
            translator.get_translation(lang_id, "menu", "edit"),
            Some("Edit".to_string())
        );
    }

    #[test]
    fn test_add_translation_with_lang_id_skips_non_string_values() {
        let mut translator = TranslatorHelper::default();
        let lang_id = translator.add_language("en", "English");

        let toml = r#"
[menu]
file = "File"
count = 42
enabled = true
"#;

        let result = translator.add_translation_with_lang_id(lang_id, "menu", toml);
        assert!(result.is_ok());

        // String values should be stored
        assert_eq!(
            translator.get_translation(lang_id, "menu", "file"),
            Some("File".to_string())
        );

        // Non-string values should be skipped
        assert_eq!(translator.get_translation(lang_id, "menu", "count"), None);
        assert_eq!(translator.get_translation(lang_id, "menu", "enabled"), None);
    }

    #[test]
    fn test_add_translation_with_lang_id_multiple_sections() {
        let mut translator = TranslatorHelper::default();
        let lang_id = translator.add_language("en", "English");

        let toml = r#"
[menu]
file = "File"

[messages]
welcome = "Welcome!"
"#;

        translator.add_translation_with_lang_id(lang_id, "menu", toml).unwrap();
        translator.add_translation_with_lang_id(lang_id, "messages", toml).unwrap();

        assert_eq!(
            translator.get_translation(lang_id, "menu", "file"),
            Some("File".to_string())
        );
        assert_eq!(
            translator.get_translation(lang_id, "messages", "welcome"),
            Some("Welcome!".to_string())
        );
    }

    // =========================================================================
    // Tests for get_translation
    // =========================================================================

    #[test]
    fn test_get_translation_existing_key() {
        let mut translator = TranslatorHelper::default();
        let lang_id = translator.add_language("en", "English");

        let toml = r#"
[menu]
file = "File"
"#;

        translator.add_translation_with_lang_id(lang_id, "menu", toml).unwrap();

        let result = translator.get_translation(lang_id, "menu", "file");
        assert_eq!(result, Some("File".to_string()));
    }

    #[test]
    fn test_get_translation_nonexistent_key() {
        let mut translator = TranslatorHelper::default();
        let lang_id = translator.add_language("en", "English");

        let toml = r#"
[menu]
file = "File"
"#;

        translator.add_translation_with_lang_id(lang_id, "menu", toml).unwrap();

        let result = translator.get_translation(lang_id, "menu", "nonexistent");
        assert_eq!(result, None);
    }

    #[test]
    fn test_get_translation_nonexistent_section() {
        let mut translator = TranslatorHelper::default();
        let lang_id = translator.add_language("en", "English");

        let result = translator.get_translation(lang_id, "nonexistent_section", "key");
        assert_eq!(result, None);
    }

    #[test]
    fn test_get_translation_nonexistent_language() {
        let mut translator = TranslatorHelper::default();
        let en_id = translator.add_language("en", "English");

        let toml = r#"
[menu]
file = "File"
"#;

        translator.add_translation_with_lang_id(en_id, "menu", toml).unwrap();

        // Use a language ID that doesn't have translations
        let result = translator.get_translation(999, "menu", "file");
        assert_eq!(result, None);
    }

    #[test]
    fn test_get_translation_unicode_support() {
        let mut translator = TranslatorHelper::default();
        let en_id = translator.add_language("en", "English");
        let zh_id = translator.add_language("zh", "Chinese");
        let ru_id = translator.add_language("ru", "Russian");

        let en_toml = r#"
[menu]
file = "File"
"#;

        let zh_toml = r#"
[menu]
file = "文件"
"#;

        let ru_toml = r#"
[menu]
file = "Файл"
"#;

        translator.add_translation_with_lang_id(en_id, "menu", en_toml).unwrap();
        translator.add_translation_with_lang_id(zh_id, "menu", zh_toml).unwrap();
        translator.add_translation_with_lang_id(ru_id, "menu", ru_toml).unwrap();

        assert_eq!(
            translator.get_translation(en_id, "menu", "file"),
            Some("File".to_string())
        );
        assert_eq!(
            translator.get_translation(zh_id, "menu", "file"),
            Some("文件".to_string())
        );
        assert_eq!(
            translator.get_translation(ru_id, "menu", "file"),
            Some("Файл".to_string())
        );
    }

    #[test]
    fn test_get_translation_empty_value() {
        let mut translator = TranslatorHelper::default();
        let lang_id = translator.add_language("en", "English");

        let toml = r#"
[menu]
empty = ""
"#;

        translator.add_translation_with_lang_id(lang_id, "menu", toml).unwrap();

        let result = translator.get_translation(lang_id, "menu", "empty");
        assert_eq!(result, Some(String::new()));
    }

    #[test]
    fn test_get_translation_special_characters() {
        let mut translator = TranslatorHelper::default();
        let lang_id = translator.add_language("en", "English");

        let toml = r#"
[menu]
special = "Line 1\nLine 2\tTabbed \"Quoted\""
"#;

        translator.add_translation_with_lang_id(lang_id, "menu", toml).unwrap();

        let result = translator.get_translation(lang_id, "menu", "special");
        assert_eq!(result, Some("Line 1\nLine 2\tTabbed \"Quoted\"".to_string()));
    }

    #[test]
    fn test_get_translation_case_sensitive_keys() {
        let mut translator = TranslatorHelper::default();
        let lang_id = translator.add_language("en", "English");

        let toml = r#"
[menu]
File = "Upper"
file = "Lower"
"#;

        translator.add_translation_with_lang_id(lang_id, "menu", toml).unwrap();

        assert_eq!(
            translator.get_translation(lang_id, "menu", "File"),
            Some("Upper".to_string())
        );
        assert_eq!(
            translator.get_translation(lang_id, "menu", "file"),
            Some("Lower".to_string())
        );
    }

    #[test]
    fn test_get_translation_returns_owned_string() {
        let mut translator = TranslatorHelper::default();
        let lang_id = translator.add_language("en", "English");

        let toml = r#"
[menu]
file = "File"
"#;

        translator.add_translation_with_lang_id(lang_id, "menu", toml).unwrap();

        let result = translator.get_translation(lang_id, "menu", "file").unwrap();
        
        // Modify the returned string to ensure it's owned, not a reference
        let modified = result + " Menu";
        assert_eq!(modified, "File Menu");
        
        // Original should still be intact
        assert_eq!(
            translator.get_translation(lang_id, "menu", "file"),
            Some("File".to_string())
        );
    }
}