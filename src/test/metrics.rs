/// Test metrics
#[derive(Debug, Clone, Copy)]
pub struct TestMetrics {
    /// Words per minute
    pub wpm: f64,
    /// Characters per minute
    pub cpm: f64,
    /// Accuracy percentage (0-100)
    pub accuracy: f64,
}

impl TestMetrics {
    /// Calculate metrics from test data
    pub fn calculate(
        correct_chars: usize,
        incorrect_chars: usize,
        elapsed_seconds: f64,
    ) -> Self {
        let total_chars = correct_chars + incorrect_chars;

        let wpm = calculate_wpm(correct_chars, elapsed_seconds);
        let cpm = calculate_cpm(correct_chars, elapsed_seconds);
        let accuracy = calculate_accuracy(correct_chars, total_chars);

        Self { wpm, cpm, accuracy }
    }
}

impl Default for TestMetrics {
    fn default() -> Self {
        Self {
            wpm: 0.0,
            cpm: 0.0,
            accuracy: 100.0,
        }
    }
}

/// Calculate Words Per Minute (WPM)
/// Standard: 1 word = 5 characters
pub fn calculate_wpm(chars_typed: usize, time_sec: f64) -> f64 {
    if time_sec <= 0.0 {
        return 0.0;
    }

    let words = chars_typed as f64 / 5.0;
    let minutes = time_sec / 60.0;

    words / minutes
}

/// Calculate Characters Per Minute (CPM)
pub fn calculate_cpm(chars_typed: usize, time_sec: f64) -> f64 {
    if time_sec <= 0.0 {
        return 0.0;
    }

    let minutes = time_sec / 60.0;
    chars_typed as f64 / minutes
}

/// Calculate accuracy percentage
/// Returns a value between 0 and 100
pub fn calculate_accuracy(correct: usize, total: usize) -> f64 {
    if total == 0 {
        return 100.0;
    }

    (correct as f64 / total as f64) * 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wpm_calculation() {
        // 50 chars in 60 seconds = 10 WPM
        let wpm = calculate_wpm(50, 60.0);
        assert!((wpm - 10.0).abs() < 0.01);
    }

    #[test]
    fn test_cpm_calculation() {
        // 100 chars in 60 seconds = 100 CPM
        let cpm = calculate_cpm(100, 60.0);
        assert!((cpm - 100.0).abs() < 0.01);
    }

    #[test]
    fn test_accuracy() {
        assert_eq!(calculate_accuracy(80, 100), 80.0);
        assert_eq!(calculate_accuracy(100, 100), 100.0);
        assert_eq!(calculate_accuracy(0, 100), 0.0);
    }
}
