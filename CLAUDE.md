# AI Rules for TermoType
Terminal-based typing speed test application with Ratatui TUI framework.

## RUST CORE PATTERNS
- Use Result<T, E> for all fallible operations, avoid unwrap() in production - apply ? operator for clean error propagation
- Leverage ownership and borrowing rules to prevent data races - prefer move semantics over cloning for performance
- Implement newtype pattern for domain types (Wpm, Accuracy) and use enums for state machines (Idle|Running|Completed)
- Apply builder pattern for complex configurations and use From/TryFrom traits for type conversions
- Use const generics and zero-copy operations where possible, profile with flamegraph for performance bottlenecks

## TUI & EVENT HANDLING
- Implement Widget trait for reusable components, use immediate mode rendering with dirty region tracking
- Handle crossterm events with non-blocking poll(), ensure terminal state restoration with RAII guards
- Use Frame rendering with constraint-based layouts for responsive design, maintain 60+ FPS during active typing
- Apply enable_raw_mode() for character input capture, use alternate screen buffer to preserve terminal history
- Implement graceful degradation for missing terminal features, provide clear error messages before TUI initialization

## STATE & PERSISTENCE
- Design centralized AppState with clear ownership boundaries, prefer message passing over shared mutability
- Use serde with versioned schemas and atomic file writes (tempfile + rename) for profile persistence
- Calculate metrics with sliding windows for real-time updates, use Instant for monotonic timing
- Implement repository pattern for testable storage abstraction, cache expensive calculations with memoization
- Write comprehensive tests: unit tests for business logic, integration tests with TestBackend, property tests for invariants