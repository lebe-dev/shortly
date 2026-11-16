import type { APIRequestContext } from '@playwright/test';
import { test, expect, type StoredLink } from './fixtures';

async function visible(client: APIRequestContext, link: StoredLink) {
	for (const key of [link.custom_name, link.custom_name?.toUpperCase(), link.id].filter(Boolean)) {
		const details = await client.get(`/api/url/${key}`);
		expect(details.status()).toBe(200);
		expect((await details.json()).url).toBe(link.original_url);
		expect(details.headers()['cache-control']).toBe('private, no-store');
		const redirect = await client.get(`/${key}`, { maxRedirects: 0 });
		expect(redirect.status()).toBe(302);
		expect(redirect.headers().location).toBe(link.original_url);
		expect(redirect.headers()['cache-control']).toBe('private, no-store');
	}
}

async function hidden(client: APIRequestContext, link: StoredLink) {
	for (const method of ['GET', 'HEAD']) {
		for (const prefix of ['/api/url/', '/']) {
			const missing = await client.fetch(`${prefix}missing-integration-link`, {
				method,
				maxRedirects: 0
			});
			expect(missing.status()).toBe(prefix === '/' ? 200 : 400);
			for (const key of [link.custom_name, link.custom_name?.toUpperCase(), link.id].filter(
				Boolean
			)) {
				const denied = await client.fetch(`${prefix}${key}`, { method, maxRedirects: 0 });
				expect(denied.status()).toBe(missing.status());
				expect(await denied.body()).toEqual(await missing.body());
				expect(denied.headers().location).toBeUndefined();
				expect(denied.headers()['cache-control']).toBe('private, no-store');
				expect(await denied.text()).not.toContain(link.original_url);
				expect(await denied.text()).not.toContain('"access"');
			}
		}
	}
}

test('corporate names resolve inside and look missing outside, including to owner and admin', async ({
	app
}) => {
	const link = await app.create('Corporate-Docs');
	expect(link.access).toBe('internal');
	// Wait for the next timestamp second so an accidental update cannot pass unnoticed.
	await expect.poll(() => Math.floor(Date.now() / 1000)).toBeGreaterThan(link.last_accessed!);
	await hidden(app.external, link);
	await hidden(app.owner, link);
	await hidden(app.admin, link);
	// Denied requests must not count as visits.
	expect((await app.links())[0].last_accessed).toBe(link.last_accessed);
	await visible(app.internal, link);
	await expect
		.poll(async () => (await app.links())[0].last_accessed)
		.toBeGreaterThan(link.last_accessed!);
	const adminConfig = await (await app.admin.get('/api/config')).json();
	expect(adminConfig.admin.allUrls.some((entry: { id: string }) => entry.id === link.id)).toBe(
		true
	);
});

test('GET and HEAD apply the same policy and redirects cannot be cached', async ({ app }) => {
	const link = await app.create('Head-Docs');
	for (const path of [`/api/url/${link.id}`, `/${link.custom_name}`, `/${link.id}`]) {
		const allowed = await app.internal.head(path, { maxRedirects: 0 });
		expect(allowed.status()).toBe(path.startsWith('/api/') ? 200 : 302);
		expect(allowed.headers()['cache-control']).toBe('private, no-store');
		const denied = await app.external.head(path, { maxRedirects: 0 });
		expect(denied.headers().location).toBeUndefined();
		expect(denied.headers()['cache-control']).toBe('private, no-store');
	}
});

test('generated URLs remain public with a corporate default', async ({ app }) => {
	const link = await app.create();
	expect(link.access).toBe('public');
	await visible(app.external, link);
	await visible(app.internal, link);
});

test('creation requires a session and cannot override corporate policy', async ({ app }) => {
	const anonymous = await app.internal.post('/api/url', {
		data: { url: 'https://destination.invalid', name: 'Anonymous-Docs' }
	});
	expect(anonymous.status()).toBe(401);
	const override = await app.owner.post('/api/url', {
		data: { url: 'https://destination.invalid', name: 'Public-Override', access: 'public' }
	});
	expect(override.status()).toBe(400);
	expect(await app.links()).toHaveLength(0);
	const config = await (await app.external.get('/api/config')).json();
	expect(config.features.namedUrls).toMatchObject({
		defaultAccess: 'internal',
		allowAccessOverride: false
	});
});

test('stored policy survives a restart and a change of the creation default', async ({ app }) => {
	const corporate = await app.create('Persistent-Corporate');
	await app.restart({ NAMED_URL_DEFAULT_ACCESS: 'public' });
	const publicLink = await app.create('New-Public');
	await hidden(app.external, corporate);
	await visible(app.internal, corporate);
	await visible(app.external, publicLink);
});

