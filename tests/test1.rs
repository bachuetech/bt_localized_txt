// ... existing code ...

#[cfg(test)]
mod translator_sys_tests {
    use bt_localized_txt::languages::Languages;
    use bt_localized_txt::translator::TranslatorHelper;



    fn create_test_helper() -> TranslatorHelper {
        let mut helper = TranslatorHelper::default();
        helper.add_language("en", "English");
        helper.add_language("es", "Spanish");
        helper.add_language("fr", "French");
        helper
    }

    #[test]
    fn test_default_initialization() {
        let _helper = TranslatorHelper::default();
    }

    #[test]
    fn test_init_with_languages() {
        let languages = Languages::new();
        let _helper = TranslatorHelper::init(languages);
    }

    #[test]
    fn test_add_language() {
        bt_logger::build_logger("bachuetech", "translator_test", bt_logger::LogLevel::VERBOSE, bt_logger::LogTarget::STD_OUT, None);                   
        let mut helper = TranslatorHelper::default();
        let lang_id = helper.add_language("de", "German");
        assert!(lang_id == 0);
    }

    #[test]
    fn test_get_lang_id_be() {
        bt_logger::build_logger("bachuetech", "translator_test", bt_logger::LogLevel::VERBOSE, bt_logger::LogTarget::STD_OUT, None);                   
        let mut helper = TranslatorHelper::default();
        helper.add_language("en", "English");
        let lang_id = helper.add_language("de", "German");        
        assert!(lang_id == 1);        
        let lang_id = helper.get_lang_id_be("en");
        assert!(lang_id == 0);
    }

    #[test]
    fn test_add_translation_single_section() {
        let mut helper = create_test_helper();
        let toml = r#"
            [greetings]
            hello = "Hello"
            goodbye = "Goodbye"
        "#;
        
        let result = helper.add_translation("en", "greetings", toml);
        assert!(result.is_ok());
    }

    #[test]
    fn test_add_translation_multiple_languages() {
        let mut helper = create_test_helper();
        
        let en_toml = r#"
            [greetings]
            hello = "Hello"
        "#;
        
        let es_toml = r#"
            [greetings]
            hello = "Hola"
        "#;
        
        assert!(helper.add_translation("en", "greetings", en_toml).is_ok());
        assert!(helper.add_translation("es", "greetings", es_toml).is_ok());
    }

    #[test]
    fn test_add_translation_missing_section() {
        let mut helper = create_test_helper();
        let toml = r#"
            [greetings]
            hello = "Hello"
        "#;
        
        let result = helper.add_translation("en", "nonexistent", toml);
        assert!(result.is_err());
    }

    #[test]
    fn test_add_translation_invalid_toml() {
        let mut helper = create_test_helper();
        let invalid_toml = "this is not valid toml {[}";
        
        let result = helper.add_translation("en", "greetings", invalid_toml);
        assert!(result.is_err());
    }

    #[test]
    fn test_add_translation_empty_table() {
        let mut helper = create_test_helper();
        let toml = r#"
            [greetings]
        "#;
        
        let result = helper.add_translation("en", "greetings", toml);
        assert!(result.is_ok());
    }

    #[test]
    fn test_add_translation_mixed_types() {
        let mut helper = create_test_helper();
        let toml = r#"
            [greetings]
            hello = "Hello"
            number = 42
            array = ["not", "strings"]
        "#;
        
        let result = helper.add_translation("en", "greetings", toml);
        assert!(result.is_ok());
    }

