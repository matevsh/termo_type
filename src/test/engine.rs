use std::time::Instant;
use super::input::WordState;
use super::metrics::TestMetrics;

/// Test state machine
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestState {
    /// Test hasn't started yet
    NotStarted,
    /// Test is currently running
    InProgress,
    /// Test has finished
    Finished,
}

/// Test mode - either time-based or word count-based
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestMode {
    /// Time-based test (in seconds)
    Time(u32),
    /// Word count-based test
    Words(u32),
}

impl TestMode {
    /// Get default 30 second mode
    pub fn default_time() -> Self {
        TestMode::Time(30)
    }

    /// Get default 30 words mode
    pub fn default_words() -> Self {
        TestMode::Words(30)
    }
}

impl Default for TestMode {
    fn default() -> Self {
        Self::default_time()
    }
}

/// Main test engine that manages the typing test
pub struct TestEngine {
    /// Current state of the test
    pub state: TestState,
    /// Test mode (time or words)
    pub mode: TestMode,
    /// List of words to type
    pub words: Vec<String>,
    /// Current word index (which word user is typing)
    pub current_word_index: usize,
    /// Current word state (tracking character-by-character progress)
    pub current_word_state: Option<WordState>,
    /// Time when test started
    pub start_time: Option<Instant>,
    /// Time when test finished
    pub end_time: Option<Instant>,
    /// Total characters typed (including mistakes)
    pub total_chars_typed: usize,
    /// Correct characters typed
    pub correct_chars: usize,
    /// Incorrect characters (mistakes)
    pub incorrect_chars: usize,
    /// Whether the result has been saved to profile
    pub result_saved: bool,
}

impl TestEngine {
    /// Create a new test engine
    pub fn new(mode: TestMode, words: Vec<String>) -> Self {
        let current_word_state = words.first().map(|w| WordState::new(w.clone()));

        Self {
            state: TestState::NotStarted,
            mode,
            words,
            current_word_index: 0,
            current_word_state,
            start_time: None,
            end_time: None,
            total_chars_typed: 0,
            correct_chars: 0,
            incorrect_chars: 0,
            result_saved: false,
        }
    }

    /// Start the test
    pub fn start(&mut self) {
        if self.state == TestState::NotStarted {
            self.state = TestState::InProgress;
            self.start_time = Some(Instant::now());
        }
    }

    /// Finish the test
    pub fn finish(&mut self) {
        if self.state == TestState::InProgress {
            self.state = TestState::Finished;
            self.end_time = Some(Instant::now());
        }
    }

    /// Get elapsed time in seconds
    pub fn elapsed_seconds(&self) -> f64 {
        if let Some(start) = self.start_time {
            let end = self.end_time.unwrap_or_else(Instant::now);
            end.duration_since(start).as_secs_f64()
        } else {
            0.0
        }
    }

    /// Get the current word being typed
    pub fn current_word(&self) -> Option<&str> {
        self.words.get(self.current_word_index).map(|s| s.as_str())
    }

    /// Check if test should auto-finish
    pub fn should_auto_finish(&self) -> bool {
        match self.mode {
            TestMode::Time(seconds) => {
                self.elapsed_seconds() >= seconds as f64
            }
            TestMode::Words(count) => {
                self.current_word_index >= count as usize
            }
        }
    }

    /// Type a character
    pub fn type_char(&mut self, ch: char) {
        // Auto-start test on first keypress
        if self.state == TestState::NotStarted {
            self.start();
        }

        if self.state != TestState::InProgress {
            return;
        }

        if let Some(word_state) = &mut self.current_word_state {
            if word_state.add_char(ch) {
                self.total_chars_typed += 1;
            }
        }
    }

    /// Handle backspace
    pub fn backspace(&mut self) {
        if self.state != TestState::InProgress {
            return;
        }

        if let Some(word_state) = &mut self.current_word_state {
            word_state.remove_char();
        }
    }

    /// Move to next word (called on Space press)
    pub fn next_word(&mut self) {
        if self.state != TestState::InProgress {
            return;
        }

        // Update stats from current word
        if let Some(word_state) = &self.current_word_state {
            self.correct_chars += word_state.correct_count();
            self.incorrect_chars += word_state.incorrect_count();
        }

        // Move to next word
        self.current_word_index += 1;

        // Initialize next word state or finish if done
        if let Some(next_word) = self.words.get(self.current_word_index) {
            self.current_word_state = Some(WordState::new(next_word.clone()));
        } else {
            self.current_word_state = None;
        }

        // Check if we should auto-finish
        if self.should_auto_finish() {
            self.finish();
        }
    }

    /// Get current metrics
    pub fn get_metrics(&self) -> TestMetrics {
        TestMetrics::calculate(
            self.correct_chars,
            self.incorrect_chars,
            self.elapsed_seconds(),
        )
    }

    /// Reset the test to initial state
    pub fn reset(&mut self) {
        self.state = TestState::NotStarted;
        self.current_word_index = 0;
        self.current_word_state = self.words.first().map(|w| WordState::new(w.clone()));
        self.start_time = None;
        self.end_time = None;
        self.total_chars_typed = 0;
        self.correct_chars = 0;
        self.incorrect_chars = 0;
        self.result_saved = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_creation() {
        let words = vec!["test".to_string(), "words".to_string()];
        let engine = TestEngine::new(TestMode::default_time(), words);
        assert_eq!(engine.state, TestState::NotStarted);
    }

    #[test]
    fn test_start_finish() {
        let words = vec!["test".to_string()];
        let mut engine = TestEngine::new(TestMode::default_time(), words);

        engine.start();
        assert_eq!(engine.state, TestState::InProgress);
        assert!(engine.start_time.is_some());

        engine.finish();
        assert_eq!(engine.state, TestState::Finished);
        assert!(engine.end_time.is_some());
    }
}
