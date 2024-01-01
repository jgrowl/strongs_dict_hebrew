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
pub mod parse;

use std::{fs, sync::Mutex, collections::HashMap};

use once_cell::sync::Lazy;

pub use crate::types::*;
pub use crate::entry::Entry;


// Top level access point for pre-parsed and higher level type entries
// This uses statically embedded JSON Data and statically embedded
// parsed types. A clone of the dictionary is provided to avoid
// having to lock and unwrap. If you don't want clone then use 
// `STRONGS_HEB_DICT_VEC` directly.
pub fn strongs_heb_dict_vec() -> Vec<Entry> {
    STRONGS_HEB_DICT_VEC.lock().unwrap().to_vec()
}

// Top level access point if you don't want to clone
pub static STRONGS_HEB_DICT_VEC: Lazy<Mutex<Vec<Entry>>> 
= Lazy::new(|| { 
    let m = crate::parse::parsed_strongs_hebrew_vec(); 
    Mutex::new(m)
});

// TODO: Provide easier way to access Raw type if user wants it


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_strongs_hebrew_vec_by_key() {
        let filtered: Vec<_> = strongs_heb_dict_vec()
            .into_iter()
            .filter(|e| { e.key == "H8280"})
            .collect();

        assert_eq!(filtered.len(), 1);

        let h8280 = filtered.first().unwrap();
        assert_eq!(h8280.strongs_def(), "to prevail");
    }

    #[test]
    fn test_filter_strongs_hebrew_vec_by_lemma_without_dots() {
        let filtered: Vec<_> = strongs_heb_dict_vec()
            .into_iter()
            .filter(|e| { e.dotless() == "הרש" })
            .collect();
        assert_eq!(6, filtered.len())
    }

    #[test]
    fn test_filter_strongs_hebrew_vec_by_lemma_with_dots() {
        let filtered: Vec<_> = strongs_heb_dict_vec()
            .into_iter()
            .filter(|e| { e.lemma() == "הרָשָׂ" })
            .collect();
        assert_eq!(3, filtered.len())
    }
}

