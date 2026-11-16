import { spawn } from 'node:child_process';
import { mkdir } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';

const root = fileURLToPath(new URL('../../../', import.meta.url));

export default async function build() {
	// rust-embed requires this directory even when no frontend build is needed.
	await mkdir(new URL('../../../static/', import.meta.url), { recursive: true });
	const child = spawn('cargo', ['build', '--locked', '--bin', 'server', '--message-format=json'], {
		cwd: root,
		stdio: ['ignore', 'pipe', 'inherit']
	});
	let output = '';
	child.stdout.setEncoding('utf8').on('data', (chunk) => (output += chunk));
	await new Promise<void>((resolve, reject) => {
		child.once('error', reject);
		child.once('exit', (code) =>
			code === 0 ? resolve() : reject(new Error(`cargo build exited with ${code}`))
		);
	});
	for (const line of output.trim().split('\n')) {
		const artifact = JSON.parse(line);
		if (artifact.reason === 'compiler-artifact' && artifact.target.name === 'server') {
			process.env.SHORTLY_TEST_BINARY = artifact.executable;
		}
	}
	if (!process.env.SHORTLY_TEST_BINARY) throw new Error('Cargo did not produce the server binary');
}
