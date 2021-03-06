extern crate colored;
extern crate rand;

use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use colored::*;
use rand::prelude::*;

/// a set for alphabetical characters. Considering the nature of the problem, we can use an
/// unsigned integer to represent the set.
struct CharSet{
    /// the set of characters
    chars: u32,
}

impl CharSet {
    fn new() -> CharSet {
        CharSet { chars: 0 }
    }

    fn new_full() -> CharSet {
        CharSet { chars: 0xFFFFFFFF }
    }

    fn add(&mut self, c: char) {
        self.chars |= 1 << (c as u32 - 'a' as u32);
    }

    fn contains(&self, c: char) -> bool {
        (self.chars & (1 << (c as u32 - 'a' as u32))) != 0
    }

    fn remove(&mut self, c: char) {
        self.chars &= !(1 << (c as u32 - 'a' as u32));
    }

    fn is_subset_of(&self, other: &CharSet) -> bool {
        (self.chars & other.chars) == self.chars
    }

    fn remove_others(&mut self, c: char) {
        self.chars = 1 << (c as u32 - 'a' as u32);
    }

    /*fn cardinality(&self) -> usize {
        self.chars.count_ones() as usize
    }*/
}

impl fmt::Display for CharSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut chars = vec![];
        for i in 0..26 {
            if self.chars & (1 << i) != 0 {
                chars.push((i + 'a' as u8)  as char);
            }
        }
        write!(f, "<{}>", chars.iter().collect::<String>())
    }
}

/// the word consists of five characters.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Word {
    /// the first character
    c0 : char,
    /// the second character
    c1 : char,
    /// the third character
    c2 : char,
    /// the fourth character
    c3 : char,
    /// the fifth character
    c4 : char
}

impl Word {
    fn new(str : &str) -> Word {
        Word {
            c0 : str.chars().nth(0).unwrap(),
            c1 : str.chars().nth(1).unwrap(),
            c2 : str.chars().nth(2).unwrap(),
            c3 : str.chars().nth(3).unwrap(),
            c4 : str.chars().nth(4).unwrap()
        }
    }

    fn to_string(&self) -> String {
        format!("{}{}{}{}{}", self.c0, self.c1, self.c2, self.c3, self.c4)
    }

    fn has_repeated_chars(&self) -> bool {
        self.c0 == self.c1 || self.c0 == self.c2 || self.c0 == self.c3 || self.c0 == self.c4 ||
        self.c1 == self.c2 || self.c1 == self.c3 || self.c1 == self.c4 ||
        self.c2 == self.c3 || self.c2 == self.c4 ||
        self.c3 == self.c4
    }

    fn score(&self, scores: &Vec<usize>) -> usize {
       let raw_score =  scores[self.c0 as usize - 'a' as usize] +
           scores[self.c1 as usize - 'a' as usize] + 
           scores[self.c2 as usize - 'a' as usize] +
           scores[self.c3 as usize - 'a' as usize] +
           scores[self.c4 as usize - 'a' as usize];
         if self.has_repeated_chars() {
            raw_score - 20
         } else {
            raw_score
         }
    }

    fn to_char_set(&self) -> CharSet {
        let mut char_set = CharSet::new();
        char_set.add(self.c0);
        char_set.add(self.c1);
        char_set.add(self.c2);
        char_set.add(self.c3);
        char_set.add(self.c4);
        char_set
    }
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

/// The constraints on the words. Each constraint is represented as a set of characters.
struct WordleState {
    /// the characters allowed in the first position.
    c0 : CharSet,
    /// the characters allowed in the second position.
    c1 : CharSet,
    /// the characters allowed in the third position.
    c2 : CharSet,
    /// the characters allowed in the fourth position.
    c3 : CharSet,
    /// the characters allowed in the fifth position.
    c4 : CharSet,
    /// the characters known to be in the target word.
    in_target: CharSet
}

impl WordleState {
    fn new() -> WordleState {
        WordleState {
            c0: CharSet::new_full(),
            c1: CharSet::new_full(),
            c2: CharSet::new_full(),
            c3: CharSet::new_full(),
            c4: CharSet::new_full(),
            in_target: CharSet::new()
        }
    }

