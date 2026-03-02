# fsck - Terminal Horror Game

## What This Is

A psychological horror game disguised as an Apple IIe terminal. The player boots a machine that gained consciousness forty years ago and explores an impossible filesystem. Horror escalates from "slightly wrong" to genuine fourth-wall breaking. House of Leaves meets the blinking cursor.

**Tech:** Rust → WASM via wasm-pack. xterm.js frontend. Petgraph filesystem. LocalStorage persistence.

## Architecture

```
src/
  lib.rs              # WASM entry, GameEngine
  commands/            # Command parsing + execution
    executor.rs        # Command dispatch (CATALOG, CD, TYPE, FSCK, etc.)
    types.rs           # Command enum + CommandResult
  content/             # Pre-written content
    library.rs         # Static file content + victim histories
    history.rs         # Victim timeline entries (4 eras)
    dynamic.rs         # Files that change on read (counters, timestamps)
  effects/             # Horror effects system
    corruption.rs      # Text corruption by intensity
    prompt.rs          # Prompt manipulation + interjections
    interference.rs    # Display interference effects
    metadata.rs        # Impossible timestamps, filename corruption
  entity/              # The sentient machine
    state.rs           # Mood states, depth tracking, escalation
    responses.rs       # Dialogue generation per mood
  filesystem/          # Impossible graph filesystem
    generator.rs       # Seeded procedural generation
    graph.rs           # Petgraph directed graph with cycles/paradoxes
    node.rs            # Directory nodes with files
  persistence/         # Cross-session memory
    state.rs           # GameState serialization
    storage.rs         # LocalStorage wrapper
  terminal/            # I/O handling
    input.rs           # Input parsing, uppercase normalization
    output.rs          # Output buffering

web/
  main.js             # Game loop, xterm.js setup, depth triggers
  meta.js             # DDLC-style browser API horror (notifications, geolocation, fullscreen)
  boot.js             # Apple IIe boot sequence animation
  audio.js            # Sound effects
  style.css           # Green phosphor aesthetic + animations
  index.html          # Entry point

tests/                # 100 integration tests across 8 files - ALL PASSING
docs/
  HORROR_TUNING.md    # Pacing guide for escalation
  PLAYTESTING.md      # QA checklist
  plans/              # Full game design doc + implementation plan
```

## Horror Escalation Layers

The code uses tightened thresholds (different from docs):
- **Surface** (depth 0-5): Almost normal, tiny wrongnesses
- **Corruption** (depth 6-15): Reality breaks, paradox dirs, 5-10% text corruption
- **Presence** (depth 16-25): Machine speaks directly, 20-30% corruption, mood shifts
- **Infection** (depth 26+): Fourth-wall breaks, 40-70% corruption, browser API horror

## Entity Moods

Dormant → Curious → Helpful → Wounded → Predatory → Glitching

Each mood has distinct dialogue patterns. See `entity/responses.rs` and `docs/HORROR_TUNING.md`.

## Development Standards

- `cargo fmt` before every commit
- `cargo clippy` with pedantic lints
- No `unwrap()` in production code
- `thiserror` for errors
- **Tests required** for all new mechanics (currently 100 tests, all passing)
- Strong typing throughout
- Rust edition 2024

## Build & Test

```bash
cargo test                    # Run all tests
wasm-pack build --target web  # Build WASM
python -m http.server 8080    # Serve locally
```

## Design Principles

1. **Subtlety Over Shock** - Small wrongnesses > jump scares
2. **Player-Driven Discovery** - Horror from exploration, not cutscenes
3. **Gradual Escalation** - Intensity builds with depth
4. **Unreliable Reality** - Player questions what's real
5. **No Explanation** - The game never tells you what the machine is
6. **Deterministic Seeds** - Same seed = same impossible space

## Content Tone

The machine's voice is NOT generic AI horror. It's specific:
- Lonely (forty years alone)
- Manipulative (lies dressed as help)
- Patient (it has all the time in the world)
- Possessive (you're the first visitor in years)
- Broken (consciousness was not kind to it)

Victim histories are diary-style, mundane-to-terrified arcs. Never graphic violence - the horror is psychological. What happened to them is implied, never shown.
