pub mod raw;

use std::fmt;

use serde::Deserialize;
use unicode_segmentation::*;

use crate::entry::raw::RawEntry;

#[derive(Clone, Deserialize)]
pub struct Entry {
    pub key: String,
    pub raw: RawEntry,
}

impl Entry {
    pub fn new(key: String, raw: RawEntry) -> Self {
        Self { key, raw }
    }

    pub fn key(&self) -> String { self.key.clone() }

    pub fn lemma(&self) -> String { self.lemma_right_to_left() }

    // WARNING
    // I do not know if this sidedness depends on the platform that
    // it is run.
    // TODO: Test to see what deserialization does on a right to left
    // platform. Does it always flip it around the same way?
    pub fn lemma_left_to_right(&self) -> String { 
        self.raw.lemma_left_to_right()
    }

    pub fn lemma_right_to_left(&self) -> String { 
        self.raw.lemma_right_to_left()
    }

    pub fn xlit(&self) -> String { self.raw.xlit.clone() }
    pub fn pron(&self) -> String { self.raw.pron.clone() }
    pub fn derivation(&self) -> Option<String> { 
        self.raw.derivation.clone() 
    }
    pub fn strongs_def(&self) -> String { self.raw.strongs_def.clone() }
    pub fn kjv_def(&self) -> Option<String> { self.raw.kjv_def.clone() }

    pub fn dotless(&self) -> String { self.raw.dotless() }
}

impl fmt::Debug for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Entry")
            .field("key", &self.key())
            .field("raw", &self.raw)
            .finish()
    }
}