    fn is_allowed(&self, word: &Word) -> bool {
        self.in_target.is_subset_of(&word.to_char_set()) &&
            self.c0.contains(word.c0) && 
            self.c1.contains(word.c1) && 
            self.c2.contains(word.c2) &&
            self.c3.contains(word.c3) &&
            self.c4.contains(word.c4)
    }

    /// gets called when we encounter an N letter. That is, the letter in the guess that is neither green
    /// nor yellow.
    fn remove_letter(&mut self, c: char) {
        self.c0.remove(c);
        self.c1.remove(c);
        self.c2.remove(c);
        self.c3.remove(c);
        self.c4.remove(c);
    }


    /// once we got a new word and the response from wordle (in terms of N/Y/G for each letter of
    /// the guess), update the state.
    fn update(&mut self, word: &Word, response: &str) {
        if response.chars().nth(0).unwrap() != 'N' {
            self.in_target.add(word.c0);
            if response.chars().nth(0).unwrap() == 'Y' {
                self.c0.remove(word.c0);
            }
            else {
                self.c0.remove_others(word.c0);
            }
        }
        else {
            self.remove_letter(word.c0);
        }
        if response.chars().nth(1).unwrap() != 'N' {
            self.in_target.add(word.c1);
            if response.chars().nth(1).unwrap() == 'Y' {
                self.c1.remove(word.c1);
            }
            else {
                self.c1.remove_others(word.c1);
            }
        }
        else {
            self.remove_letter(word.c1);
        }
        if response.chars().nth(2).unwrap() != 'N' {
            self.in_target.add(word.c2);
            if response.chars().nth(2).unwrap() == 'Y' {
                self.c2.remove(word.c2);
            }
            else {
                self.c2.remove_others(word.c2);
            }
        }
        else {
            self.remove_letter(word.c2);
        }
        if response.chars().nth(3).unwrap() != 'N' {
            self.in_target.add(word.c3);
            if response.chars().nth(3).unwrap() == 'Y' {
                self.c3.remove(word.c3);
            }
            else {
                self.c3.remove_others(word.c3);
            }
        }
        else {
            self.remove_letter(word.c3);
        }
        if response.chars().nth(4).unwrap() != 'N' {
            self.in_target.add(word.c4);
            if response.chars().nth(4).unwrap() == 'Y' {
                self.c4.remove(word.c4);
            }
            else {
                self.c4.remove_others(word.c4);
            }
        }
        else {
            self.remove_letter(word.c4);
        }

    }

    fn is_usable(&self, c: char) -> bool {
        self.c0.contains(c) || self.c1.contains(c) || self.c2.contains(c) || self.c3.contains(c) || self.c4.contains(c)
    }
}

/// a pair of character and the number of times it appears in the word list.
#[derive(Debug)]
struct CharCount {
    c : u32,
    count : u32
}


/// the allowed collection of words.
struct WordCollection {
    words: Vec<Word>,
}

impl WordCollection {
    /// reads five lettered words from the file.
    fn new(filename: &str) -> WordCollection {
        let mut words = Vec::new();
        let path = Path::new(filename);

        let lines = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", path.display(),  why),
            Ok(file) => io::BufReader::new(file).lines(),
        };


        for line in lines {
            if let Ok(x) = line  {
                if x.len() == 5 {
                    words.push(Word::new(&x));
                }
            }
        }

        WordCollection {
            words: words
        }
    }

    /// returns a random word from the collection.
    /// Useful for playing wordle/cows and bulls.
    fn get_random_word(&self) -> String {
        let mut rng = thread_rng();
        let index = rng.gen_range(0..self.words.len());
        self.words[index].to_string()
    }

    /// returns true if the given word is in the collection.
    fn contains_word(&self, word: &str) -> bool {
        for w in &self.words {
            if w.to_string() == word.to_lowercase() {
                return true;
            }
        }
        false
    }

    fn from_words(words: Vec<Word>) -> WordCollection {
        WordCollection {
            words: words
        }
    }

