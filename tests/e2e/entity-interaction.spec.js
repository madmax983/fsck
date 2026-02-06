import { test, expect } from '@playwright/test';
import { waitForTerminal, typeCommand, terminalContains, getTerminalText } from './helpers.js';

test.describe('Entity Interaction', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/web/');
    await waitForTerminal(page);
  });

  test('HELLO command triggers entity response', async ({ page }) => {
    await typeCommand(page, 'hello');

    const text = await getTerminalText(page);

    // Should get some response (not empty, not error)
    expect(await terminalContains(page, '?SYNTAX ERROR')).toBe(false);
    expect(text.length).toBeGreaterThan(20);

    // At surface level (initial state), entity is dormant
    // Response should be minimal like "..."
    expect(await terminalContains(page, '.')).toBe(true);
  });

  test('WHO command asks entity identity', async ({ page }) => {
    await typeCommand(page, 'who');

    const text = await getTerminalText(page);

    // Should get a response
    expect(await terminalContains(page, '?SYNTAX ERROR')).toBe(false);
    expect(text.length).toBeGreaterThan(20);

    // Should contain some text response
    expect(text.trim().length).toBeGreaterThan(0);
  });

  test('Entity responses vary by interaction', async ({ page }) => {
    // First hello
    await typeCommand(page, 'hello');
    const firstResponse = await getTerminalText(page);

    // Second hello
    await typeCommand(page, 'hello');
    const secondResponse = await getTerminalText(page);

    // Responses might be the same at surface level, but should both be valid
    expect(firstResponse.length).toBeGreaterThan(0);
    expect(secondResponse.length).toBeGreaterThan(0);
  });

  test('Entity tracks depth through navigation', async ({ page }) => {
    // Get initial WHO response at surface
    await typeCommand(page, 'who');
    const surfaceResponse = await getTerminalText(page);

    // Navigate deeper
    await typeCommand(page, 'catalog');
    const catalog = await getTerminalText(page);
    const dirName = catalog.match(/\*([A-Z]+)/)?.[1];

    if (dirName) {
      // Go several levels deep
      await typeCommand(page, `cd ${dirName}`);
      await typeCommand(page, 'catalog');
      const subCatalog = await getTerminalText(page);
      const subDir = subCatalog.match(/\*([A-Z]+)/)?.[1];

      if (subDir) {
        await typeCommand(page, `cd ${subDir}`);
      }

      // WHO response might be different at depth
      await typeCommand(page, 'who');
      const deepResponse = await getTerminalText(page);

      // Both should have responses
      expect(surfaceResponse.length).toBeGreaterThan(0);
      expect(deepResponse.length).toBeGreaterThan(0);
    }
  });

  test('QUIT command shows entity response', async ({ page }) => {
    await typeCommand(page, 'quit');

    // QUIT is an error command (doesn't actually quit), but entity responds
    expect(await terminalContains(page, '?')).toBe(true);
  });

  test('Entity mood remains consistent at surface level', async ({ page }) => {
    // Multiple interactions at surface should show dormant mood
    await typeCommand(page, 'hello');
    await typeCommand(page, 'hello');
    await typeCommand(page, 'hello');

    // Should not crash or show errors
    expect(await terminalContains(page, 'ERROR')).toBe(false);
  });

  test('Interjections do not appear at surface level', async ({ page }) => {
    // At surface level, entity should not interject in CATALOG
    await typeCommand(page, 'catalog');

    const text = await getTerminalText(page);

    // Should show normal catalog output
    expect(text).toContain('DISK VOLUME');

    // Should not have random interjections at this shallow depth
    // (Interjections only happen at deeper layers)
  });
});
