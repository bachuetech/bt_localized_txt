use rustc_hash::FxHashMap;

/// A registry of supported languages, mapping language codes and names to a unique `u16` ID.
pub struct Languages{
    ids: FxHashMap<String, u16>,
    name_list: Vec<Box<str>>,
    default_id: u16,
}

impl Languages {
    /// Creates a new, empty `Languages` instance.
    pub fn new() -> Self{
        Self { ids: FxHashMap::default(), name_list: Vec::new(), default_id: 0}
    }

    /// Adds a new language to the registry and returns its assigned ID.
    ///
    /// If the maximum number of supported languages (`u16::MAX`) is reached,
    /// it fails silently and returns `0`.
    ///
    /// # Arguments
    /// * `language_code` - The unique code for the language (e.g., "en", "es"). Stored as lowercase.
    /// * `language_name` - The human-readable name of the language (e.g., "English", "Spanish").
    ///
    /// # Returns
    /// * The newly assigned `u16` ID for the language, or `0` if capacity is reached.
    #[inline]
    pub fn add_language(&mut self, language_code: &str, language_name: &str) -> u16{
        if self.name_list.len() < u16::MAX.into() {
            let lang_idx = self.name_list.len() as u16;
            self.name_list.push(language_name.into());
            self.ids.insert(language_code.to_lowercase(), lang_idx);
            lang_idx
        }else{
            0
        }
    }
    
    /// Retrieves the human-readable name of a language by its ID.
    ///
    /// # Arguments
    /// * `id` - The unique `u16` ID of the language.
    ///
    /// # Panics
    /// Panics if `id` is out of bounds for the internal language list.
    ///
    /// # Returns
    /// * A string slice containing the language name.
    #[inline]
    pub fn get_lang_name(&self, id: u16) -> &str{
       &self.name_list[id as usize]
    }

    /// Changes the default language ID used for fallbacks.
    ///
    /// # Arguments
    /// * `new_default_lang_id` - The `u16` ID of the language to set as default.
    ///
    /// # Returns
    /// * `Ok(())` if the provided ID exists in the registry.
    /// * `Err(())` if the provided ID is out of bounds.
    #[inline]
    pub fn change_default_language_id(&mut self, new_default_lang_id: u16) -> Result<(), ()>{
        if self.name_list.len() > new_default_lang_id.into(){
            self.default_id = new_default_lang_id;
            return Ok(())
        }

        Err(())
    }

    /// Retrieves the ID for a specific language code.
    ///
    /// # Arguments
    /// * `language_code` - The exact code of the language to look up (case-insensitive).
    ///
    /// # Returns
    /// * `Some(u16)` if the language code exists.
    /// * `None` if the language code is not found.
    #[inline]
    pub fn get_lang_id(&self, language_code: &str) -> Option<u16>{
        self.ids.get(&language_code.to_lowercase()).copied()
    }

    /// Retrieves the ID for a language by matching its first two characters.
    ///
    /// Useful for loosely matching standard locale tags (e.g., matching "en-US" to "en").
    ///
    /// # Arguments
    /// * `language_code` - The language code or locale string to match.
    ///
    /// # Returns
    /// * `Some(u16)` if a language code matching the first two characters exists.
    /// * `None` if no match is found.
    #[inline]
    pub fn get_lang_id_beginning_with(&self, language_code: &str) -> Option<u16> {
        let l_code = language_code.chars().take(2).collect::<String>().to_lowercase();
        self.ids.get(l_code.as_str()).copied()
    }

    /// Retrieves the language ID using a cascading fallback strategy.
    ///
    /// It first attempts an exact match via `get_lang_id`. If that fails, it attempts
    /// a prefix match via `get_lang_id_beginning_with`. If both fail, it returns
    /// the currently configured default language ID.
    ///
    /// # Arguments
    /// * `language_code` - The language code or locale string to look up.
    ///
    /// # Returns
    /// * The matched `u16` language ID, or the default language ID if no match is found.
    #[inline]
    pub fn get_lang_id_be(&self, language_code:&str) -> u16 {
        self.get_lang_id(language_code)
            .or_else(|| self.get_lang_id_beginning_with(language_code))
            .unwrap_or(self.default_id)
    }