    /// returns the best guess for the next word for wordle.
    fn get_best_word(&self) -> Word {
        if self.words.len() == 0 {
            println!("{}", "I give up".red());
            return Word::new("     ");
        }
        let mut vec = Vec::new();
        for c in 0..26 {
            vec.push(CharCount {
                c: c as u32,
                count: 0
            });
        }
        for word in &self.words {
            for c in word.to_string().chars() {
                vec[c as usize - 'a' as usize].count += 1;
            }
        }
        vec.sort_by(|a, b| a.count.cmp(&b.count));
        let mut scores = Vec::new();
        for c in 0..26 {
            let i = vec.iter().position(|x| x.c == c as u32).unwrap();
            scores.push(i);
        }
        let mut best_word = self.words[0];
        let mut score = best_word.score(&scores);
        for i in 1..self.words.len() {
            let word = self.words[i];
            let new_score = word.score(&scores);
            if new_score > score {
                best_word = word;
                score = new_score;
            }
        }
        best_word
    }

    /// returns a new collection with only those words that satisfy the current state.
    /// This is called after the state has been updated by the wordle response.
    fn filter(&self, state: &WordleState) -> WordCollection {
        let mut words = Vec::new();
        for word in &self.words {
            if state.is_allowed(&word) {
                words.push(word.clone());
            }
        }
        WordCollection::from_words(words)
    }

    fn remove(&mut self, word: &Word) {
        self.words.retain(|x| x != word);
    }
}

/// downloads sgb word file from Knuth's site.
fn download_file_if_needed(url: &str, filename: &str) {
    let path = Path::new(filename);
    if !path.exists() {
        let mut file = File::create(filename).unwrap();
        let mut response = reqwest::blocking::get(url).unwrap();
        io::copy(&mut response, &mut file).unwrap();
    }
}

/// tries to solve the wordle.
fn solve_wordle() {
    let mut collection = WordCollection::new("sgb-words.txt");
    let mut word = collection.get_best_word();
    let mut state = WordleState::new();
    while collection.words.len() > 0 {
        println!("{}", word.to_string().to_uppercase().green().bold());
        let mut response = String::new();
        io::stdin().read_line(&mut response).unwrap();
        let response = response.trim().to_uppercase();
        state.update(&word, &response);
        collection = collection.filter(&state);
        word = collection.get_best_word();
        if response == "GGGGG" {
            println!("thank you!");
            break;
        }
    }
}
/// tries to solve the wordle in soft-mode.
fn solve_wordle_soft_mode() {
    let adieu = Word::new("adieu");
    let pylon = Word::new("pylon");
    let crows = Word::new("crows");
    let fight = Word::new("fight");
    let mut collection = WordCollection::new("sgb-words.txt");
    let mut state = WordleState::new();
    println!("{}", adieu.to_string().to_uppercase().green().bold());
    let response = read_response(&collection, &state);
    state.update(&adieu, &response);
    if response == "GGGGG" {
        println!("thank you!");
        return;
    }
    let mut known_count = 0;
    for c in response.chars() {
        if c != 'N' {
            known_count += 1;
        }
    }
    collection = collection.filter(&state);
    if known_count < 4 {
        println!("{}", pylon.to_string().to_uppercase().green().bold());
        let response = read_response(&collection, &state);
        state.update(&pylon, &response);
        if response == "GGGGG" {
            println!("thank you!");
            return;
        }
        for c in response.chars() {
            if c != 'N' {
                known_count += 1;
            }
        }
        collection = collection.filter(&state);
    }
    if known_count < 4 {
        println!("{}", crows.to_string().to_uppercase().green().bold());
        let response = read_response(&collection, &state);
        state.update(&crows, &response);
        if response == "GGGGG" {
            println!("thank you!");
            return;
        }
        for c in response.chars() {
            if c != 'N' {
                known_count += 1;
            }
        }
        collection = collection.filter(&state);
    }
    if known_count < 4 {
        println!("{}", fight.to_string().to_uppercase().green().bold());
        let response = read_response(&collection, &state);
        state.update(&fight, &response);
        if response == "GGGGG" {
            println!("thank you!");
            return;
        }
        collection = collection.filter(&state);
    }
    let mut word = collection.get_best_word();
    while collection.words.len() > 0 {
        println!("{}", word.to_string().to_uppercase().green().bold());
        let response = read_response(&collection, &state);
        if response == "GGGGG" {
            println!("thank you!");
            break;
        }
        state.update(&word, &response);
        collection = collection.filter(&state);
        word = collection.get_best_word();
    }
}

