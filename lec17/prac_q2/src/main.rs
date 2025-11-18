use std::fs;
use std::env;
use std::io::{self, BufRead};
use std::error::Error;
use std::collections::HashMap;

// NOTE: You *may not* change the names or types of the members of this struct.
//       You may only add lifetime-relevant syntax.
pub struct SearchResult {
    pub matches: Vec<&str>,
    pub contains: &str
}

/// Returns a [`SearchResult`] struct, where the matches vec is
/// a vector of every sentence that contains `contains`.
///
/// A sentence is defined as a slice of an `&str` which is the first
/// character of the string, or the first non-space character after
/// a full-stop (`.`), all the way until the last non-space character
/// before a full-stop or the end of the string.
///
/// For example, In the string "Hello. I am Tom . Goodbye", the three
/// sentences are "Hello", "I am Tom" and "Goodbye"
fn find_sentences_containing(text: &str, contains: &str) -> SearchResult {
    todo!()
}

/// Given a vec of [`SearchResult`]s, return a hashmap, which lists how many
/// time each sentence occured in the search results.
fn count_sentence_matches(searches: Vec<SearchResult>) -> HashMap<&str, i32> {
    todo!()
}


/////////// DO NOT CHANGE BELOW HERE ///////////

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let text = fs::read_to_string(file_path)?;

    let mut sentence_matches = {
        let mut found = vec![];

        let stdin = io::stdin();
        let matches = stdin.lock().lines().map(|l| l.unwrap()).collect::<Vec<_>>();
        for line in matches.iter() {
            let search_result = find_sentences_containing(&text, line);
            println!("Found {} results for '{}'.", search_result.matches.len(), search_result.contains);
            found.push(search_result);
        }

        count_sentence_matches(found).into_iter().collect::<Vec<_>>()
    };
    sentence_matches.sort();

    for (key, value) in sentence_matches {
        println!("'{}' occured {} times.", key, value);
    }

    Ok(())
}
