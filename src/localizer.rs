use rustc_hash::FxHashMap;

/// A collection of localized string values by a string code.
/// format: localized string code, string value
pub struct StringValues{
    values: FxHashMap<String, String>,
}

/// A collection of [`StringValues`] representing different languages, keyed by a numeric language ID.
pub struct Locale{
    locale: FxHashMap<u16,StringValues >
}


impl Default for StringValues {
    fn default() -> Self {
        Self {
            values: FxHashMap::default(),
        }
    }
}

impl Clone for StringValues {
    fn clone(&self) -> Self {
        Self {
            values: self.values.clone(),
        }
    }
}

impl StringValues {
    /// Retrieves a cloned string value associated with the specified string code.
    ///
    /// # Arguments
    /// * `string_code` - The code identifying the desired string.
    ///
    /// # Returns
    /// * `Some(String)` if the string code exists.
    /// * `None` if the string code does not exist.
    pub fn get_string_value(&self, string_code: &str) -> Option<String>{
        self.values.get(string_code).cloned()
    }

    /// Retrieves a string value associated with the specified string code, or a default empty string.
    ///
    /// # Arguments
    /// * `string_code` - The code identifying the desired string.
    ///
    /// # Returns
    /// * The string value if found, or an empty `String` if not found.
    pub fn get_string_value_or_default(&self, string_code: &str) -> String{
        self.values.get(string_code).unwrap_or(&String::new()).to_owned()
    }     
}

impl Locale {
    /// Creates a new, empty `Locale` instance.
    pub fn new() -> Self{
        Self { locale: FxHashMap::default() }
    }
    
    /// Inserts or updates a localized string value for a specific language.
    ///
    /// If the entry already exists, its memory is reused to avoid extra allocations.
    ///
    /// # Arguments
    /// * `language_id` - The numeric ID of the language.
    /// * `string_code` - The code identifying the string.
    /// * `string_value` - The translated value of the string.
    pub fn insert(&mut self, language_id: u16, string_code: &str, string_value: &str){
        let string_map = self.locale.entry(language_id).or_default();
        
        // Overwrite existing values (or insert new) with zero extra allocations
        match string_map.values.entry(string_code.to_string()) {
            std::collections::hash_map::Entry::Occupied(mut entry) => {
                *entry.get_mut() = string_value.to_string();  // Reuse existing String memory
            }
            std::collections::hash_map::Entry::Vacant(entry) => {
                entry.insert(string_value.to_string());
            }
        }
    }

    /// Inserts a batch of localized string values for a specific language.
    ///
    /// Pre-allocates memory based on the iterator's size hint for improved performance.
    ///
    /// # Arguments
    /// * `language_id` - The numeric ID of the language.
    /// * `items` - An iterator yielding tuples of static string slices representing `(string_code, string_value)`.
    pub fn insert_batch<I>(&mut self, language_id: u16, items: I)
    where
        I: IntoIterator<Item = (&'static str, &'static str)>,
    {
        let iter = items.into_iter();
        let (count, _) = iter.size_hint();

        // Early return: Don't even create an empty map if there's nothing to add.
        if count == 0 {
            return;
        }

        // Optimize allocation
        let string_map = self.locale.entry(language_id).or_default();
        
        // Ensure we fit everything (or at least start with a good chunk)
        string_map.values.reserve(count);

        for (code, value) in iter {
            string_map.values.insert(code.into(), value.into());
        }
    }

    /// Retrieves a cloned set of all string values for a specified language.
    ///
    /// # Arguments
    /// * `language_id` - The numeric ID of the language to retrieve.
    ///
    /// # Returns
    /// * `Some(StringValues)` if the language exists.
    /// * `None` if the language does not exist.
    pub fn get_string_values(&self, language_id: u16) -> Option<StringValues>{
        self.locale.get(&language_id).cloned()
    }