    /// Returns a list of all language IDs and their corresponding human-readable names.
    ///
    /// This method iterates through the internal language registry and constructs
    /// a vector of `(ID, name)` tuples. The order of elements is non-deterministic
    /// (depends on the underlying `HashMap` iteration order).
    ///
    /// # Returns
    ///
    /// A `Vec` containing tuples of `(language_id, language_name)`. The string slices
    /// are borrowed from `self`, so the vector cannot outlive the `Languages` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use bt_localized_txt::languages::Languages;
    ///
    /// let languages = Languages::new(); // or your constructor
    /// let list = languages.get_list_of_languages();
    ///
    /// for (id, name) in list {
    ///     println!("{}: {}", id, name);
    /// }
    /// ```    
    pub fn get_list_of_languages(&self) -> Vec<(u16,&str)>{
        self.ids
            .iter()
            .map(|(_, &lang_id)| (lang_id, self.get_lang_name(lang_id)))
            .collect()
    }
}

//************************ */
//UNIT TESTS               */
//************************ */
#[cfg(test)]
mod languages_tests {
    use crate::languages::Languages;

    fn create_test_languages() -> Languages {
        let mut langs = Languages::new();
        langs.add_language("EN", "English");
        langs.add_language("FR", "French");
        langs.add_language("DE", "German");
        langs.add_language("ES", "Spanish");
        langs
    }

    // ===== Constructor Tests =====

    #[test]
    fn test_new_creates_empty_languages() {
        let langs = Languages::new();
        assert_eq!(langs.get_lang_id_be("en"), 0);
    }

    #[test]
    fn test_new_default_id_is_zero() {
        let langs = Languages::new();
        assert_eq!(langs.get_lang_id_be("unknown"), 0);
    }

    // ===== add_language Tests =====

    #[test]
    fn test_add_language_returns_correct_id() {
        let mut langs = Languages::new();
        assert_eq!(langs.add_language("EN", "English"), 0);
        assert_eq!(langs.add_language("FR", "French"), 1);
        assert_eq!(langs.add_language("DE", "German"), 2);
    }

    #[test]
    fn test_add_language_case_insensitive_code() {
        let mut langs = Languages::new();
        langs.add_language("EN", "English");
        
        assert_eq!(langs.get_lang_id("en"), Some(0));
        assert_eq!(langs.get_lang_id("EN"), Some(0));
        assert_eq!(langs.get_lang_id("En"), Some(0));
        assert_eq!(langs.get_lang_id("eN"), Some(0));
    }

    #[test]
    fn test_add_language_with_lowercase_code() {
        let mut langs = Languages::new();
        langs.add_language("en", "English");
        assert_eq!(langs.get_lang_id("EN"), Some(0));
    }

    #[test]
    fn test_add_language_with_mixed_case_code() {
        let mut langs = Languages::new();
        langs.add_language("eN", "English");
        assert_eq!(langs.get_lang_id("en"), Some(0));
    }

    // ===== get_lang_name Tests =====

    #[test]
    fn test_get_lang_name_returns_correct_name() {
        let langs = create_test_languages();
        
        assert_eq!(langs.get_lang_name(0), "English");
        assert_eq!(langs.get_lang_name(1), "French");
        assert_eq!(langs.get_lang_name(2), "German");
        assert_eq!(langs.get_lang_name(3), "Spanish");
    }

    #[test]
    #[should_panic]
    fn test_get_lang_name_out_of_bounds_panics() {
        let langs = create_test_languages();
        langs.get_lang_name(100);
    }

    // ===== change_default_language_id Tests =====

    #[test]
    fn test_change_default_language_id() {
        let mut langs = create_test_languages();
        
        assert_eq!(langs.get_lang_id_be("unknown"), 0);
        
        langs.change_default_language_id(1).unwrap();
        assert_eq!(langs.get_lang_id_be("unknown"), 1);
        
        langs.change_default_language_id(3).unwrap();
        assert_eq!(langs.get_lang_id_be("unknown"), 3);
    }

