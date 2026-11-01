use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;

fn normalize_letters(input: &str) -> Result<(char, String), String> {
    let mut letters = input
        .chars()
        .map(|letter| letter.to_ascii_uppercase())
        .collect::<Vec<_>>();

    if letters.len() != 7 {
        return Err("The puzzle letters must be exactly 7 letters long.".to_string());
    }
    if letters.iter().any(|letter| !letter.is_ascii_alphabetic()) {
        return Err("The puzzle letters must contain only alphabetic characters.".to_string());
    }

    letters.sort_unstable();
    letters.dedup();
    if letters.len() != 7 {
        return Err("The puzzle letters must all be distinct.".to_string());
    }

    let mut normalized = input
        .chars()
        .map(|letter| letter.to_ascii_uppercase())
        .collect::<String>();
    normalized.make_ascii_uppercase();

    let center = normalized.chars().next().unwrap();
    Ok((center, normalized))
}

fn solve(reader: impl BufRead, center: char, letters: &str) -> Vec<String> {
    reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|word| word.to_ascii_uppercase())
        .filter(|word| word.len() > 3)
        .filter(|word| word.chars().any(|letter| letter == center))
        .filter(|word| word.chars().all(|letter| letters.contains(letter)))
        .collect()
}

fn run() -> Result<(), String> {
    let mut args = env::args().skip(1);
    let puzzle = args
        .next()
        .ok_or_else(|| "Usage: spelling-bee <7-letter puzzle>".to_string())?;
    if args.next().is_some() {
        return Err("Usage: spelling-bee <7-letter puzzle>".to_string());
    }

    let (center, letters) = normalize_letters(&puzzle)?;

    let file = File::open("words_alpha.txt").map_err(|_| {
        "words_alpha.txt was not found. Download it with:\ncurl -O https://raw.githubusercontent.com/dwyl/english-words/master/words_alpha.txt"
            .to_string()
    })?;

    for word in solve(BufReader::new(file), center, &letters) {
        println!("{word}");
    }

    Ok(())
}

fn main() {
    if let Err(message) = run() {
        eprintln!("{message}");
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn normalizes_letters_and_center() {
        let (center, letters) = normalize_letters("aBcdefg").unwrap();
        assert_eq!(center, 'A');
        assert_eq!(letters, "ABCDEFG");
    }

    #[test]
    fn rejects_repeated_letters() {
        assert!(normalize_letters("AABCDEE").is_err());
    }

    #[test]
    fn solves_only_matching_words() {
        let words = Cursor::new("cat\nbadger\ncab\ngolden\nbead\n");
        let matches = solve(words, 'A', "ABCDEFG");
        assert_eq!(matches, vec!["BEAD"]);
    }
}