    /// Retrieves a cloned string value for a specific language and string code.
    ///
    /// # Arguments
    /// * `language_id` - The numeric ID of the language.
    /// * `string_code` - The code identifying the desired string.
    ///
    /// # Returns
    /// * `Some(String)` if both the language and string code exist.
    /// * `None` if either the language or string code is missing.
    pub fn get_string_value(&self, language_id: u16, string_code: &str) -> Option<String>{
        // First lookup: Language ID
        let string_map = self.locale.get(&language_id)?;
        string_map.values.get(string_code).cloned()
    }

    /// Retrieves a string value for a specific language and string code, or a default empty string.
    ///
    /// # Arguments
    /// * `language_id` - The numeric ID of the language.
    /// * `string_code` - The code identifying the desired string.
    ///
    /// # Returns
    /// * The string value if found, or an empty `String` if the language or string code is missing.
    pub fn get_string_value_or_default(&self, language_id: u16, string_code: &str) -> String{
        // First lookup: Language ID
        let string_map = match self.locale.get(&language_id){
            Some(sm) => sm,
            None => return String::new(),
        };
        string_map.values.get(string_code).unwrap_or(&String::new()).to_owned()
    }    
}

//************************ */
//UNIT TESTS               */
//************************ */

#[cfg(test)]
mod localizer_tests {
    use super::*;

    #[test]
    fn test_new_locale_is_empty() {
        let locale = Locale::new();
        assert!(locale.get_string_value(1, "hello").is_none());
    }

    #[test]
    fn test_insert_single_entry() {
        let mut locale = Locale::new();
        locale.insert(1, "greeting", "Hello");
        assert_eq!(locale.get_string_value(1, "greeting"), Some("Hello".to_string()));
    }

    #[test]
    fn test_insert_multiple_languages() {
        let mut locale = Locale::new();
        locale.insert(1, "greeting", "Hello");
        locale.insert(2, "greeting", "Bonjour");
        
        assert_eq!(locale.get_string_value(1, "greeting"), Some("Hello".to_string()));
        assert_eq!(locale.get_string_value(2, "greeting"), Some("Bonjour".to_string()));
    }

    #[test]
    fn test_insert_overwrites_existing() {
        let mut locale = Locale::new();
        locale.insert(1, "key", "original");
        locale.insert(1, "key", "updated");
        
        assert_eq!(locale.get_string_value(1, "key"), Some("updated".to_string()));
    }

    #[test]
    fn test_insert_same_key_different_languages() {
        let mut locale = Locale::new();
        locale.insert(1, "farewell", "Goodbye");
        locale.insert(2, "farewell", "Au revoir");
        
        assert_eq!(locale.get_string_value(1, "farewell"), Some("Goodbye".to_string()));
        assert_eq!(locale.get_string_value(2, "farewell"), Some("Au revoir".to_string()));
    }

    #[test]
    fn test_get_string_value_nonexistent_language() {
        let locale = Locale::new();
        assert!(locale.get_string_value(999, "greeting").is_none());
    }

    #[test]
    fn test_get_string_value_nonexistent_key() {
        let mut locale = Locale::new();
        locale.insert(1, "existing", "value");
        assert!(locale.get_string_value(1, "nonexistent").is_none());
    }

    #[test]
    fn test_insert_batch_empty() {
        let mut locale = Locale::new();
        locale.insert_batch(1, std::iter::empty::<(&'static str, &'static str)>());
        // Should not create empty map - verify by checking another language works
        locale.insert(2, "key", "value");
        assert!(locale.get_string_value(1, "key").is_none());
        assert_eq!(locale.get_string_value(2, "key"), Some("value".to_string()));
    }

    #[test]
    fn test_insert_batch_single_item() {
        let mut locale = Locale::new();
        locale.insert_batch(1, vec![("key", "value")]);
        assert_eq!(locale.get_string_value(1, "key"), Some("value".to_string()));
    }