    #[test]
    fn test_add_translation_non_table_section() {
        let mut helper = create_test_helper();
        let toml = r#"
            greetings = "not a table"
        "#;
        
        let result = helper.add_translation("en", "greetings", toml);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_translations_existing_language() {
        let mut helper = create_test_helper();
        let toml = r#"
            [greetings]
            hello = "Hello"
        "#;
        
        helper.add_translation("en", "greetings", toml).unwrap();
        let lang_id = helper.get_lang_id_be("en");
        let result = helper.get_translations(lang_id, "greetings");
        assert!(result.is_some());
    }

    #[test]
    fn test_get_translations_nonexistent_section() {
        let helper = create_test_helper();
        let lang_id = helper.get_lang_id_be("en");
        let result = helper.get_translations(lang_id, "nonexistent");
        assert!(result.is_none());
    }

    #[test]
    fn test_get_translations_nonexistent_language() {
        let mut helper = create_test_helper();
        let toml = r#"
            [greetings]
            hello = "Hello"
        "#;
        
        helper.add_translation("en", "greetings", toml).unwrap();
        let result = helper.get_translations(9999, "greetings");
        assert!(result.is_none());
    }

    #[test]
    fn test_multiple_sections() {
        let mut helper = create_test_helper();
        
        let greetings = r#"
            [greetings]
            hello = "Hello"
        "#;
        
        let farewells = r#"
            [farewells]
            goodbye = "Goodbye"
        "#;
        
        assert!(helper.add_translation("en", "greetings", greetings).is_ok());
        assert!(helper.add_translation("en", "farewells", farewells).is_ok());
        
        let lang_id = helper.get_lang_id_be("en");
        assert!(helper.get_translations(lang_id, "greetings").is_some());
        assert!(helper.get_translations(lang_id, "farewells").is_some());
    }

    #[test]
    fn test_add_translation_updates_existing_section() {
        let mut helper = create_test_helper();
        
        let toml1 = r#"
            [greetings]
            hello = "Hello"
        "#;
        
        let toml2 = r#"
            [greetings]
            hi = "Hi"
        "#;
        
        helper.add_translation("en", "greetings", toml1).unwrap();
        helper.add_translation("en", "greetings", toml2).unwrap();
        
        let lang_id = helper.get_lang_id_be("en");
        assert!(helper.get_translations(lang_id, "greetings").is_some());
    }

    #[test]
    fn test_clone_translator_helper() {
        let mut helper1 = create_test_helper();
        let toml = r#"
            [greetings]
            hello = "Hello"
        "#;
        
        helper1.add_translation("en", "greetings", toml).unwrap();
        
        let helper2 = helper1.clone();
        let lang_id = helper2.get_lang_id_be("en");
        assert!(helper2.get_translations(lang_id, "greetings").is_some());
    }

    #[test]
    fn test_add_translation_unicode_strings() {
        let mut helper = create_test_helper();
        let toml = r#"
            [greetings]
            hello = "你好"
            emoji = "👋🌍"
        "#;
        
        let result = helper.add_translation("en", "greetings", toml);
        assert!(result.is_ok());
    }

    #[test]
    fn test_add_translation_special_characters() {
        let mut helper = create_test_helper();
        let toml = r#"
            [greetings]
            hello = "Hello \"World\""
            newline = "Line1\nLine2"
        "#;
        
        let result = helper.add_translation("en", "greetings", toml);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_translations_after_multiple_additions() {
        let mut helper = create_test_helper();
        
        let toml1 = r#"
            [section]
            key1 = "value1"
        "#;
        
        let toml2 = r#"
            [section]
            key2 = "value2"
        "#;
        
        helper.add_translation("en", "section", toml1).unwrap();
        helper.add_translation("en", "section", toml2).unwrap();
        
        let lang_id = helper.get_lang_id_be("en");
        assert!(helper.get_translations(lang_id, "section").is_some());
    }

    #[test]
    fn test_add_translation_with_empty_string_values() {
        let mut helper = create_test_helper();
        let toml = r#"
            [greetings]
            hello = ""
            world = ""
        "#;
        
        let result = helper.add_translation("en", "greetings", toml);
        assert!(result.is_ok());
    }

    #[test]
    fn test_multiple_language_ids() {
        let helper = create_test_helper();
        
        let en_id = helper.get_lang_id_be("en");
        let es_id = helper.get_lang_id_be("es");
        let fr_id = helper.get_lang_id_be("fr");
        
        assert_ne!(en_id, es_id);
        assert_ne!(es_id, fr_id);
        assert_ne!(en_id, fr_id);
    }

    #[test]
    fn test_add_translation_large_section() {
        let mut helper = create_test_helper();
        let mut toml_content = String::from("[large_section]\n");
        
        for i in 0..100 {
            toml_content.push_str(&format!("key{} = \"value{}\"\n", i, i));
        }
        
        let result = helper.add_translation("en", "large_section", &toml_content);
        assert!(result.is_ok());
    }

    #[test]
    fn test_concurrent_section_names() {
        let mut helper = create_test_helper();
        
        let sections = vec!["section_a", "section_b", "section_c"];
        for section in &sections {
            let toml = format!("[{0}]\nkey = \"value\"", section);
            assert!(helper.add_translation("en", section, &toml).is_ok());
        }
        
        let lang_id = helper.get_lang_id_be("en");
        for section in &sections {
            assert!(helper.get_translations(lang_id, section).is_some());
        }
    }
}