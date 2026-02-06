# End-to-End Browser Tests

Automated browser tests for the fsck game using Playwright.

## Setup

Install dependencies:

```bash
npm install
npx playwright install
```

## Running Tests

### Run all tests
```bash
npm run test:e2e
```

### Run with UI mode (recommended for development)
```bash
npm run test:e2e:ui
```

### Run in headed mode (see the browser)
```bash
npm run test:e2e:headed
```

### Debug a specific test
```bash
npm run test:e2e:debug
```

### View test report
```bash
npm run test:e2e:report
```

## Test Structure

### Test Files

- **game-initialization.spec.js** - Tests game loading, WASM initialization, and boot sequence
- **basic-commands.spec.js** - Tests fundamental commands (CATALOG, HELP, FSCK, HOME, etc.)
- **navigation.spec.js** - Tests CD command and directory navigation
- **file-operations.spec.js** - Tests TYPE command and file reading
- **entity-interaction.spec.js** - Tests HELLO, WHO, and entity mood system
- **filesystem-paradoxes.spec.js** - Tests the impossible filesystem features and paradoxes

### Helper Functions

The `helpers.js` file provides utilities for interacting with the terminal:

- `typeCommand(page, command)` - Type a command and press Enter
- `getTerminalText(page)` - Get all visible terminal text
- `terminalContains(page, text)` - Check if terminal contains specific text
- `clearTerminal(page)` - Clear the terminal screen
- `waitForTerminal(page)` - Wait for terminal to be ready

## Test Coverage

The test suite covers:

✅ Game initialization and WASM loading
✅ All basic commands (CATALOG, CD, TYPE, HELP, FSCK, HOME)
✅ Directory navigation and the parent (..) operator
✅ File reading and content display
✅ Entity interaction and mood system
✅ Filesystem paradoxes and self-referential directories
✅ Error handling for invalid commands and missing files
✅ Case-insensitive command parsing
✅ Deterministic filesystem generation with seeds

## Browser Support

Tests run on:
- Chromium (Chrome/Edge)
- Firefox
- WebKit (Safari)

## CI/CD

The test suite is configured for CI environments:
- Automatic retries on failure
- Screenshot capture on test failure
- Trace recording for debugging
- HTML report generation

## Writing New Tests

1. Create a new `.spec.js` file in `tests/e2e/`
2. Import test helpers from `./helpers.js`
3. Use descriptive test names that explain the behavior being tested
4. Follow the existing pattern of using `test.beforeEach` for setup

Example:

```javascript
import { test, expect } from '@playwright/test';
import { waitForTerminal, typeCommand, terminalContains } from './helpers.js';

test.describe('My Feature', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/web/');
    await waitForTerminal(page);
  });

  test('should do something', async ({ page }) => {
    await typeCommand(page, 'my_command');
    expect(await terminalContains(page, 'EXPECTED OUTPUT')).toBe(true);
  });
});
```

## Debugging Tips

### View the browser
Run tests in headed mode to see what's happening:
```bash
npm run test:e2e:headed
```

### Step through tests
Use debug mode to pause execution:
```bash
npm run test:e2e:debug
```

### Check screenshots
Failed tests automatically capture screenshots in `test-results/`

### View traces
Open the HTML report to see detailed execution traces:
```bash
npm run test:e2e:report
```

### Test a specific file
```bash
npx playwright test navigation.spec.js
```

### Test a specific test
```bash
npx playwright test -g "CD changes directory"
```
