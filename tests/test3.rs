#[cfg(test)]
mod get_language_name_tests {
    use bt_localized_txt::translator::TranslatorHelper;

    #[test]
    fn test_get_language_name_valid_id() {
        let mut translator = TranslatorHelper::default();
        let lang_id = translator.add_language("en", "English");
        let result = translator.get_language_name(lang_id);
        assert_eq!(result, Some("English".to_string()));
    }

    #[test]
    fn test_get_language_name_first_id() {
        let mut translator = TranslatorHelper::default();
        let first_id = translator.add_language("en", "English");
        let second_id = translator.add_language("es", "Spanish");
        
        assert_eq!(translator.get_language_name(first_id), Some("English".to_string()));
        assert_eq!(translator.get_language_name(second_id), Some("Spanish".to_string()));
    }

    #[test]
    fn test_get_language_name_single_language() {
        let mut translator = TranslatorHelper::default();
        let id = translator.add_language("fr", "French");
        assert_eq!(translator.get_language_name(id), Some("French".to_string()));
    }

    #[test]
    fn test_get_language_name_max_valid_id() {
        let mut translator = TranslatorHelper::default();
        let _id1 = translator.add_language("en", "English");
        let id2 = translator.add_language("es", "Spanish");
        
        // Last valid ID should work
        assert_eq!(translator.get_language_name(id2), Some("Spanish".to_string()));
    }

    #[test]
    fn test_get_language_name_invalid_id_out_of_bounds() {
        let mut translator = TranslatorHelper::default();
        translator.add_language("en", "English");
        
        // ID 1 exceeds the single language (only ID 0 exists)
        let result = translator.get_language_name(1);
        assert_eq!(result, None);
    }

    #[test]
    fn test_get_language_name_empty_state() {
        let translator = TranslatorHelper::default();
        let result = translator.get_language_name(0);
        assert_eq!(result, None);
    }

    #[test]
    fn test_get_language_name_boundary_condition() {
        let mut translator = TranslatorHelper::default();
        translator.add_language("en", "English");
        translator.add_language("es", "Spanish");
        
        // ID 2 is out of bounds (valid IDs are 0 and 1, len is 2)
        // Should return None when id >= len
        let result = translator.get_language_name(2);
        assert_eq!(result, None);
    }

    #[test]
    fn test_get_language_name_various_languages() {
        let mut translator = TranslatorHelper::default();
        let ids: Vec<u16> = vec![
            translator.add_language("en", "English"),
            translator.add_language("es", "Español"),
            translator.add_language("fr", "Français"),
            translator.add_language("de", "Deutsch"),
            translator.add_language("ja", "日本語"),
        ];

        let expected = vec!["English", "Español", "Français", "Deutsch", "日本語"];
        
        for (i, expected_name) in expected.iter().enumerate() {
            let result = translator.get_language_name(ids[i]);
            assert_eq!(result, Some(expected_name.to_string()));
        }
    }

    #[test]
    fn test_get_language_name_unicode_support() {
        let mut translator = TranslatorHelper::default();
        let id = translator.add_language("zh", "中文");
        assert_eq!(translator.get_language_name(id), Some("中文".to_string()));
    }

    #[test]
    fn test_get_language_name_multiple_calls_consistency() {
        let mut translator = TranslatorHelper::default();
        let id = translator.add_language("en", "English");
        
        // Multiple calls should return the same result
        let result1 = translator.get_language_name(id);
        let result2 = translator.get_language_name(id);
        assert_eq!(result1, result2);
        assert_eq!(result1, Some("English".to_string()));
    }

    #[test]
    fn test_get_language_name_very_large_id() {
        let mut translator = TranslatorHelper::default();
        translator.add_language("en", "English");
        
        // Test with a very large ID (u16::MAX)
        let result = translator.get_language_name(u16::MAX);
        assert_eq!(result, None);
    }
}