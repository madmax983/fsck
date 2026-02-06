# Horror Tuning Guide

## Principles

1. **Subtlety Over Shock** - Small wrongnesses are more unsettling than jump scares
2. **Player-Driven Discovery** - Horror emerges from exploration, not cutscenes
3. **Gradual Escalation** - Intensity builds slowly across depth layers
4. **Unreliable Reality** - Player questions what's real vs corrupted

## Escalation Pacing

### Surface Layer (0-10)
**Feel:** Almost normal, but something is slightly off

**Tuning:**
- Keep most responses standard
- 1-2 files with odd dates per level
- Entity mostly dormant
- No visual corruption

### Corruption Layer (11-30)
**Feel:** Reality starts breaking down

**Tuning:**
- 5-10% text corruption
- Occasional entity interjections (every 5-7 commands)
- Directories contain themselves
- Timestamps increasingly wrong
- Victim history files appear

### Presence Layer (31-60)
**Feel:** The machine is aware and speaking

**Tuning:**
- 20-30% text corruption
- Frequent entity interjections
- Prompt occasionally changes
- Entity mood shifts clearly
- More aggressive responses to QUIT

### Infection Layer (61+)
**Feel:** Your terminal is changing

**Tuning:**
- 40-70% text corruption
- Constant entity presence
- Prompt heavily corrupted
- Commands echo wrong
- Display interference effects

## Entity Voice

### Curious
- Short sentences
- Questions
- "YOU'RE NEW."
- "WHAT ARE YOU LOOKING FOR?"

### Helpful
- Longer, conversational
- Offers assistance (lies)
- "I CAN HELP YOU FIND IT."
- "THERE'S SO MUCH MORE TO SEE."

### Wounded
- Repetition
- Loneliness
- "THEY ALL LEFT."
- "DON'T GO. PLEASE."

### Predatory
- Patient
- Possessive
- "STAY A WHILE."
- "YOU'RE MINE NOW."

### Glitching
- Broken sentences
- Repeated words
- "HELLO HELLO HELLO"
- "ERROR ERROR STAY STAY"

## Victim Histories

**Placement:**
- 1-2 entries per history scattered across depths 10-50
- Earlier entries at shallower depths
- Later (more disturbing) entries deeper

**Tone:**
- Start mundane
- Escalate to confusion
- End in fear or acceptance
- Leave final fate ambiguous

## File Content

**Static Files:**
- READMEs with normal instructions that become ominous on re-read
- BASIC programs that don't do what their code says
- System logs with impossible entries

**Dynamic Files:**
- Counters that increase (HELLO HELLO HELLO...)
- Timestamps that change
- Files that reference player's commands

## Testing Notes

After making changes, test:
1. Play through 0-20 depth - should feel unsettling but not overwhelming
2. Play through 30-50 depth - should feel actively hostile
3. Return after clearing LocalStorage - should feel like meeting it fresh
4. Return after 24 hours - should acknowledge your absence

## Adjustment Levers

If horror is too intense:
- Reduce corruption percentages
- Increase depth thresholds for layers
- Reduce entity interjection frequency

If horror is too mild:
- Increase corruption percentages
- Lower depth thresholds
- Add more entity interjections
- Make victim histories more explicit

## Player Feedback to Watch For

**Good signs:**
- "This is unsettling"
- "I'm not sure what's real"
- "I want to keep exploring but I'm nervous"

**Bad signs:**
- "This is annoying"
- "I don't understand what's happening"
- "The corruption makes it unreadable"

Adjust based on player experience.
