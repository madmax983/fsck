export async function playBootSequence(term, game) {
    const delay = (ms) => new Promise(resolve => setTimeout(resolve, ms));

    // Apple IIe boot text
    term.writeln('APPLE ][');
    await delay(300);
    term.writeln('');
    await delay(200);
    term.writeln('DOS VERSION 3.3');
    await delay(400);
    term.writeln('');
    await delay(100);
    term.writeln('CHECKING DISK...');
    await delay(600);

    // Fake disk checks with random sectors
    for (let i = 0; i < 5; i++) {
        const sector = Math.floor(Math.random() * 256);
        term.write(`\rSECTOR ${sector}...`);
        await delay(100);
    }

    term.writeln('');
    await delay(300);
    term.writeln('');
    await delay(200);

    // Check if returning player
    const isReturning = game.is_returning_player();
    if (isReturning) {
        term.writeln('WELCOME BACK');
        await delay(500);
        term.writeln('');
        await delay(300);
    }

    term.writeln('');
}
