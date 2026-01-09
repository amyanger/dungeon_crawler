# Dungeon Crawler

A terminal-based roguelike dungeon crawler game written in Rust.

## Features

- Procedural dungeon generation
- Real-time terminal rendering with crossterm
- Player movement and combat system
- Enemy AI with pathfinding
- Random encounters and loot

## Requirements

- Rust 1.70+
- A terminal that supports ANSI escape codes

## Installation

```bash
git clone https://github.com/amyanger/dungeon_crawler.git
cd dungeon_crawler
cargo build --release
```

## Running

```bash
cargo run
```

Or run the release build:

```bash
./target/release/dungeon_crawler
```

## Controls

- `W/A/S/D` or Arrow Keys - Move
- `Space` - Attack
- `Q` - Quit

## Dependencies

- `crossterm` - Terminal manipulation
- `rand` - Random number generation

## Author

Arjun Myanger - [GitHub](https://github.com/amyanger)

## License

MIT License - see LICENSE file for details
