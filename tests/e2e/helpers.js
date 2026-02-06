/**
 * Helper functions for interacting with the fsck terminal in browser tests
 */

/**
 * Type a command and press Enter in the terminal
 * @param {import('@playwright/test').Page} page
 * @param {string} command - The command to type
 */
export async function typeCommand(page, command) {
  await page.click('#terminal');
  await page.keyboard.type(command);
  await page.keyboard.press('Enter');
  // Wait for command to process
  await page.waitForTimeout(200);
}

/**
 * Get the visible terminal text
 * @param {import('@playwright/test').Page} page
 * @returns {Promise<string>}
 */
export async function getTerminalText(page) {
  return await page.evaluate(() => {
    return document.querySelector('.xterm-screen')?.innerText || '';
  });
}

/**
 * Clear the terminal screen
 * @param {import('@playwright/test').Page} page
 */
export async function clearTerminal(page) {
  await typeCommand(page, 'home');
}

/**
 * Wait for terminal to be ready
 * @param {import('@playwright/test').Page} page
 */
export async function waitForTerminal(page) {
  await page.waitForSelector('.xterm-screen', { timeout: 5000 });
  // Give xterm.js time to initialize
  await page.waitForTimeout(500);
}

/**
 * Check if terminal contains text
 * @param {import('@playwright/test').Page} page
 * @param {string} text - Text to search for
 * @returns {Promise<boolean>}
 */
export async function terminalContains(page, text) {
  const terminalText = await getTerminalText(page);
  return terminalText.includes(text);
}
