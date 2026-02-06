import { test, expect } from '@playwright/test';
import { waitForTerminal, typeCommand, terminalContains, getTerminalText } from './helpers.js';

test.describe('Navigation', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/web/');
    await waitForTerminal(page);
  });

  test('CD changes directory successfully', async ({ page }) => {
    // First get the list of directories
    await typeCommand(page, 'catalog');
    const initialCatalog = await getTerminalText(page);

    // Find a directory name (starts with *)
    const dirMatch = initialCatalog.match(/\*([A-Z]+)/);
    expect(dirMatch).not.toBeNull();

    const dirName = dirMatch[1];

    // Change to that directory
    await typeCommand(page, `cd ${dirName}`);

    // Should not show error
    expect(await terminalContains(page, '?')).toBe(false);

    // List contents of new directory
    await typeCommand(page, 'catalog');
    const newCatalog = await getTerminalText(page);

    // Should show DISK VOLUME
    expect(newCatalog).toContain('DISK VOLUME');
  });

  test('CD to non-existent directory shows error', async ({ page }) => {
    await typeCommand(page, 'cd NONEXISTENT');

    expect(await terminalContains(page, '?')).toBe(true);
  });

  test('CD with no argument shows error', async ({ page }) => {
    await typeCommand(page, 'cd');

    expect(await terminalContains(page, '?SYNTAX ERROR')).toBe(true);
  });

  test('Can navigate to subdirectories', async ({ page }) => {
    // Go to first directory
    await typeCommand(page, 'catalog');
    let text = await getTerminalText(page);
    const firstDir = text.match(/\*([A-Z]+)/)?.[1];

    if (firstDir) {
      await typeCommand(page, `cd ${firstDir}`);
      await typeCommand(page, 'catalog');

      // Check if there are subdirectories
      text = await getTerminalText(page);
      const subDir = text.match(/\*([A-Z]+)/)?.[1];

      if (subDir) {
        await typeCommand(page, `cd ${subDir}`);

        // Should succeed (no error)
        expect(await terminalContains(page, '?SYNTAX ERROR')).toBe(false);
      }
    }
  });

  test('Can navigate using .. to go up', async ({ page }) => {
    // Navigate down
    await typeCommand(page, 'catalog');
    const text = await getTerminalText(page);
    const dirName = text.match(/\*([A-Z]+)/)?.[1];

    if (dirName) {
      await typeCommand(page, `cd ${dirName}`);
      await typeCommand(page, 'catalog');

      // Navigate back up
      await typeCommand(page, 'cd ..');

      // Should be back at root
      expect(await terminalContains(page, '?')).toBe(false);
    }
  });
});
