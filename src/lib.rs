#![cfg_attr(
    debug_assertions, 
    allow(
        dead_code, 
        unused_imports, 
        non_snake_case, 
        unused_variables,
        unused_mut,
        non_camel_case_types,
        confusable_idents,
        uncommon_codepoints,
    )
)]
pub mod types;
pub mod entry;

use std::fs;

pub use types::*;
pub use entry::Entry;

// Main entry point using a slightly higher-level Entry datatype
pub fn strongs_hebrew_vec() -> Vec<Entry> {
    parse_strongs_hebrew()
        .iter()
        .map(|(k,v)| {
            Entry::new(k.clone(), v.clone())
        })
        .collect()
}

// Deserializes json string into a raw entry data type
// This raw type is meant to be as close to the actual data in
// the json as possible.
pub fn parse_strongs_hebrew() -> StrongsRawMap {
    let data = strongs_hebrew_string();

    use serde_json::from_str;
    let deserialized: StrongsRawMap = from_str(&data).unwrap();
    deserialized
}

// Reads json from file into a string
pub fn strongs_hebrew_string() -> String {
    let root = std::env::current_dir().unwrap().display().to_string();
    let file_path = format!("{}/strongs-hebrew-dictionary.json", root);

    let data = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    data
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_strongs_hebrew_vec_by_key() {
        let filtered: Vec<_> = strongs_hebrew_vec()
            .into_iter()
            .filter(|e| { e.key == "H8280"})
            .collect();

        assert_eq!(filtered.len(), 1);

        let h8280 = filtered.first().unwrap();
        assert_eq!(h8280.strongs_def(), "to prevail");
    }

    #[test]
    fn test_filter_strongs_hebrew_vec_by_lemma_without_dots() {
        let filtered: Vec<_> = strongs_hebrew_vec()
            .into_iter()
            .filter(|e| { e.dotless() == "הרש" })
            .collect();
        assert_eq!(6, filtered.len())
    }

    #[test]
    fn test_filter_strongs_hebrew_vec_by_lemma_with_dots() {
        let filtered: Vec<_> = strongs_hebrew_vec()
            .into_iter()
            .filter(|e| { e.lemma() == "הרָשָׂ" })
            .collect();
        assert_eq!(3, filtered.len())
    }

    #[test]
    fn test_parse_strongs_hebrew_lookup() {
        let key = "H8280";
        let strongs = parse_strongs_hebrew();
        let s = strongs.get(key).unwrap();
        let strongs_def = &s.strongs_def;
        assert_eq!(strongs_def, "to prevail");
    }
}

