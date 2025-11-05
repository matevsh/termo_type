/// State of a single character during typing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharState {
    /// Character hasn't been typed yet
    Untyped,
    /// Character was typed correctly
    Correct,
    /// Character was typed incorrectly
    Incorrect,
}

/// Represents the state of a word being typed
#[derive(Debug, Clone)]
pub struct WordState {
    /// The target word
    pub target: String,
    /// State of each character
    pub char_states: Vec<CharState>,
    /// Current cursor position in the word
    pub cursor_pos: usize,
}

impl WordState {
    /// Create a new word state
    pub fn new(target: String) -> Self {
        let len = target.chars().count();
        Self {
            target,
            char_states: vec![CharState::Untyped; len],
            cursor_pos: 0,
        }
    }

    /// Add a character to the current position
    pub fn add_char(&mut self, ch: char) -> bool {
        if self.cursor_pos >= self.char_states.len() {
            return false;
        }

        let target_char = self.target.chars().nth(self.cursor_pos);
        if let Some(expected) = target_char {
            if ch == expected {
                self.char_states[self.cursor_pos] = CharState::Correct;
            } else {
                self.char_states[self.cursor_pos] = CharState::Incorrect;
            }
            self.cursor_pos += 1;
            true
        } else {
            false
        }
    }

    /// Remove the last character (backspace)
    pub fn remove_char(&mut self) -> bool {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
            self.char_states[self.cursor_pos] = CharState::Untyped;
            true
        } else {
            false
        }
    }

    /// Check if word is complete (all chars typed)
    pub fn is_complete(&self) -> bool {
        self.cursor_pos >= self.char_states.len()
    }

    /// Check if word has any errors
    pub fn has_errors(&self) -> bool {
        self.char_states.iter().any(|&s| s == CharState::Incorrect)
    }

    /// Get number of correct characters
    pub fn correct_count(&self) -> usize {
        self.char_states.iter().filter(|&&s| s == CharState::Correct).count()
    }

    /// Get number of incorrect characters
    pub fn incorrect_count(&self) -> usize {
        self.char_states.iter().filter(|&&s| s == CharState::Incorrect).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_state_creation() {
        let word = WordState::new("test".to_string());
        assert_eq!(word.char_states.len(), 4);
        assert_eq!(word.cursor_pos, 0);
    }

    #[test]
    fn test_add_correct_char() {
        let mut word = WordState::new("test".to_string());
        word.add_char('t');
        assert_eq!(word.char_states[0], CharState::Correct);
        assert_eq!(word.cursor_pos, 1);
    }

    #[test]
    fn test_add_incorrect_char() {
        let mut word = WordState::new("test".to_string());
        word.add_char('x');
        assert_eq!(word.char_states[0], CharState::Incorrect);
    }

    #[test]
    fn test_backspace() {
        let mut word = WordState::new("test".to_string());
        word.add_char('t');
        word.remove_char();
        assert_eq!(word.char_states[0], CharState::Untyped);
        assert_eq!(word.cursor_pos, 0);
    }
}
