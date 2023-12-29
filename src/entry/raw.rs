use std::fmt;

use serde::Deserialize;
use unicode_segmentation::*;

// This raw type is meant to pull in strongs json file as simply as 
// possible. I want to keep it as close to source data as possible
#[derive(Clone, Deserialize)]
pub struct RawEntry {
    pub lemma: String,
    pub xlit: String,
    pub pron: String,
    pub derivation: Option<String>,
    pub strongs_def: String,
    pub kjv_def: Option<String>,
}

impl RawEntry {

    // The lemma is read in as a hebrew right to left string
    // When deserialized though it sets it to left to right
    // The diacritical marks are on the right base characters, but
    // the characters are left to right 
    pub fn lemma_left_to_right(&self) -> String { 
        self.lemma.to_owned()
    }

    // This will return the right to left string with diacritical marks
    // in the right places
    pub fn lemma_right_to_left(&self) -> String {
        let lemma = self.lemma_left_to_right();
        let graphemes = lemma.graphemes(true);
        graphemes.rev().collect()
    }

    pub fn xlit(&self) -> String { self.xlit.clone() }
    pub fn pron(&self) -> String { self.pron.clone() }
    pub fn derivation(&self) -> Option<String> { self.derivation.clone() }
    pub fn strongs_def(&self) -> String { self.strongs_def.clone() }
    pub fn kjv_def(&self) -> Option<String> { self.kjv_def.clone() }

    pub fn dotless(&self) -> String {
        let lemma = self.lemma_right_to_left();

        // Using this library as it seems like it would be comprehensive
        // in removing any hebrew diacriticals since it uses a character
        // range removal technique. 
        //
        niqqud::remove_thorough(&lemma).to_string()
    }

}

impl fmt::Debug for RawEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RawEntry")
            .field("dotless", &self.dotless())
            .field("lemma_right_to_left", &self.lemma_right_to_left())
            .field("lemma_left_to_right", &self.lemma_left_to_right())
            .field("xlit", &self.xlit())
            .field("pron", &self.pron())
            .field("derivation", &self.derivation())
            .field("strongs_def", &self.strongs_def())
            .field("kjv_def", &self.kjv_def())
            .finish()
    }
}


