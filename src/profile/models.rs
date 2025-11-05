use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// Represents a best score for a specific test mode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BestScore {
    /// Words per minute
    pub wpm: f64,
    /// Characters per minute
    pub cpm: f64,
    /// Accuracy percentage
    pub accuracy: f64,
    /// When this score was achieved (Unix timestamp)
    pub timestamp: u64,
}

impl BestScore {
    /// Create a new best score
    pub fn new(wpm: f64, cpm: f64, accuracy: f64) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            wpm,
            cpm,
            accuracy,
            timestamp,
        }
    }

    /// Check if this score is better than another (based on WPM)
    pub fn is_better_than(&self, other: &BestScore) -> bool {
        self.wpm > other.wpm
    }
}

/// User profile with best scores
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    /// Best score for 30 seconds mode
    pub best_30_seconds: Option<BestScore>,
    /// Best score for 30 words mode
    pub best_30_words: Option<BestScore>,
}

impl Profile {
    /// Create a new empty profile
    pub fn new() -> Self {
        Self {
            best_30_seconds: None,
            best_30_words: None,
        }
    }

    /// Update profile with a new score
    /// Returns true if the score was a new personal best
    pub fn update_score(&mut self, mode: &crate::test::TestMode, score: BestScore) -> bool {
        match mode {
            crate::test::TestMode::Time(30) => {
                if let Some(current_best) = &self.best_30_seconds {
                    if score.is_better_than(current_best) {
                        self.best_30_seconds = Some(score);
                        true
                    } else {
                        false
                    }
                } else {
                    self.best_30_seconds = Some(score);
                    true
                }
            }
            crate::test::TestMode::Words(30) => {
                if let Some(current_best) = &self.best_30_words {
                    if score.is_better_than(current_best) {
                        self.best_30_words = Some(score);
                        true
                    } else {
                        false
                    }
                } else {
                    self.best_30_words = Some(score);
                    true
                }
            }
            _ => false, // Don't track custom modes
        }
    }
}

impl Default for Profile {
    fn default() -> Self {
        Self::new()
    }
}
