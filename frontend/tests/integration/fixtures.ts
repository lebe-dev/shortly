import { test as base, expect, request, type APIRequestContext } from '@playwright/test';
import { spawn, type ChildProcess } from 'node:child_process';
import { once } from 'node:events';
import { mkdtemp, rm, writeFile } from 'node:fs/promises';
import { createServer, request as httpRequest, type Server } from 'node:http';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { setTimeout as delay } from 'node:timers/promises';

type Access = 'public' | 'internal';
type Settings = Record<string, string>;
export type StoredLink = {
	id: string;
	custom_name?: string;
	access: Access;
	original_url: string;
	last_accessed?: number;
};

const defaults: Settings = {
	NAMED_URL_DEFAULT_ACCESS: 'internal',
	NAMED_URL_ALLOW_ACCESS_OVERRIDE: 'false',
	INTERNAL_NETWORK_CIDRS: '203.0.113.0/24,2001:db8:1234::/48',
	TRUSTED_PROXY_CIDRS: '127.0.0.1/32,192.0.2.0/24',
	NAMED_URL_CHECK_RATE_LIMIT: '1000',
	PASSKEY_ENABLED: 'false'
};

async function listen(server: Server): Promise<string> {
	await new Promise<void>((resolve, reject) => {
		server.once('error', reject);
		server.listen(0, '127.0.0.1', resolve);
	});
	const address = server.address();
	if (!address || typeof address === 'string') throw new Error('Expected a TCP listener');
	return `http://127.0.0.1:${address.port}`;
}

async function close(server: Server) {
	if (!server.listening) return;
	await new Promise<void>((resolve, reject) => {
		server.close((error) => (error ? reject(error) : resolve()));
		server.closeAllConnections();
	});
}

export class IntegrationEnvironment {
	internal!: APIRequestContext;
	external!: APIRequestContext;
	owner!: APIRequestContext;
	admin!: APIRequestContext;
	raw!: APIRequestContext;
	logs = '';
	private directory = '';
	private backend = '';
	private child?: ChildProcess;
	private servers: Server[] = [];
	private contexts: APIRequestContext[] = [];
	private settings: Settings;
	private internalURL = '';
	private externalURL = '';
	private oauthURL = '';

	constructor(settings: Settings) {
		this.settings = { ...defaults, ...settings };
	}

	async start() {
		this.directory = await mkdtemp(join(tmpdir(), 'shortly-playwright-'));
		// Prevent dotenv from discovering any developer configuration in parent directories.
		await writeFile(join(this.directory, '.env'), '');
		const oauth = createServer((req, res) => {
			void (async () => {
				res.setHeader('content-type', 'application/json');
				if (req.method === 'POST' && req.url === '/oauth/token') {
					let body = '';
					for await (const chunk of req) body += chunk;
					const code = new URLSearchParams(body).get('code');
					if (code === 'owner' || code === 'admin') {
						res.end(JSON.stringify({ access_token: code }));
						return;
					}
				}
				if (req.method === 'GET' && req.url === '/api/v4/user') {
					const role = req.headers.authorization?.replace('Bearer ', '');
					if (role === 'owner' || role === 'admin') {
						res.end(
							JSON.stringify({ id: role === 'owner' ? 1 : 2, username: `integration-${role}` })
						);
						return;
					}
				}
				res.writeHead(400).end('{}');
			})().catch(() => res.destroy());
		});
		this.servers.push(oauth);
		this.oauthURL = await listen(oauth);
		this.internalURL = await this.proxy('203.0.113.10');
		this.externalURL = await this.proxy('198.51.100.10');
		await this.restart();
		this.internal = await this.client(this.internalURL);
		this.external = await this.client(this.externalURL);
		this.owner = await this.client(this.externalURL);
		this.admin = await this.client(this.externalURL);
		await this.login(this.owner, 'owner');
		await this.login(this.admin, 'admin');
	}

	private async proxy(ip: string) {
		const server = createServer((req, res) => {
			// Append the verified client after any supplied prefix, as an edge proxy does.
			const supplied = req.headers['x-forwarded-for'];
			const upstream = httpRequest(
				`${this.backend}${req.url}`,
				{
					method: req.method,
					headers: {
						...req.headers,
						host: new URL(this.backend).host,
						'x-forwarded-for': supplied ? `${supplied}, ${ip}` : ip,
						'x-real-ip': ip
					}
				},
				(response) => {
					res.writeHead(response.statusCode ?? 502, response.headers);
					response.pipe(res);
				}
			);
			upstream.on('error', () => res.writeHead(502).end());
			req.pipe(upstream);
		});
		this.servers.push(server);
		return listen(server);
	}

	async client(baseURL: string, headers: Record<string, string> = {}) {
		const context = await request.newContext({ baseURL, extraHTTPHeaders: headers, timeout: 5000 });
		this.contexts.push(context);
		return context;
	}

	async forwarded(chain?: string) {
		return this.client(this.backend, chain === undefined ? {} : { 'x-forwarded-for': chain });
	}

