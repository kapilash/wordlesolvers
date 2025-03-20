use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Side {
    a : char,
    b : char,
    c : char
}

impl Side {
    fn new(a: char, b: char, c: char) -> Self {
        Side { a, b, c }
    }

    // a word is valid if no two consecutive letters belong the same side
    fn is_valid_word(&self, word:&str) -> bool
    {
        if word.len() < 2 {
            return false;
        }
        let mut last_char = word.chars().nth(0).unwrap();
        for c in word.chars().skip(1) {
            if last_char == c {
                return false;
            }
            if last_char == self.a || last_char == self.b || last_char == self.c {
                if c == self.a || c == self.b || c == self.c {
                    return false;
                }
            }
            last_char = c;
        }
        true
    }

    fn has_letter(&self, letter: char) -> bool {
        self.a == letter || self.b == letter || self.c == letter
    }

    fn any_missing_letter(&self, word1: &str, word2: &str) -> bool {
        let mut has_a = false;
        let mut has_b = false;
        let mut has_c = false;
        for c in word1.chars() {
            if c == self.a {
                has_a = true;
            } else if c == self.b {
                has_b = true;
            } else if c == self.c {
                has_c = true;
            }
        }
        for c in word2.chars() {
            if c == self.a {
                has_a = true;
            } else if c == self.b {
                has_b = true;
            } else if c == self.c {
                has_c = true;
            }
        }
        !has_a || !has_b || !has_c
    }
}


// reads a file line by line
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// two words are a valid pair if the last letter of the first word is the same as the first letter
// of the second word and every let
fn valid_word_pair(word1: &str, word2: &str) -> bool {
    let last_char = word1.chars().last().unwrap();
    let first_char = word2.chars().next().unwrap();
    last_char == first_char
}



struct Puzzle {
    sides : Vec<Side>
}

impl Puzzle {
    fn new() -> Self {
        Puzzle { sides: Vec::new() }
    }

    fn add_side(&mut self, side_chars: &str) {
        if side_chars.len() != 3 {
            panic!("Side must have exactly 3 characters");
        }
        let chars: Vec<char> = side_chars.chars().collect();
        let side = Side::new(chars[0], chars[1], chars[2]);
        self.sides.push(side);
    }

    fn is_valid_word(&self, word: &str) -> bool {
        for side in &self.sides {
            if !side.is_valid_word(word) {
                return false;
            }
        }
        for c in word.chars() {
            let mut found = false;
            for side in &self.sides {
                if side.has_letter(c) {
                    found = true;
                    break;
                }
            }
            if !found {
                return false;
            }
        }
        true
    }

    // two words are a valid pair if the last letter of the first word is the same as the first
    // letter and evert letter of the puzzle is in either of the words
    fn valid_word_pair(&self, word1: &str, word2: &str) -> bool {
        if valid_word_pair(word1, word2) {
            if self.sides.iter().any(|side| side.any_missing_letter(word1, word2)) {
                return false;
            }
            return true;
        }
        false
    }

    // print all words from the file that are valid
    fn print_valid_words(&self, filename: &str) {
        let mut words = Vec::new();
        if let Ok(lines) = read_lines(filename) {
            for line in lines {
                if let Ok(word) = line {
                    let word = word.trim().to_uppercase();
                    if word.len() < 3 {
                        continue;
                    }
                    if self.is_valid_word(&word) {
                        words.push(word.clone());
                    }
                }
            }
        } else {
            println!("Error reading file");
        }
        for word1 in words.iter() {
            for word2 in words.iter() {
                if *word1 != *word2 && self.valid_word_pair(word1, word2) {
                    println!("{} {}", word1, word2);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_word() {
        let side = Side::new('A', 'B', 'C');
        assert!(side.is_valid_word("AOCOB"));
        assert!(side.is_valid_word("ADCDE"));
        assert!(!side.is_valid_word("AA"));
        assert!(!side.is_valid_word("AMERAB"));
        assert!(!side.is_valid_word("CDADAB"));
    }

    #[test]
    fn test_puzzle1() {
        let mut puzzle = Puzzle::new();
        puzzle.add_side("MBO");
        puzzle.add_side("VIG");
        puzzle.add_side("DAE");
        puzzle.add_side("SUR");
        assert!(!puzzle.is_valid_word("ABELMOSCHUS"));
    }

}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 6 {
        println!("Usage: {} <side1> <side2> <side3> <side4> <file>", args[0]);
        return;
    }
    let mut puzzle = Puzzle::new();
    for i in 1..5 {
        puzzle.add_side(&args[i]);
    }
    let filename = &args[5];
    puzzle.print_valid_words(filename);

}