    #[test]
    fn test_insert_batch_multiple_items() {
        let mut locale = Locale::new();
        locale.insert_batch(1, vec![
            ("greeting", "Hello"),
            ("farewell", "Goodbye"),
            ("thanks", "Thank you"),
        ]);
        
        assert_eq!(locale.get_string_value(1, "greeting"), Some("Hello".to_string()));
        assert_eq!(locale.get_string_value(1, "farewell"), Some("Goodbye".to_string()));
        assert_eq!(locale.get_string_value(1, "thanks"), Some("Thank you".to_string()));
    }

    #[test]
    fn test_insert_batch_multiple_languages() {
        let mut locale = Locale::new();
        locale.insert_batch(1, vec![("greeting", "Hello")]);
        locale.insert_batch(2, vec![("greeting", "Bonjour")]);
        
        assert_eq!(locale.get_string_value(1, "greeting"), Some("Hello".to_string()));
        assert_eq!(locale.get_string_value(2, "greeting"), Some("Bonjour".to_string()));
    }

    #[test]
    fn test_insert_batch_overwrites_existing() {
        let mut locale = Locale::new();
        locale.insert(1, "key", "original");
        locale.insert_batch(1, vec![("key", "updated")]);
        assert_eq!(locale.get_string_value(1, "key"), Some("updated".to_string()));
    }

    #[test]
    fn test_insert_batch_mixed_with_single_insert() {
        let mut locale = Locale::new();
        locale.insert(1, "single", "one");
        locale.insert_batch(1, vec![("batch1", "a"), ("batch2", "b")]);
        locale.insert(1, "another", "two");
        
        assert_eq!(locale.get_string_value(1, "single"), Some("one".to_string()));
        assert_eq!(locale.get_string_value(1, "batch1"), Some("a".to_string()));
        assert_eq!(locale.get_string_value(1, "batch2"), Some("b".to_string()));
        assert_eq!(locale.get_string_value(1, "another"), Some("two".to_string()));
    }

    #[test]
    fn test_language_id_zero() {
        let mut locale = Locale::new();
        locale.insert(0, "key", "value");
        assert_eq!(locale.get_string_value(0, "key"), Some("value".to_string()));
    }

    #[test]
    fn test_language_id_max() {
        let mut locale = Locale::new();
        locale.insert(u16::MAX, "key", "value");
        assert_eq!(locale.get_string_value(u16::MAX, "key"), Some("value".to_string()));
    }

    #[test]
    fn test_empty_string_key() {
        let mut locale = Locale::new();
        locale.insert(1, "", "empty_key_value");
        assert_eq!(locale.get_string_value(1, ""), Some("empty_key_value".to_string()));
    }

    #[test]
    fn test_empty_string_value() {
        let mut locale = Locale::new();
        locale.insert(1, "empty_value", "");
        assert_eq!(locale.get_string_value(1, "empty_value"), Some("".to_string()));
    }

    #[test]
    fn test_unicode_strings() {
        let mut locale = Locale::new();
        locale.insert(1, "japanese", "こんにちは");
        locale.insert(2, "japanese", "你好");
        locale.insert(3, "emoji", "🎉🎊");
        
        assert_eq!(locale.get_string_value(1, "japanese"), Some("こんにちは".to_string()));
        assert_eq!(locale.get_string_value(2, "japanese"), Some("你好".to_string()));
        assert_eq!(locale.get_string_value(3, "emoji"), Some("🎉🎊".to_string()));
    }

    #[test]
    fn test_special_characters_in_key() {
        let mut locale = Locale::new();
        locale.insert(1, "key:with:colons", "value1");
        locale.insert(1, "key with spaces", "value2");
        locale.insert(1, "key\nwith\nnewlines", "value3");
        
        assert_eq!(locale.get_string_value(1, "key:with:colons"), Some("value1".to_string()));
        assert_eq!(locale.get_string_value(1, "key with spaces"), Some("value2".to_string()));
        assert_eq!(locale.get_string_value(1, "key\nwith\nnewlines"), Some("value3".to_string()));
    }

