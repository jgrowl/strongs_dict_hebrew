use std::{fs, sync::Mutex, collections::HashMap};

use once_cell::sync::Lazy;
use crate::Entry;
use crate::types::*;

// This returns the crate embedded, pre-parsed Raw map
pub static STRONGS_HEB_DICT_RAW_MAP: Lazy<Mutex<StrongsRawMap>> 
= Lazy::new(|| { 
    let m = parse_strongs_hebrew(); 
    Mutex::new(m)
});

// Strongs Hebrew JSON data as a String bundled into crate
const STRONGS_HEBREW_DICTIONARY_DATA: &str = include_str!(
    "strongs-hebrew-dictionary.json"
);

// Parses the embedded Strong's Hebrew JSON data and returns
// higher level `Vec<Entry>`
pub fn parsed_strongs_hebrew_vec() -> Vec<Entry> {
    STRONGS_HEB_DICT_RAW_MAP.lock().unwrap()
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
    let data = STRONGS_HEBREW_DICTIONARY_DATA;

    use serde_json::from_str;
    let deserialized: StrongsRawMap = from_str(&data).unwrap();
    deserialized
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_strongs_hebrew_lookup() {
        let strongs = parse_strongs_hebrew();
        let key = "H8280";
        let s = strongs.get(key).unwrap();
        let strongs_def = &s.strongs_def;
        assert_eq!(strongs_def, "to prevail");
    }
}

