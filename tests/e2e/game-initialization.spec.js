import { test, expect } from '@playwright/test';
import { waitForTerminal, getTerminalText } from './helpers.js';

test.describe('Game Initialization', () => {
  test('should load the game and display boot sequence', async ({ page }) => {
    await page.goto('/web/');
    await waitForTerminal(page);

    const terminalText = await getTerminalText(page);

    // Check for Apple II boot message
    expect(terminalText).toContain('APPLE ][');

    // Check for prompt
    expect(terminalText).toContain(']');
  });

  test('should have proper terminal styling', async ({ page }) => {
    await page.goto('/web/');
    await waitForTerminal(page);

    // Check background is black (accept both rgb and rgba formats)
    const backgroundColor = await page.evaluate(() => {
      const terminal = document.getElementById('terminal');
      return window.getComputedStyle(terminal).backgroundColor;
    });
    expect(backgroundColor).toMatch(/rgba?\(0,\s*0,\s*0/);
  });

  test('should initialize WASM successfully', async ({ page }) => {
    await page.goto('/web/');

    // Wait for WASM to load (no errors in console)
    const errors = [];
    page.on('pageerror', error => errors.push(error.message));

    await waitForTerminal(page);

    // Filter out favicon 404 (expected)
    const criticalErrors = errors.filter(e => !e.includes('favicon'));
    expect(criticalErrors).toHaveLength(0);
  });

  test('should have blinking cursor', async ({ page }) => {
    await page.goto('/web/');
    await waitForTerminal(page);

    // Check cursor element exists
    const cursor = await page.locator('.xterm-cursor');
    await expect(cursor).toBeVisible();
  });
});
