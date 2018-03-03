use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

fn sorted_string(s: &str) -> String {
    let mut s = s.chars().collect::<Vec<_>>();
    s.sort();
    s.into_iter().collect::<String>()
}

struct Anagram(HashMap<String, Vec<String>>);

impl Anagram {
    fn new<P: AsRef<Path>>(dict_file: P) -> Result<Self, io::Error> {
        let file = File::open(dict_file)?;
        let file = io::BufReader::new(file);
        let mut anagram = Anagram(HashMap::new());
        for line in file.lines() {
            let word = line?;
            anagram.add_word(word);
        }
        Ok(anagram)
    }

    fn add_word(&mut self, word: String) {
        let sorted = sorted_string(&word);
        self.0.entry(sorted).or_insert(Vec::new()).push(word);
    }

    fn find(&self, word: &str) -> Option<&Vec<String>> {
        let word = sorted_string(word);
        self.0.get(&word)
    }
}

fn main() {
    let word = std::env::args().nth(1).expect("USAGE: word");
    let table = Anagram::new("/usr/share/dict/words").expect("failed to make table");

    println!("{:?}", table.find(&word));
}
