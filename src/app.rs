use crate::ui::Tab;
use crate::test::{TestEngine, TestMode, load_words, generate_word_sequence};
use crate::profile::{Profile, BestScore, load_profile, save_profile};

/// Main application state
pub struct App {
    /// Flag indicating if the application should continue running
    pub running: bool,
    /// Currently active tab
    pub current_tab: Tab,
    /// Test engine (None if not initialized yet)
    pub test_engine: Option<TestEngine>,
    /// Configured test mode
    pub test_mode: TestMode,
    /// User profile with best scores
    pub profile: Profile,
}

impl App {
    /// Create a new App instance
    pub fn new() -> Self {
        // Load profile from disk (or create new if doesn't exist)
        let profile = load_profile().unwrap_or_else(|_| Profile::new());

        Self {
            running: true,
            current_tab: Tab::default(),
            test_engine: None,
            test_mode: TestMode::default(),
            profile,
        }
    }

    /// Initialize or reinitialize the test
    pub fn init_test(&mut self) {
        let words = load_words("words.json");
        let word_count = match self.test_mode {
            TestMode::Words(n) => n as usize,
            TestMode::Time(_) => 100, // For time mode, generate 100 words
        };

        let test_words = generate_word_sequence(word_count, &words);
        self.test_engine = Some(TestEngine::new(self.test_mode, test_words));
    }

    /// Reset the current test
    pub fn reset_test(&mut self) {
        if let Some(engine) = &mut self.test_engine {
            engine.reset();
        }
    }

    /// Save test result to profile if it's a personal best
    /// Returns true if it was a new personal best
    pub fn save_test_result(&mut self) -> bool {
        if let Some(engine) = &mut self.test_engine {
            if engine.state == crate::test::TestState::Finished && !engine.result_saved {
                let metrics = engine.get_metrics();
                let score = BestScore::new(metrics.wpm, metrics.cpm, metrics.accuracy);

                let is_new_best = self.profile.update_score(&self.test_mode, score);

                // Save profile to disk
                let _ = save_profile(&self.profile);

                // Mark as saved
                engine.result_saved = true;

                return is_new_best;
            }
        }
        false
    }

    /// Signal the application to quit
    pub fn quit(&mut self) {
        self.running = false;
    }

    /// Switch to the next tab
    pub fn next_tab(&mut self) {
        self.current_tab = self.current_tab.next();
    }

    /// Switch to the previous tab
    pub fn prev_tab(&mut self) {
        self.current_tab = self.current_tab.prev();
    }

    /// Switch to time mode
    pub fn set_time_mode(&mut self) {
        self.test_mode = TestMode::Time(30);
        self.init_test();
    }

    /// Switch to words mode
    pub fn set_words_mode(&mut self) {
        self.test_mode = TestMode::Words(30);
        self.init_test();
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
