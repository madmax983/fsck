# fsck Playtesting Checklist

## Functional Testing

### Basic Commands
- [ ] CATALOG shows directories and files
- [ ] CD navigates between directories
- [ ] CD .. returns to parent
- [ ] TYPE displays file contents
- [ ] HOME clears screen
- [ ] HELP shows command list
- [ ] HELLO triggers entity response
- [ ] WHO triggers entity response
- [ ] FSCK runs filesystem check

### Filesystem Behavior
- [ ] Directories can contain themselves
- [ ] Same seed produces same structure
- [ ] Different seeds produce different structures
- [ ] Depth increases when navigating deeper
- [ ] Files are distributed throughout filesystem

### Entity Behavior
- [ ] Responses change with depth
- [ ] Mood shifts appropriately
- [ ] Interjections occur at deeper levels
- [ ] Quit command is denied or mocked

### Persistence
- [ ] State saves after commands
- [ ] Game loads previous state on restart
- [ ] Entity remembers player across sessions
- [ ] Return greetings display correctly

### Visual Effects
- [ ] Text corruption intensifies with depth
- [ ] Prompt changes at deeper levels
- [ ] Timestamps become impossible
- [ ] Filenames show corruption

## Horror Effectiveness

### Pacing
- [ ] First 10 depths feel mostly normal
- [ ] Weirdness emerges gradually (depth 11-30)
- [ ] Entity becomes more present (depth 31-60)
- [ ] Terminal corruption is unsettling (depth 61+)

### Atmosphere
- [ ] Boot sequence sets tone
- [ ] Green phosphor aesthetic works
- [ ] Victim histories are disturbing
- [ ] Entity dialogue is unsettling

### Player Agency
- [ ] Player feels exploration is their choice
- [ ] Navigation is intuitive
- [ ] Impossible topology is mind-bending not frustrating
- [ ] No feeling of "wrong" choices

### Specific Moments to Test
- [ ] First self-containing directory
- [ ] First entity interjection
- [ ] Finding first victim history
- [ ] Reaching depth 30 (presence layer)
- [ ] Reaching depth 60 (infection layer)
- [ ] Returning after 24+ hours away

## Performance
- [ ] WASM loads quickly
- [ ] No lag on command input
- [ ] Smooth terminal rendering
- [ ] LocalStorage saves without blocking

## Browser Compatibility
- [ ] Chrome/Edge
- [ ] Firefox
- [ ] Safari
- [ ] Mobile browsers (optional)
