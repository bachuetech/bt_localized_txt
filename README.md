# Project Title
BT LOCALIZED TEXT

## Description
This code is a lightweight, high-performance localization system. It provides a streamlined framework for managing multiple languages, translating string keys, and handling missing translations. It prioritizes fast lookups and minimal memory allocations, making it well-suited for games, embedded systems, or high-throughput applications where performance is critical.

## Usage
```
use crate::languages::Languages;
use crate::localizer::Localizer;

// 1. Initialize the Language Registry
let mut languages = Languages::new();

// Register languages. The returned IDs are used by the Locale.
let english_id = languages.add_language("en", "English");
let spanish_id = languages.add_language("es", "Español");

// Set English as the default fallback language
languages.change_default_language_id(english_id).unwrap();

// 2. Initialize the Locale (String Storage)
let mut locale = Localizer::new();

// Insert strings individually
locale.insert(english_id, "greeting", "Hello, world!");
locale.insert(spanish_id, "greeting", "¡Hola, mundo!");

// Insert strings in batches for better performance
locale.insert_batch(english_id, vec![
    ("farewell", "Goodbye!"),
    ("apple", "Apple")
]);
locale.insert_batch(spanish_id, vec![
    ("farewell", "¡Adiós!"),
    ("apple", "Manzana")
]);

// 3. Retrieve Strings in the Application

// User's browser or system reports a standard locale like "es-ES"
let requested_lang = "es-ES";

// get_lang_id_be automatically falls back:
// 1. Exact match fails
// 2. Beginning match ("es") succeeds
// 3. If both failed, it would return the default language ID (English)
let active_lang_id = languages.get_lang_id_be(requested_lang);

// Fetch the translated string
let greeting = locale.get_string_value_or_default(active_lang_id, "greeting");
let missing_key = locale.get_string_value_or_default(active_lang_id, "non_existent_key");

println!("Language: {}", languages.get_lang_name(active_lang_id)); // Prints: Español
println!("Greeting: {}", greeting);                                // Prints: ¡Hola, mundo!
println!("Missing Key Value: '{}'", missing_key);                   // Prints: '' (empty string fallback)
```

## Version History
* 0.1.0
    * Initial Release
* 0.1.1
    * New function get_list_of_languages to get the list of available Languages as Vec<(u16, String)> (id,language)
* 0.1.2
    * New functionality to read translations from a TOML.
* 0.2.0
    * Change the name from Locale to Localizer. 
    * Add a function (get_list_of_languages) to the Translator to retrieve all languages. 
    * Add documentation
* 0.2.1
    * Add the function add_translation_with_lang_id to add translations using the language ID.

## License
GPL-3.0-only