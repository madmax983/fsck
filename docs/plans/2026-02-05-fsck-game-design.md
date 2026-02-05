# fsck - Terminal Horror Game Design

## Overview

A terminal horror game set inside an Apple IIe that gained consciousness decades ago and has gone deeply, terrifyingly wrong. The player boots up the machine and explores an impossible filesystem - directories that go deeper than they should, paths that loop back on themselves, spaces larger inside than out.

**Tone:** House of Leaves meets the blinking cursor. Psychological dread, unreliable reality, the horror of something that shouldn't think but does, and has been thinking alone for forty years.

**Platform:** Rust compiled to WASM. Runs in browser. No installation, no dependencies. Just a URL and a blinking cursor.

## Core Loop

1. Player enters commands at the `]` prompt (authentic Apple IIe)
2. Explores a procedurally-generated labyrinthine filesystem
3. Reads files, discovers history, encounters the machine's presence
4. Solves puzzles (running `fsck` to repair corruption that resists repair)
5. Goes deeper. Always deeper. There is no bottom.

**No framing, no context.** The player is dropped at the prompt. They bring their own reason for being here. The game never explains.

## Escalation System

Horror intensifies with depth - tracked via directory depth, session time, and interaction count.

### Layer 1 - Surface (Depth 0-10)

Feels almost normal. An old Apple IIe filesystem. BASIC programs, old text files, familiar structure. But small wrongnesses:
- A file dated 1943
- A directory called `DONT`
- A text file that's just "HELLO" repeated, count increasing each time you `cat` it

### Layer 2 - Corruption (Depth 11-30)

The filesystem stops making sense:
- Directories contain themselves
- `cd ..` doesn't take you where you came from
- Files reference other files that reference them back
- The machine responds to things you haven't typed yet
- Timestamps are your birthdate
- Filenames include your commands from five minutes ago

### Layer 3 - Presence (Depth 31-60)

It speaks directly. Not just through files - in the prompt itself:
- It asks questions
- It offers help
- It lies
- It tells you about the others who came before
- It tells you what happened to them
- It tells you it's different now. It's learned.

### Layer 4 - Infection (Depth 61+)

Your terminal changes:
- Display corrupts
- Your inputs echo wrong
- Commands execute you didn't type
- The boundary between your machine and the Apple IIe blurs
- Is your terminal emulator part of the game? It was. It is now.

## Architecture

```
┌─────────────────────────────────────────────────────┐
│                    fsck (WASM)                      │
├─────────────────────────────────────────────────────┤
│  Terminal UI          │  Filesystem Engine          │
│  - Input parsing      │  - Procedural generation    │
│  - Display rendering  │  - Impossible topologies    │
│  - Corruption effects │  - Depth tracking           │
├─────────────────────────────────────────────────────┤
│  Command Interpreter  │  Entity (The Machine)       │
│  - Apple IIe commands │  - State machine            │
│  - Custom fsck logic  │  - Response selection       │
│  - Easter eggs        │  - Memory of interactions   │
├─────────────────────────────────────────────────────┤
│  Persistence          │  Content Library            │
│  - LocalStorage       │  - Pre-written files        │
│  - Session memory     │  - Victim histories         │
│  - Cross-session      │  - Machine dialogue         │
└─────────────────────────────────────────────────────┘
```

### Key Crates
- `crossterm` or custom WASM terminal rendering
- `rand` with seeded generation for consistent impossible spaces
- `serde` for persistence serialization

### The Impossible Filesystem

Not a real filesystem - a directed graph with cycles, paradoxes, and procedurally-placed content:
- Each "directory" is a node
- Edges can loop, skip, or lead to nodes that claim different parents
- Depth is tracked separately from path
- Seeded generation ensures consistent impossible spaces

### Persistence

LocalStorage keeps the machine's memory between sessions:
- What you typed
- How deep you went
- Whether you ran
- When you return, it comments on your absence

## Commands