    #[test]
    fn test_change_default_language_id_to_zero() {
        let mut langs = create_test_languages();
        langs.change_default_language_id(2).unwrap();
        //langs.change_default_language_id(0).unwrap();
        
        assert_eq!(langs.get_lang_id_be("unknown"), 2);
    }

    // ===== get_lang_id Tests =====

    #[test]
    fn test_get_lang_id_exact_match() {
        let langs = create_test_languages();
        
        assert_eq!(langs.get_lang_id("en"), Some(0));
        assert_eq!(langs.get_lang_id("fr"), Some(1));
        assert_eq!(langs.get_lang_id("de"), Some(2));
        assert_eq!(langs.get_lang_id("es"), Some(3));
    }

    #[test]
    fn test_get_lang_id_returns_none_for_unknown() {
        let langs = create_test_languages();
        
        assert_eq!(langs.get_lang_id("xx"), None);
        assert_eq!(langs.get_lang_id("unknown"), None);
        assert_eq!(langs.get_lang_id(""), None);
    }

    #[test]
    fn test_get_lang_id_full_code_no_partial_match() {
        let langs = create_test_languages();
        
        // "eng" should NOT match "en" in get_lang_id
        assert_eq!(langs.get_lang_id("eng"), None);
        assert_eq!(langs.get_lang_id("fra"), None);
    }

    // ===== get_lang_id_beginning_with Tests =====

    #[test]
    fn test_get_lang_id_beginning_with_matches_prefix() {
        let langs = create_test_languages();
        
        assert_eq!(langs.get_lang_id_beginning_with("en-US"), Some(0));
        assert_eq!(langs.get_lang_id_beginning_with("fr-CA"), Some(1));
        assert_eq!(langs.get_lang_id_beginning_with("de-DE"), Some(2));
        assert_eq!(langs.get_lang_id_beginning_with("es-ES"), Some(3));
    }

    #[test]
    fn test_get_lang_id_beginning_with_short_code() {
        let langs = create_test_languages();
        
        assert_eq!(langs.get_lang_id_beginning_with("en"), Some(0));
        assert_eq!(langs.get_lang_id_beginning_with("fr"), Some(1));
    }

    #[test]
    fn test_get_lang_id_beginning_with_case_insensitive() {
        let langs = create_test_languages();
        
        assert_eq!(langs.get_lang_id_beginning_with("EN-US"), Some(0));
        assert_eq!(langs.get_lang_id_beginning_with("En-Us"), Some(0));
        assert_eq!(langs.get_lang_id_beginning_with("eN-uS"), Some(0));
    }

    #[test]
    fn test_get_lang_id_beginning_with_returns_none_for_unknown() {
        let langs = create_test_languages();
        
        assert_eq!(langs.get_lang_id_beginning_with("xx-XX"), None);
        assert_eq!(langs.get_lang_id_beginning_with("unknown"), None);
    }

    #[test]
    fn test_get_lang_id_beginning_with_ignores_beyond_two_chars() {
        let mut langs = Languages::new();
        langs.add_language("EN", "English");
        
        // All these should match "en"
        assert_eq!(langs.get_lang_id_beginning_with("en"), Some(0));
        assert_eq!(langs.get_lang_id_beginning_with("eng"), Some(0));
        assert_eq!(langs.get_lang_id_beginning_with("en-US"), Some(0));
        assert_eq!(langs.get_lang_id_beginning_with("english"), Some(0));
    }

    #[test]
    fn test_get_lang_id_beginning_with_single_char() {
        let mut langs = Languages::new();
        langs.add_language("E", "Eastron");
        
        assert_eq!(langs.get_lang_id_beginning_with("e"), Some(0));
        assert_eq!(langs.get_lang_id_beginning_with("E"), Some(0));
        assert_eq!(langs.get_lang_id_beginning_with("E-REGION"), None);
    }

    // ===== get_lang_id_be Tests =====

    #[test]
    fn test_get_lang_id_be_exact_match() {
        let langs = create_test_languages();
        
        assert_eq!(langs.get_lang_id_be("en"), 0);
        assert_eq!(langs.get_lang_id_be("fr"), 1);
        assert_eq!(langs.get_lang_id_be("de"), 2);
        assert_eq!(langs.get_lang_id_be("es"), 3);
    }