	private async login(client: APIRequestContext, role: 'owner' | 'admin') {
		const login = await client.get('/api/auth/login', { maxRedirects: 0 });
		expect(login.status()).toBe(307);
		const auth = new URL(login.headers().location);
		expect(auth.origin).toBe(this.oauthURL);
		const callback = await client.get('/api/auth/callback', {
			params: { code: role, state: auth.searchParams.get('state')! },
			maxRedirects: 0
		});
		expect(callback.status()).toBe(302);
		expect(callback.headers()['set-cookie']).toContain('session_token=');
	}

	async create(name?: string, access?: Access): Promise<StoredLink> {
		const response = await this.owner.post('/api/url', {
			data: { url: 'https://destination.invalid/confidential', name, access }
		});
		expect(response.status(), await response.text()).toBe(200);
		const created = await response.json();
		expect(created.access).toBe(
			access ?? (name ? this.settings.NAMED_URL_DEFAULT_ACCESS : 'public')
		);
		const links = await this.links();
		const link = links.find((link) =>
			name ? link.custom_name?.toLowerCase() === name.toLowerCase() : !link.custom_name
		);
		expect(link).toBeDefined();
		expect(new URL(created.url).pathname).toBe(`/${link!.custom_name ?? link!.id}`);
		return link!;
	}

	async links(): Promise<StoredLink[]> {
		const response = await this.owner.get('/api/user/urls');
		expect(response.status()).toBe(200);
		return response.json();
	}

	async restart(settings: Settings = {}) {
		await this.stopBackend();
		this.settings = { ...this.settings, ...settings };
		// Reserve an OS-assigned port. Retry if another process claims it before Rust binds.
		for (let attempt = 0; attempt < 3; attempt++) {
			const reservation = createServer();
			this.backend = await listen(reservation);
			await close(reservation);
			const config = {
				bind: new URL(this.backend).host,
				'base-url': this.internalURL,
				'db-cnn': 'sqlite://./test.db?mode=rwc',
				'log-level': 'warn',
				'log-target': 'console',
				'short-url': { ttl: 168, 'max-length': 2048 },
				scheduler: { 'cleanup-expired-urls': '0 0 0 1 1 *' },
				features: {
					'create-url': {
						enabled: true,
						'auth-only': true,
						'max-per-user': 100,
						'max-per-day': 10
					},
					'named-urls': { enabled: true, 'min-length': 3, 'max-length': 32, 'reserved-names': [] }
				},
				auth: {
					enabled: true,
					type: 'gitlab',
					'admin-users': 'integration-admin',
					providers: {
						gitlab: {
							'base-url': this.oauthURL,
							'application-id': 'integration',
							secret: 'integration'
						}
					}
				},
				metrics: { enabled: false }
			};
			await writeFile(join(this.directory, 'config.yml'), JSON.stringify(config));
			const binary = process.env.SHORTLY_TEST_BINARY;
			if (!binary) throw new Error('Missing server binary; run through Playwright global setup');
			// Inherit OS essentials only: app ENV, proxy settings and real credentials stay out.
			const env: NodeJS.ProcessEnv = { ...this.settings };
			for (const key of ['PATH', 'SystemRoot', 'WINDIR', 'TMPDIR', 'TMP', 'TEMP']) {
				if (process.env[key]) env[key] = process.env[key];
			}
			this.child = spawn(binary, { cwd: this.directory, env, stdio: ['ignore', 'pipe', 'pipe'] });
			const child = this.child;
			let spawnError: Error | undefined;
			child.once('error', (error) => (spawnError = error));
			for (const stream of [child.stdout, child.stderr]) {
				stream?.setEncoding('utf8').on('data', (chunk) => {
					this.logs = (this.logs + chunk).slice(-64_000);
				});
			}
			const probe = await request.newContext({ baseURL: this.backend, timeout: 500 });
			try {
				const deadline = Date.now() + 10_000;
				while (Date.now() < deadline && child.exitCode === null && !spawnError) {
					try {
						if ((await probe.get('/api/health')).status() === 200) {
							this.raw = await this.client(this.backend);
							return;
						}
					} catch {
						/* Server is still starting. */
					}
					await delay(50);
				}
			} finally {
				await probe.dispose();
			}
			await this.stopBackend();
			if (spawnError) throw spawnError;
		}
		throw new Error(`Server failed to start:\n${this.logs}`);
	}

	private async stopBackend() {
		const child = this.child;
		if (!child || child.exitCode !== null || child.signalCode !== null) return;
		const exited = once(child, 'exit');
		child.kill('SIGTERM');
		const timer = setTimeout(() => child.kill('SIGKILL'), 3000);
		try {
			await exited;
		} finally {
			clearTimeout(timer);
		}
	}

	async dispose() {
		try {
			await Promise.all(this.contexts.map((context) => context.dispose()));
		} finally {
			await this.stopBackend();
			await Promise.all(this.servers.map(close));
			if (this.directory) await rm(this.directory, { recursive: true, force: true });
		}
	}
}

export const test = base.extend<{ settings: Settings; app: IntegrationEnvironment }>({
	settings: [{}, { option: true }],
	app: async ({ settings }, use, testInfo) => {
		const app = new IntegrationEnvironment(settings);
		try {
			await app.start();
			await use(app);
		} finally {
			if (testInfo.status !== testInfo.expectedStatus) {
				await testInfo.attach('server.log', { body: app.logs, contentType: 'text/plain' });
			}
			await app.dispose();
		}
	}
});

export { expect };