test('removing corporate CIDRs revokes access to existing internal links', async ({ app }) => {
	const link = await app.create('Revoked-Docs');
	await visible(app.internal, link);
	await app.restart({ NAMED_URL_DEFAULT_ACCESS: 'public', INTERNAL_NETWORK_CIDRS: '' });
	await hidden(app.internal, link);
	await hidden(app.external, link);
});

test('client headers cannot bypass the external edge proxy', async ({ app }) => {
	const link = await app.create('Spoof-Protected');
	const forgedHeaders: Record<string, string>[] = [
		{ 'x-forwarded-for': '203.0.113.10' },
		{ 'x-forwarded-for': '203.0.113.10, 192.0.2.5' },
		{ 'x-real-ip': '203.0.113.10', forwarded: 'for=203.0.113.10' }
	];
	for (const headers of forgedHeaders) {
		const response = await app.external.get(`/api/url/${link.custom_name}`, { headers });
		expect(response.status()).toBe(400);
		expect(await response.text()).not.toContain(link.original_url);
		const redirect = await app.external.get(`/${link.id}`, { headers, maxRedirects: 0 });
		expect(redirect.headers().location).toBeUndefined();
	}
});

for (const [label, chain] of [
	['IPv4 corporate client', '203.0.113.10'],
	['IPv6 corporate client', '2001:db8:1234::10'],
	['IPv4-mapped IPv6 client', '::ffff:203.0.113.10'],
	['several trusted proxies', '203.0.113.10, 192.0.2.5, 192.0.2.6']
]) {
	test(`trusted forwarding accepts ${label}`, async ({ app }) => {
		const link = await app.create('Forwarded-Docs');
		await visible(await app.forwarded(chain), link);
	});
}

for (const [label, chain] of [
	['missing header', undefined],
	['empty header', ''],
	['malformed address', '203.0.113.10, garbage'],
	['trailing comma', '203.0.113.10,'],
	['only trusted proxies', '127.0.0.1, 192.0.2.5'],
	['overlong chain', Array(33).fill('203.0.113.10').join(',')],
	['spoofed corporate prefix', '203.0.113.10, 198.51.100.10, 192.0.2.5'],
	['IPv6 outside corporate CIDRs', '2001:db8:5678::10']
]) {
	test(`trusted forwarding denies ${label}`, async ({ app }) => {
		const link = await app.create('Denied-Forwarding');
		await hidden(await app.forwarded(chain), link);
	});
}

test.describe('without trusted proxies', () => {
	test.use({ settings: { TRUSTED_PROXY_CIDRS: '' } });
	test('TCP peer wins over forged forwarding headers', async ({ app }) => {
		const link = await app.create('Direct-Protected');
		const client = await app.forwarded('203.0.113.10');
		await hidden(client, link);
		await visible(client, await app.create());
	});
});

test.describe('proxy address overlaps corporate CIDRs', () => {
	test.use({ settings: { INTERNAL_NETWORK_CIDRS: '203.0.113.0/24,127.0.0.1/32' } });
	test('invalid forwarding never falls back to the trusted proxy IP', async ({ app }) => {
		const link = await app.create('Fail-Closed-Proxy');
		await visible(app.internal, link);
		for (const chain of [undefined, 'garbage', '127.0.0.1,192.0.2.5']) {
			await hidden(await app.forwarded(chain), link);
		}
	});
});

test.describe('direct corporate TCP connection', () => {
	test.use({ settings: { TRUSTED_PROXY_CIDRS: '', INTERNAL_NETWORK_CIDRS: '127.0.0.1/32' } });
	test('explicitly allowed loopback peer works and ignores external forwarding headers', async ({
		app
	}) => {
		await visible(await app.forwarded('198.51.100.10'), await app.create('Direct-Corporate'));
	});
});

test.describe('access override enabled', () => {
	test.use({
		settings: { NAMED_URL_DEFAULT_ACCESS: 'public', NAMED_URL_ALLOW_ACCESS_OVERRIDE: 'true' }
	});
	test('per-link public and internal modes are stored and enforced', async ({ app }) => {
		const corporate = await app.create('Explicit-Corporate', 'internal');
		const publicLink = await app.create('Explicit-Public', 'public');
		await hidden(app.external, corporate);
		await visible(app.internal, corporate);
		await visible(app.external, publicLink);
		const generated = await app.owner.post('/api/url', {
			data: { url: 'https://destination.invalid', access: 'internal' }
		});
		expect(generated.status()).toBe(400);
	});
});
