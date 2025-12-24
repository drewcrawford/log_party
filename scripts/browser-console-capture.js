const playwright = require('playwright');

const TARGET_URL = process.env.TARGET_URL || 'http://127.0.0.1:1334';
const BROWSER_TYPE = process.env.BROWSER || 'firefox';
const TIMEOUT_MS = parseInt(process.env.TIMEOUT_MS || '30000');
const OMIT_BROWSER_LOGS = process.env.OMIT_BROWSER_LOGS === 'true';

// Overall timeout (this is expected, not an error)
const timeoutId = setTimeout(() => {
  console.log('Timeout reached (' + TIMEOUT_MS + 'ms)');
  process.exit(0);
}, TIMEOUT_MS);

(async () => {
  const browserType = playwright[BROWSER_TYPE];
  if (!browserType) {
    console.error(`Unknown browser: ${BROWSER_TYPE}`);
    process.exit(1);
  }

  const browser = await browserType.launch({ headless: false });
  const context = await browser.newContext();
  const page = await context.newPage();

  // Capture all console messages
  page.on('console', msg => {
    if (!OMIT_BROWSER_LOGS) {
      const text = msg.text();
      const type = msg.type();
      console.log(`[${type}] ${text}`);
    }
  });

  // Capture page errors
  page.on('pageerror', error => {
    console.log(`[PAGE ERROR] ${error.message}`);
  });

  console.log(`Opening ${TARGET_URL} in ${BROWSER_TYPE}...`);

  try {
    await page.goto(TARGET_URL, { waitUntil: 'domcontentloaded', timeout: 10000 });

    // Wait for the full timeout duration, capturing console output
    await page.waitForTimeout(TIMEOUT_MS);

  } catch (error) {
    console.error('Error:', error.message);
    process.exitCode = 1;
  } finally {
    clearTimeout(timeoutId);
    browser.close().catch(() => {});
    process.exit(process.exitCode || 0);
  }
})();
