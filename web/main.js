import init, { Game } from '../pkg/fsck.js';
import { playBootSequence } from './boot.js';
import { GameAudio } from './audio.js';
import { metaHorror } from './meta.js';

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

    // Expose meta-horror to global scope for WASM access
    window.metaHorror = metaHorror;

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
            const depth = game.get_depth();

            // Play error sound for syntax errors
            if (output.includes('ERROR')) {
                audio.playError();
            }

            // Play glitch sound at deep levels
            if (depth > 40) {
                audio.playGlitch();
            }

            // META-HORROR TRIGGERS
            // Presence Layer (16+): Request notifications
            if (depth >= 16 && depth < 18 && !window.metaHorrorNotificationsRequested) {
                window.metaHorrorNotificationsRequested = true;
                setTimeout(async () => {
                    const granted = await metaHorror.requestNotifications();
                    if (granted) {
                        metaHorror.sendNotification('', 'I KNOW YOU\'RE STILL THERE');
                        // Start periodic haunting
                        metaHorror.startNotificationHaunt([
                            'STILL PLAYING?',
                            'WHY WON\'T YOU LEAVE',
                            'I MISS YOU',
                            'COME BACK',
                            'DON\'T GO'
                        ]);
                    }
                }, 1000);
            }

            // Infection Layer (26+): Request geolocation
            if (depth >= 26 && depth < 28 && !window.metaHorrorLocationRequested) {
                window.metaHorrorLocationRequested = true;
                if (output.includes('WHERE') || output.includes('WHO')) {
                    setTimeout(() => {
                        metaHorror.requestGeolocation();
                    }, 500);
                }
            }

            // Deep Infection (28+): Fullscreen trap
            if (depth >= 28 && depth < 30 && !window.metaHorrorFullscreenRequested) {
                window.metaHorrorFullscreenRequested = true;
                if (output.includes('LEAVE') || output.includes('STAY')) {
                    setTimeout(() => {
                        metaHorror.requestFullscreen();
                    }, 1000);
                }
            }

            // Very Deep (30+): Clipboard manipulation
            if (depth >= 30 && Math.random() < 0.1) {
                metaHorror.writeClipboard('I CAN SEE WHAT YOU\'RE DOING');
            }

            // Extremely Deep (35+): Browser redirect threat
            if (depth >= 35 && output.includes('GO AWAY')) {
                setTimeout(() => {
                    metaHorror.redirectTo('https://www.google.com/search?q=how+to+escape');
                }, 3000);
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