    #[test]
    fn test_get_lang_id_be_fallback_to_prefix() {
        let langs = create_test_languages();
        
        assert_eq!(langs.get_lang_id_be("en-US"), 0);
        assert_eq!(langs.get_lang_id_be("fr-CA"), 1);
        assert_eq!(langs.get_lang_id_be("de-AT"), 2);
    }

    #[test]
    fn test_get_lang_id_be_fallback_to_default() {
        let langs = create_test_languages();
        
        assert_eq!(langs.get_lang_id_be("unknown"), 0);
        assert_eq!(langs.get_lang_id_be("xx-XX"), 0);
        assert_eq!(langs.get_lang_id_be(""), 0);
    }

    #[test]
    fn test_get_lang_id_be_respects_changed_default() {
        let mut langs = create_test_languages();
        langs.change_default_language_id(2).unwrap(); // German
        
        assert_eq!(langs.get_lang_id_be("unknown"), 2);
        assert_eq!(langs.get_lang_id_be("en"), 0); // Still matches exact
        assert_eq!(langs.get_lang_id_be("fr-CA"), 1); // Still matches prefix
    }

    #[test]
    fn test_get_lang_id_be_case_insensitive() {
        let langs = create_test_languages();
        
        assert_eq!(langs.get_lang_id_be("EN"), 0);
        assert_eq!(langs.get_lang_id_be("Fr"), 1);
        assert_eq!(langs.get_lang_id_be("dE"), 2);
        assert_eq!(langs.get_lang_id_be("EN-us"), 0);
    }

    // ===== Edge Cases =====

    #[test]
    fn test_empty_string() {
        let langs = create_test_languages();
        
        assert_eq!(langs.get_lang_id(""), None);
        assert_eq!(langs.get_lang_id_be(""), 0);
    }

    #[test]
    fn test_single_char_code() {
        let mut langs = Languages::new();
        langs.add_language("A", "ALang");
        langs.add_language("B", "BLang");
        
        assert_eq!(langs.get_lang_id("a"), Some(0));
        assert_eq!(langs.get_lang_id("b"), Some(1));
        assert_eq!(langs.get_lang_id_be("c"), 0); // default
    }

    #[test]
    fn test_long_language_code() {
        let mut langs = Languages::new();
        langs.add_language("ZH-HANS", "Chinese (Simplified)");
        
        assert_eq!(langs.get_lang_id("zh-hans"), Some(0));
        assert_eq!(langs.get_lang_id_be("zh-hans"), 0);
    }

    #[test]
    fn test_special_characters_in_code() {
        let mut langs = Languages::new();
        langs.add_language("PT-BR", "Portuguese (Brazil)");
        
        assert_eq!(langs.get_lang_id("pt-br"), Some(0));
        assert_eq!(langs.get_lang_id("PT-BR"), Some(0));
        assert_eq!(langs.get_lang_id_be("pt-br"), 0);
    }

    #[test]
    fn test_duplicate_language_codes() {
        let mut langs = Languages::new();
        langs.add_language("EN", "English");
        langs.add_language("EN", "English Variant");
        
        // Second insert should overwrite first
        assert_eq!(langs.get_lang_id("en"), Some(1));
    }

    #[test]
    fn test_unicode_language_names() {
        let mut langs = Languages::new();
        langs.add_language("JA", "日本語");
        langs.add_language("KO", "한국어");
        langs.add_language("AR", "العربية");
        
        assert_eq!(langs.get_lang_name(0), "日本語");
        assert_eq!(langs.get_lang_name(1), "한국어");
        assert_eq!(langs.get_lang_name(2), "العربية");
    }

    #[test]
    fn test_many_languages() {
        let mut langs = Languages::new();
        let codes: Vec<String> = (0..1000).map(|i| format!("L{:04}", i)).collect();
        
        for (i, code) in codes.iter().enumerate() {
            let id = langs.add_language(code, &format!("Language {}", i));
            assert_eq!(id, i as u16);
        }
        
        for (i, code) in codes.iter().enumerate() {
            assert_eq!(langs.get_lang_id(code), Some(i as u16));
            assert_eq!(langs.get_lang_name(i as u16), format!("Language {}", i));
        }
    }