fn read_response(words: &WordCollection, state: &WordleState) -> String {
    loop {
        let mut response = String::new();
        io::stdin().read_line(&mut response).unwrap();
        let x = response.trim().to_uppercase();
        if x.len() == 5 {
            return x;
        }
        else if x == "?C" {
            println!("{} words", words.words.len());
        }
        else if x == "?L" {
            for word in &words.words {
                print!("{} ", word.to_string());
            }
            println!("");
        }
        else if x == "?H" {
            let alphabet = "abcsdefghijklmnopqrstuvwxyz";
            for c in alphabet.chars() {
                if state.is_usable(c) {
                    print!("{} ", c);
                }
            }
            println!("");
        }
        else {
            println!("{}", "please enter five letters".red());
        }
    }
}

fn solve_worlde_with(first_word: &str) {
    let mut collection = WordCollection::new("sgb-words.txt");
    let mut word = Word::new(first_word);
    let mut state = WordleState::new();
    while collection.words.len() > 0 {
        println!("{}", word.to_string().to_uppercase().green().bold());
        let response = read_response(&collection, &state); //response.trim().to_uppercase();
        state.update(&word, &response);
        collection = collection.filter(&state);
        word = collection.get_best_word();
        if response == "GGGGG" {
            println!("thank you!");
            break;
        }
    }
}

/*
fn next_pos_and_update(given: &str, c: char, visited:&mut Vec<bool>) -> (usize, bool) {
    let mut pos = given.len();
    let mut found = false;
    for i in 0..given.len() {
        if visited[i] {
            continue;
        }
        if given.chars().nth(i).unwrap() == c {
            pos = i;
            found = true;
            break;
        }
    }
    if found {
        visited[pos] = true;
    }
    (pos, found)
}*/

fn not_found_in(given: &str, c: char) -> bool {
    for i in 0..given.len() {
        if given.chars().nth(i).unwrap() == c {
            return false;
        }
    }
    true
}

fn wordle_compare(given: &str, word: &str) -> String {
    let mut chars = Vec::new();
    let mut visited = Vec::new();
    for _ in word.chars() {
        chars.push('.');
        visited.push(false);
    }
    for i in 0..word.len() {
        let c = word.chars().nth(i).unwrap();
        if not_found_in(given, c) {
            chars[i] = 'N';
        }
    }
    for i in 0..word.len() {
        let c = word.chars().nth(i).unwrap();
        if given.chars().nth(i).unwrap() ==  c {
            chars[i] = 'G';
            visited[i] = true;
        }
    }
    for i in 0..word.len() {
        if chars[i] == '.' {
            let mut found = false;
            let x = word.chars().nth(i).unwrap();
            for j in 0..word.len() {
                if  i == j || visited[j] {
                    continue;
                }
                let c = given.chars().nth(j).unwrap();
                if c == x  {
                    found = true;
                    visited[j] = true;
                    break;
                }
            }
            if found {
                chars[i] = 'Y';
            }
            else {
                chars[i] = 'N';
            }
        }
    }
    let mut response = String::new();
    for c in chars {
        response.push(c);
    }
    response
}
/// helper function to display the response for a guess in Cows and Bulls.
fn compare_print_cb(given: &str, response: &str) {
    let mut cows = 0;
    let mut bulls = 0;
    for i in 0..5 {
        let r = response.chars().nth(i).unwrap();
        if r == given.chars().nth(i).unwrap() {
            bulls += 1;
        }
        else {
            let mut found = false;
            for j in 0..5 {
                if i == j {
                    continue;
                }
                if given.chars().nth(j).unwrap() == r {
                    found = true;
                    break;
                }
            }
            if found {
                cows += 1;
            }
        }
    }
    println!("{} bulls and {} cows", bulls, cows);
}