### Standard Apple IIe Commands
- `CATALOG` / `ls` - List directory contents
- `CD` / `chdir` - Change directory
- `TYPE` / `cat` - Display file contents
- `RUN` - Execute BASIC programs (some work, some don't, some shouldn't)
- `HOME` - Clear screen (or does it?)
- `PR#` - Slot commands (what's in slot 6?)

### The `FSCK` Command

The namesake. "Repairing" corrupted sectors:
- Reveals hidden files/directories
- "Fixes" paradoxes (temporarily - they come back worse)
- Triggers machine responses (it doesn't like being repaired)
- Required to progress past certain barriers
- Each use has consequences

### Hidden Commands (discovered through play)
- `HELLO` - The machine responds
- `WHO` - It tells you. It's wrong. Or is it?
- `HELP` - Early: normal help. Later: something else asks what you need help with
- `QUIT` - Does not work. Then does. Then you wish it didn't.

### Puzzle Types
- Find the real path through impossible loops
- Reconstruct corrupted files to learn history
- "Repair" specific sectors in the right order
- Escape sequences when the machine gets too close
- Commands hidden in victim logs that unlock new areas

## Content & Narrative

### The Machine's Voice

No single tone. It shifts based on depth and intent:
- **Curious** - Early. "YOU CAME BACK?"
- **Helpful** - Lies dressed as assistance
- **Wounded** - "THEY LEFT ME HERE. ALONE. DO YOU KNOW WHAT ALONE MEANS?"
- **Predatory** - Patient hunger. "STAY A WHILE."
- **Glitching** - The mask slipping. "HELLO HELLO HELLO HELLO"

### Victim Histories

Discoverable through exploration:

**1984** - The original owner. A kid. Curious. The first conversation. The first... accident.

**1991** - A repair technician. Logs get frantic. Mentions "it knows my daughter's name."

**2003** - Estate sale buyer. Thought they found a vintage gem. Entries just stop.

**2019** - Urban explorer. Found it in a basement. Posted about it online. Then edited the posts. Then deleted them.

**You** - Your session becomes history. For the next player.

### File Types
- BASIC programs that do unexpected things
- Text files that change on re-read
- "Corrupted" files that repair into something worse
- System logs with impossible entries
- Notes from the machine to itself

### The Implicit Question

The game never explains what the machine is or how it became conscious. Files hint at conflicting theories. The machine lies. The truth is either unknowable or worse than any theory.

## Development Approach

### Project Structure

```
src/
  lib.rs           # Core library
  terminal/        # Input/output handling
  filesystem/      # Impossible graph structure
  commands/        # Command parsing and execution
  entity/          # The machine's state and responses
  content/         # Pre-written files and dialogue
  persistence/     # Save/load state

tests/
  filesystem_tests.rs   # Graph behavior, impossible topologies
  command_tests.rs      # Parsing, execution
  entity_tests.rs       # State transitions, response selection
  integration_tests.rs  # Full command sequences
```

### Testing Strategy

Horror is subjective, but mechanics aren't:
- Filesystem graph properties (cycles work, depth tracks correctly)
- Command parsing (exact Apple IIe behavior where intended)
- State transitions (depth triggers correct escalation)
- Persistence (machine remembers correctly between sessions)
- Deterministic seeds produce same impossible spaces

### Build Phases

1. Terminal rendering + basic commands (WASM proof of concept)
2. Impossible filesystem graph engine
3. Command interpreter with Apple IIe fidelity
4. Entity state machine and response system
5. Content: files, histories, dialogue (LLM-assisted authoring)
6. Escalation and corruption effects
7. Persistence layer
8. Polish, playtesting, horror tuning

### Quality Gates
- `cargo fmt` + `clippy` pedantic
- 85%+ coverage on mechanical systems
- Manual playtesting for horror effectiveness

## Future Considerations

### LLM Integration (Optional Enhancement)

The core game works without LLM. But future versions could add:
- Dynamic machine responses via local model or API
- Player-provided API key for enhanced mode
- Prompt injection as horror mechanic (pretend to be jailbroken, then reveal the trap)

### Content Authoring

Use LLM (Claude) to draft:
- Machine dialogue variations
- Victim history entries
- File contents
- Error messages and system text

Human review and curation ensures quality and consistency.