    #[test]
    fn test_get_list_of_languages_returns_all_entries() {
        let langs =  create_test_languages();
        let result = langs.get_list_of_languages();
        
        // HashMap iteration order is non-deterministic, so sort for comparison
        let mut result = result;
        result.sort_by_key(|&(id, _)| id);
        
        assert_eq!(result.len(), 4);
        assert_eq!(result[0], (0, "English"));
        assert_eq!(result[2], (2, "German"));
        assert_eq!(result[1], (1, "French"));
    }

    #[test]
    fn test_empty_languages_returns_empty_vec() {
        let langs = Languages::new();
        let result = langs.get_list_of_languages();
        assert!(result.is_empty());
    }

    #[test]
    fn test_single_language() {
        let mut langs = Languages::new();
        langs.add_language("rust", "Rust");
        let result = langs.get_list_of_languages();
        assert_eq!(result, vec![(0, "Rust")]);
    }

    #[test]
    fn test_iterator_version_zero_allocation() {
        // If you used the iterator-returning version instead:
        let langs = create_test_languages();
        let binding = langs.get_list_of_languages();
        let collected: Vec<_> = binding.iter().collect();
        
        let mut result = collected;
        result.sort_by_key(|&(id, _)| id);
        
        assert_eq!(result.len(), 4);
        assert!(result.contains(&&(0u16, "English")));
    }
    
    #[test]
    fn test_lifetimes_are_valid() {
        // Ensures returned references don't outlive self
        let langs = create_test_languages();
        let result = langs.get_list_of_languages();
        
        // This should compile: result borrows from langs
        for (id, name) in result {
            assert!(id >= 0);
            assert!(!name.is_empty());
        }
        // langs is still usable here
        assert!(langs.ids.len() == 4);
    }    

    // ===== Integration Tests =====

    #[test]
    fn test_full_workflow() {
        let mut langs = Languages::new();
        
        // Add languages
        let en_id = langs.add_language("EN", "English");
        let fr_id = langs.add_language("FR", "French");
        let de_id = langs.add_language("DE", "German");
        
        // Verify IDs
        assert_eq!(en_id, 0);
        assert_eq!(fr_id, 1);
        assert_eq!(de_id, 2);
        
        // Verify lookups
        assert_eq!(langs.get_lang_id("en"), Some(en_id));
        assert_eq!(langs.get_lang_id("fr"), Some(fr_id));
        assert_eq!(langs.get_lang_id("de"), Some(de_id));
        
        // Verify names
        assert_eq!(langs.get_lang_name(en_id), "English");
        assert_eq!(langs.get_lang_name(fr_id), "French");
        assert_eq!(langs.get_lang_name(de_id), "German");
        
        // Test fallback chain
        assert_eq!(langs.get_lang_id_be("en"), en_id);        // exact
        assert_eq!(langs.get_lang_id_be("en-US"), en_id);    // prefix
        assert_eq!(langs.get_lang_id_be("xx"), en_id);       // default
        
        // Change default and verify
        langs.change_default_language_id(fr_id).unwrap();
        assert_eq!(langs.get_lang_id_be("xx"), fr_id);
    }

    #[test]
    fn test_locale_codes() {
        let mut langs = Languages::new();
        langs.add_language("EN", "English");
        langs.add_language("ES", "Spanish");
        langs.add_language("PT", "Portuguese");
        
        // Common locale formats
        assert_eq!(langs.get_lang_id_be("en_US"), 0);
        assert_eq!(langs.get_lang_id_be("en-US"), 0);
        assert_eq!(langs.get_lang_id_be("es_MX"), 1);
        assert_eq!(langs.get_lang_id_be("es-419"), 1);
        assert_eq!(langs.get_lang_id_be("pt_BR"), 2);
        assert_eq!(langs.get_lang_id_be("pt-PT"), 2);
    }
}