    #[test]
    fn test_many_entries_same_language() {
        let mut locale = Locale::new();
        for i in 0..100 {
            let key = format!("key_{}", i);
            let value = format!("value_{}", i);
            locale.insert(1, &key, &value);
        }
        
        for i in 0..100 {
            let key = format!("key_{}", i);
            let expected = format!("value_{}", i);
            assert_eq!(locale.get_string_value(1, &key), Some(expected));
        }
    }

    #[test]
    fn test_multiple_languages_isolation() {
        let mut locale = Locale::new();
        
        // Populate language 1
        for i in 0..50 {
            locale.insert(1, &format!("key_{}", i), &format!("en_{}", i));
        }
        
        // Populate language 2
        for i in 0..50 {
            locale.insert(2, &format!("key_{}", i), &format!("fr_{}", i));
        }
        
        // Verify isolation
        assert_eq!(locale.get_string_value(1, "key_0"), Some("en_0".to_string()));
        assert_eq!(locale.get_string_value(2, "key_0"), Some("fr_0".to_string()));
        assert!(locale.get_string_value(3, "key_0").is_none());
    }

    #[test]
    fn test_insert_batch_from_array() {
        let mut locale = Locale::new();
        let items = [("a", "1"), ("b", "2"), ("c", "3")];
        locale.insert_batch(1, items);
        
        assert_eq!(locale.get_string_value(1, "a"), Some("1".to_string()));
        assert_eq!(locale.get_string_value(1, "b"), Some("2".to_string()));
        assert_eq!(locale.get_string_value(1, "c"), Some("3".to_string()));
    }

    #[test]
    fn test_get_returns_clone_not_reference() {
        let mut locale = Locale::new();
        locale.insert(1, "key", "original");
        
        let mut val = locale.get_string_value(1, "key").unwrap();
        val.push_str("_modified");
        
        // Original should be unchanged
        assert_eq!(locale.get_string_value(1, "key"), Some("original".to_string()));
        assert_eq!(val, "original_modified");
    }
   
}

#[cfg(test)]
mod string_values_tests {
    use super::*;
    const EN_VALUES: [(&str, &str);3] = [
        ("greeting", "Hello"),
        ("farewell", "Goodbye"),
        ("thanks", "Thank you"),
    ];

    fn create_test_string_values() -> StringValues {
        let mut locale = Locale::new();
        locale.insert_batch(1,EN_VALUES);
        /*locale.insert_batch(1, vec![
            ("greeting", "Hello"),
            ("farewell", "Goodbye"),
            ("thanks", "Thank you"),
        ]);*/
        locale.insert(2, "farewell", "Au revoir");
        locale.get_string_values(1).unwrap()
    }

    #[test]
    fn test_get_string_value_found() {
        let sv = create_test_string_values();
        assert_eq!(sv.get_string_value("greeting"), Some("Hello".to_string()));
        assert_eq!(sv.get_string_value("farewell"), Some("Goodbye".to_string()));
    }

    #[test]
    fn test_get_string_value_not_found() {
        let sv = create_test_string_values();
        assert_eq!(sv.get_string_value("missing"), None);
    }

    #[test]
    fn test_get_string_value_empty_key() {
        let sv = create_test_string_values();
        assert_eq!(sv.get_string_value(""), None);
    }

    #[test]
    fn test_get_string_value_or_default_found() {
        let sv = create_test_string_values();
        assert_eq!(sv.get_string_value_or_default("greeting"), "Hello".to_string());
        assert_eq!(sv.get_string_value_or_default("farewell"), "Goodbye".to_string());
    }

    #[test]
    fn test_get_string_value_or_default_not_found() {
        let sv = create_test_string_values();
        assert_eq!(sv.get_string_value_or_default("missing"), String::new());
    }

    #[test]
    fn test_get_string_value_or_default_empty_key() {
        let sv = create_test_string_values();
        assert_eq!(sv.get_string_value_or_default(""), String::new());
    }
}