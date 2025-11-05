# TermoType

A terminal-based typing speed test application built with Rust and Ratatui, inspired by Monkey Type.

## Features

- **Multiple Test Modes**
  - 30-second time-based test
  - 30-word count-based test

- **Real-time Metrics**
  - WPM (Words Per Minute)
  - CPM (Characters Per Minute)
  - Accuracy percentage
  - Live progress tracking

- **Profile System**
  - Automatic saving of personal best scores
  - Separate records for 30s and 30-word modes
  - Persistent storage in `~/.config/termotype/profile.json`

- **Color-coded Typing**
  - Green for correct characters
  - Red for mistakes
  - Visual cursor position
  - Gray for untyped text

## Installation

1. Make sure you have Rust installed (https://rustup.rs/)

2. Clone or navigate to the project directory

3. Build the project:
```bash
cargo build --release
```

4. Run the application:
```bash
cargo run --release
```

## Usage

### Navigation

- `1`, `2`, `3` - Switch to Test, Stats, or Options tab
- `Tab` / `Shift+Tab` - Navigate between tabs
- `Esc` or `q` - Quit application

### Test Tab

- Start typing to begin the test automatically
- `Space` - Move to next word
- `Backspace` - Delete last character
- `Enter` - Reset test

### Options Tab

- `t` - Switch to Time mode (30 seconds)
- `w` - Switch to Words mode (30 words)

### Stats Tab

- View your personal best scores
- See WPM, CPM, and accuracy for each mode
- Check when you achieved each record

## How It Works

1. **Start**: Navigate to the Test tab and start typing the displayed words
2. **Type**: Type each word correctly and press Space to move to the next word
3. **Mistakes**: Errors are highlighted in red but you can continue typing
4. **Finish**: The test automatically ends when time runs out (30s mode) or all words are typed (30-word mode)
5. **Results**: Your score is automatically saved if it's a personal best!

## Technical Details

- **Language**: Rust
- **TUI Framework**: Ratatui
- **Terminal Backend**: Crossterm
- **Data Format**: JSON for profiles and word lists

## Project Structure

```
src/
├── main.rs              # Entry point and event loop
├── app.rs               # Application state
├── ui/                  # UI components
│   ├── tabs.rs          # Tab navigation
│   ├── test_view.rs     # Typing test interface
│   ├── stats_view.rs    # Statistics display
│   └── options_view.rs  # Settings interface
├── test/                # Test logic
│   ├── engine.rs        # Test state machine
│   ├── input.rs         # Input validation
│   ├── metrics.rs       # WPM/CPM calculations
│   └── words.rs         # Word loading
└── profile/             # Profile management
    ├── models.rs        # Data structures
    └── storage.rs       # Persistence

## Customization

### Custom Word List

Edit `words.json` in the project root to customize the word list. The file should contain a JSON array of strings:

```json
[
  "word1",
  "word2",
  "word3"
]
```

## License

This project is open source and available under the MIT License.

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

## Acknowledgments

- Inspired by [Monkey Type](https://monkeytype.com/)
- Built with [Ratatui](https://github.com/ratatui-org/ratatui)
