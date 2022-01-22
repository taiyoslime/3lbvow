use super::constants;
use rand::Rng;

use super::state;
use std::collections::HashMap;

pub fn generate_new_answer() -> String {
    let mut rng = rand::thread_rng();

    let len = constants::WORDLIST.len();
    let index = rng.gen_range(0, len);
    constants::WORDLIST[index].to_string().to_ascii_uppercase()
}

pub fn is_valid_word(word: &str) -> bool {
    let word: &str = &(word.to_ascii_lowercase());
    constants::WORDLIST.binary_search(&word).is_ok()
}

//TODO
pub fn generate_new_alphabets_status() -> state::AlphabetsStatus {
    let mut hm = HashMap::new();

    for ch in b'A'..=b'Z'{
        hm.insert(ch as char, state::AlphaStatus::Unknown);
    }
    hm
}