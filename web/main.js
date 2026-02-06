import init, { Game } from '../pkg/fsck.js';
import { playBootSequence } from './boot.js';
import { GameAudio } from './audio.js';

async function main() {
    // Initialize WASM
    await init();

    // Create game instance
    const game = new Game();

    // Initialize xterm.js with Apple IIe aesthetics
    const term = new Terminal({
        fontFamily: '"Apple II", "Courier New", monospace',
        fontSize: 16,
        cols: 80,  // Classic 80-column terminal
        rows: 24,  // Standard terminal height
        convertEol: true,  // Convert \n to \r\n automatically
        theme: {
            foreground: '#33ff33',
            background: '#000000',
            cursor: '#33ff33',
        },
        cursorBlink: true,
        cursorStyle: 'block',
        scrollback: 1000,
    });

    term.open(document.getElementById('terminal'));

    // Initialize audio (requires user gesture)
    const audio = new GameAudio();
    document.addEventListener('click', () => audio.init(), { once: true });

    // Play boot sequence
    await playBootSequence(term, game);

    // Show prompt
    term.write(game.get_prompt());

    // Input buffer
    let inputBuffer = '';

    // Handle keyboard input
    term.onKey(({ key, domEvent }) => {
        const ev = domEvent;

        if (ev.key === 'Enter') {
            term.writeln('');
            const output = game.process_input(inputBuffer);

            // Play error sound for syntax errors
            if (output.includes('ERROR')) {
                audio.playError();
            }

            // Play glitch sound at deep levels
            if (game.get_depth() > 40) {
                audio.playGlitch();
            }

            if (output) {
                term.write(output);
            }
            term.write(game.get_prompt());
            inputBuffer = '';
        } else if (ev.key === 'Backspace') {
            if (inputBuffer.length > 0) {
                inputBuffer = inputBuffer.slice(0, -1);
                term.write('\b \b');
            }
        } else if (key.length === 1 && !ev.ctrlKey && !ev.altKey) {
            audio.playKeystroke();
            inputBuffer += key;
            term.write(key.toUpperCase());
        }
    });
}

main();
