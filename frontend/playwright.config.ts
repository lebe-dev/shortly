import { defineConfig } from '@playwright/test';
import { fileURLToPath } from 'node:url';

export default defineConfig({
	testDir: './tests/integration',
	globalSetup: './tests/integration/build.ts',
	outputDir: fileURLToPath(new URL('../target/playwright/results', import.meta.url)),
	fullyParallel: true,
	forbidOnly: Boolean(process.env.CI),
	workers: 4,
	timeout: 30_000,
	retries: 0,
	reporter: 'list',
	use: { trace: 'retain-on-failure' }
});
