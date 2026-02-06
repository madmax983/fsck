import { test, expect } from '@playwright/test';
import { waitForTerminal, typeCommand, terminalContains } from './helpers.js';

test.describe('Basic Commands', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/web/');
    await waitForTerminal(page);
  });

  test('CATALOG command lists directories and files', async ({ page }) => {
    await typeCommand(page, 'catalog');

    expect(await terminalContains(page, 'DISK VOLUME 254')).toBe(true);
    expect(await terminalContains(page, 'DIR')).toBe(true);
  });

  test('HELP command displays available commands', async ({ page }) => {
    await typeCommand(page, 'help');

    expect(await terminalContains(page, 'AVAILABLE COMMANDS')).toBe(true);
    expect(await terminalContains(page, 'CATALOG')).toBe(true);
    expect(await terminalContains(page, 'CD')).toBe(true);
    expect(await terminalContains(page, 'TYPE')).toBe(true);
    expect(await terminalContains(page, 'FSCK')).toBe(true);
  });

  test('FSCK command runs filesystem check', async ({ page }) => {
    await typeCommand(page, 'fsck');

    expect(await terminalContains(page, 'CHECKING DISK')).toBe(true);
  });

  test('HOME command clears screen', async ({ page }) => {
    await typeCommand(page, 'catalog');
    expect(await terminalContains(page, 'DISK VOLUME')).toBe(true);

    await typeCommand(page, 'home');

    // After HOME, previous output should be cleared
    // The terminal might still show the prompt
    const text = await page.evaluate(() => {
      return document.querySelector('.xterm-screen')?.innerText || '';
    });

    // Should be much shorter after clear
    expect(text.length).toBeLessThan(100);
  });

  test('Unknown command shows error', async ({ page }) => {
    await typeCommand(page, 'invalid_command');

    expect(await terminalContains(page, '?SYNTAX ERROR')).toBe(true);
  });

  test('Empty command does nothing', async ({ page }) => {
    await typeCommand(page, '');

    // Should just show another prompt, no error
    expect(await terminalContains(page, '?SYNTAX ERROR')).toBe(false);
  });

  test('Commands are case-insensitive', async ({ page }) => {
    await typeCommand(page, 'HeLp');

    expect(await terminalContains(page, 'AVAILABLE COMMANDS')).toBe(true);
  });
});