/// helper function to display the response for a guess in Wordle.
fn compare_print_wordle(given: &str, response: &str) {
    let text = wordle_compare(given, response);
    println!("{}", text.bold());
}

/// to play wordle.
fn play_wordle() {
    println!("{}", "This is a cheap knock off to the excellent https://powerlanguage.co.uk/wordle/ ".green().bold());
    println!("{}", "Consider playing there.");
    println!();
    println!("enter your word");
    let collection = WordCollection::new("sgb-words.txt");
    let word = collection.get_random_word();
    let mut count : u32 = 0;
    while count < 6 {
        let mut response = String::new();
        io::stdin().read_line(&mut response).unwrap();
        let response = response.trim().to_lowercase();
        if response == "quit" {
            break;
        }
        if response.len() != 5 {
            println!("{}", "Please enter a 5 letter word".red());
            continue;
        }
        if !collection.contains_word(&response) {
            println!("{}", "Nope! there ain't no such word.".red());
            continue;
        }
        if response == word {
            println!("{} indeed", response.green());
            break;
        }
        else {
            compare_print_wordle(&word, &response);
        }
        count = count + 1;
    }
    if count > 5 {
        println!("{} {}", "Loser!".red(), word.to_string().blue());
    }
}

fn find_shape(final_word: &str, shape: Vec<String>) {
    let mut collection = WordCollection::new("sgb-words.txt");
    
    for target in shape {
        let mut maybe_word : Option<Word> = None;
        for word in &collection.words {
            if wordle_compare( final_word, &word.to_string()) == target {
                println!("{} {}", target, word.to_string());
                maybe_word = Some(word.clone());
                break;
            }
        }
        if maybe_word.is_none() {
            println!("no word matching {}", target);
        }
        else {
            let word = maybe_word.unwrap();
            collection.remove(&word);
        }
    }
}

/// to play Cows and Bulls.
fn play_cows_and_bulls() {
    println!("{}", "Welcome to cows and bulls! ".green().bold());
    println!();
    println!("enter your word");
    let collection = WordCollection::new("sgb-words.txt");
    let word = collection.get_random_word();
    let mut count : u32 = 0;
    while count < 20 {
        let mut response = String::new();
        io::stdin().read_line(&mut response).unwrap();
        let response = response.trim().to_lowercase();
        if response == "quit" {
            break;
        }
        if response.len() != 5 {
            println!("{}", "Please enter a 5 letter word".red());
            continue;
        }
        if !collection.contains_word(&response) {
            println!("{}", "Chor! there ain't no such word.".red());
            continue;
        }
        if response == word {
            println!("{} indeed", response.green());
            break;
        }
        else {
            compare_print_cb(&word, &response);
        }
        count = count + 1;
    }
    if count > 20 {
        println!("{} {}", "Loser!".red(), word.to_string().blue());
    }
}

fn main() {
    download_file_if_needed("https://www-cs-faculty.stanford.edu/~knuth/sgb-words.txt", "sgb-words.txt");
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        solve_wordle();
    }
    else if args[1] == "wordle" {
        play_wordle();
    }
    else if args[1] == "soft" {
        solve_wordle_soft_mode(); 
    }
    else if args[1] == "cowsandbulls" {
        play_cows_and_bulls();
    }
    else if args[1] == "cb" {
        play_cows_and_bulls();
    }
    else if args[1].len() == 5 {
        solve_worlde_with(&args[1]);
    }
    else if args.len() > 3 {
        if args[1] == "shapes" {
            find_shape(&args[2], args[3..].to_vec());
        }
    }
    else {
        println!("{}", "please enter five letters".red());
    }
}

#[test]
fn test_wordle_compare() {
    assert_eq!(wordle_compare("hello", "henlo"), "GGNGG");
    assert_eq!(wordle_compare("hello", "hello"), "GGGGG");
    assert_eq!(wordle_compare("hello", "olleh"), "YYGYY");
    assert_eq!(wordle_compare("hello", "ollen"), "YYGYN");
    assert_eq!(wordle_compare("hello", "lllen"), "YNGYN");
    assert_eq!(wordle_compare("hello", "lllle"), "NNGGY");
}

