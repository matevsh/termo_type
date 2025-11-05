use anyhow::{Context, Result};
use rand::seq::SliceRandom;
use std::fs;
use std::path::Path;

/// Hardcoded fallback list of Polish words
const FALLBACK_WORDS: &[&str] = &[
    "ale", "bez", "być", "czy", "dla", "dom", "gdy", "już", "jak", "jego",
    "jest", "jeden", "jeszcze", "która", "może", "który", "mieć", "nasz", "nie", "najpierw",
    "oraz", "pierwszy", "pod", "przez", "przy", "ponieważ", "który", "się", "swój", "tak",
    "tam", "ten", "teraz", "tylko", "właśnie", "bardzo", "gdzie", "jestem", "można", "musieć",
    "nowy", "podczas", "ponad", "przed", "również", "rzecz", "sposób", "według", "wiele", "właśnie",
    "zawsze", "ziemia", "życie", "świat", "czas", "człowiek", "praca", "system", "grupa", "problem",
    "program", "firma", "produkt", "projekt", "funkcja", "metoda", "wynik", "proces", "przykład", "część",
    "miejsce", "sprawy", "strona", "forma", "droga", "środek", "przypadek", "liczba", "wartość", "stopień",
    "różny", "ostatni", "duży", "mały", "wielki", "nowy", "stary", "dobry", "zły", "czarny",
    "biały", "długi", "krótki", "wysoki", "niski", "szeroki", "wąski", "głęboki", "płytki", "ciężki",
];

/// Load words from a JSON file
/// Returns a vector of words
pub fn load_words_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<String>> {
    let content = fs::read_to_string(path)
        .context("Failed to read words file")?;

    let words: Vec<String> = serde_json::from_str(&content)
        .context("Failed to parse JSON")?;

    if words.is_empty() {
        anyhow::bail!("Words file is empty");
    }

    Ok(words)
}

/// Load words with fallback to hardcoded list
/// First tries to load from the specified file, falls back to FALLBACK_WORDS if it fails
pub fn load_words<P: AsRef<Path>>(path: P) -> Vec<String> {
    load_words_from_file(path).unwrap_or_else(|_| {
        // Fallback to hardcoded list
        FALLBACK_WORDS
            .iter()
            .map(|s| s.to_string())
            .collect()
    })
}

/// Generate a sequence of random words for the test
/// `count` - number of words to generate
/// `words` - source word list
pub fn generate_word_sequence(count: usize, words: &[String]) -> Vec<String> {
    let mut rng = rand::thread_rng();
    let mut sequence = Vec::with_capacity(count);

    for _ in 0..count {
        if let Some(word) = words.choose(&mut rng) {
            sequence.push(word.clone());
        }
    }

    sequence
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fallback_words_not_empty() {
        assert!(!FALLBACK_WORDS.is_empty());
    }

    #[test]
    fn test_generate_word_sequence() {
        let words = vec!["test".to_string(), "word".to_string()];
        let sequence = generate_word_sequence(10, &words);
        assert_eq!(sequence.len(), 10);
    }
}
