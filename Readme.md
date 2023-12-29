# strongs_dict_hebrew

This library is a rust wrapper for strongs hebrew dictionary. 
json is parsed and deserialized into a `Entry` datatype.
All entries are returned into a `Vec<Entry>`

## example

```
use strongs_dict_hebrew::strongs_hebrew_vec;

let filtered: Vec<_> = strongs_hebrew_vec()
    .into_iter()
    .filter(|e| { e.key == "H8280"})
    .filter(|e| { e.dotless() == "הרש" })
    .filter(|e| { e.lemma() == "הרָשָׂ" })
    .collect();

assert_eq!(filtered.len(), 1);
```

## credit
Json data provided by @openscriptures

https://github.com/openscriptures/strongs
