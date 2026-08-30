import { HttpError } from './error';

export interface PasskeyCredentialDto {
	id: number;
	name: string;
	createdAt: number;
	lastUsedAt?: number;
}

interface ChallengeResponse<T> {
	challengeId: string;
	options: { publicKey: T };
}

/** Error returned by the passkey endpoints, carrying a stable code. */
export class PasskeyError extends Error {
	constructor(
		public code: string,
		message: string
	) {
		super(message);
		this.name = 'PasskeyError';
	}
}

/** Thrown when the user closes the browser dialog or no passkey is available. */
export class PasskeyCancelledError extends Error {
	constructor(message: string) {
		super(message);
		this.name = 'PasskeyCancelledError';
	}
}

export function isPasskeySupported(): boolean {
	return (
		typeof window !== 'undefined' &&
		typeof window.PublicKeyCredential !== 'undefined' &&
		typeof navigator.credentials?.create === 'function'
	);
}

function base64UrlToBytes(value: string): Uint8Array {
	const padded = value.replace(/-/g, '+').replace(/_/g, '/');
	const binary = atob(padded.padEnd(padded.length + ((4 - (padded.length % 4)) % 4), '='));
	const bytes = new Uint8Array(binary.length);

	for (let i = 0; i < binary.length; i++) {
		bytes[i] = binary.charCodeAt(i);
	}

	return bytes;
}

function bytesToBase64Url(buffer: ArrayBuffer): string {
	const bytes = new Uint8Array(buffer);
	let binary = '';

	for (let i = 0; i < bytes.length; i++) {
		binary += String.fromCharCode(bytes[i]);
	}

	return btoa(binary).replace(/\+/g, '-').replace(/\//g, '_').replace(/=+$/, '');
}

/** The server encodes every binary field as base64url, the browser wants buffers. */
function toCreationOptions(
	options: PublicKeyCredentialCreationOptionsJSON
): PublicKeyCredentialCreationOptions {
	return {
		...options,
		challenge: base64UrlToBytes(options.challenge),
		user: {
			...options.user,
			id: base64UrlToBytes(options.user.id)
		},
		excludeCredentials: (options.excludeCredentials ?? []).map((credential) => ({
			...credential,
			id: base64UrlToBytes(credential.id)
		}))
	} as unknown as PublicKeyCredentialCreationOptions;
}

function toRequestOptions(
	options: PublicKeyCredentialRequestOptionsJSON
): PublicKeyCredentialRequestOptions {
	return {
		...options,
		challenge: base64UrlToBytes(options.challenge),
		allowCredentials: (options.allowCredentials ?? []).map((credential) => ({
			...credential,
			id: base64UrlToBytes(credential.id)
		}))
	} as unknown as PublicKeyCredentialRequestOptions;
}

interface PublicKeyCredentialDescriptorJSON {
	id: string;
	type: string;
	transports?: string[];
}

interface PublicKeyCredentialCreationOptionsJSON {
	challenge: string;
	user: { id: string; name: string; displayName: string };
	excludeCredentials?: PublicKeyCredentialDescriptorJSON[];
	[key: string]: unknown;
}

interface PublicKeyCredentialRequestOptionsJSON {
	challenge: string;
	allowCredentials?: PublicKeyCredentialDescriptorJSON[];
	[key: string]: unknown;
}

function serializeRegistration(credential: PublicKeyCredential) {
	const response = credential.response as AuthenticatorAttestationResponse;

	return {
		id: credential.id,
		rawId: bytesToBase64Url(credential.rawId),
		type: credential.type,
		response: {
			attestationObject: bytesToBase64Url(response.attestationObject),
			clientDataJSON: bytesToBase64Url(response.clientDataJSON),
			transports:
				typeof response.getTransports === 'function' ? response.getTransports() : undefined
		},
		clientExtensionResults: credential.getClientExtensionResults()
	};
}

function serializeAuthentication(credential: PublicKeyCredential) {
	const response = credential.response as AuthenticatorAssertionResponse;

	return {
		id: credential.id,
		rawId: bytesToBase64Url(credential.rawId),
		type: credential.type,
		response: {
			authenticatorData: bytesToBase64Url(response.authenticatorData),
			clientDataJSON: bytesToBase64Url(response.clientDataJSON),
			signature: bytesToBase64Url(response.signature),
			userHandle: response.userHandle ? bytesToBase64Url(response.userHandle) : undefined
		},
		clientExtensionResults: credential.getClientExtensionResults()
	};
}

async function readError(response: Response): Promise<never> {
	try {
		const body = await response.json();

		if (body?.code) {
			throw new PasskeyError(body.code, body.message ?? response.statusText);
		}
	} catch (e) {
		if (e instanceof PasskeyError) {
			throw e;
		}
	}

	throw new HttpError(response.status, response.statusText);
}

async function postJson<T>(url: string, body?: unknown): Promise<T> {
	const response = await fetch(url, {
		method: 'POST',
		credentials: 'include',
		headers: { 'Content-Type': 'application/json' },
		body: body === undefined ? undefined : JSON.stringify(body)
	});

	if (!response.ok) {
		return readError(response);
	}

	return response.json();
}

/** Log in with a passkey. Returns the username of the account that was entered. */
export async function loginWithPasskey(): Promise<string> {
	const start = await postJson<ChallengeResponse<PublicKeyCredentialRequestOptionsJSON>>(
		'/api/auth/passkey/login/start'
	);

	const credential = await requestCredential(toRequestOptions(start.options.publicKey));

	const result = await postJson<{ username: string }>('/api/auth/passkey/login/finish', {
		challengeId: start.challengeId,
		credential: serializeAuthentication(credential)
	});

	return result.username;
}

/** Register a new passkey for the current user. */
export async function registerPasskey(name: string): Promise<PasskeyCredentialDto> {
	const start = await postJson<ChallengeResponse<PublicKeyCredentialCreationOptionsJSON>>(
		'/api/user/passkeys/register/start'
	);

	const credential = await createCredential(toCreationOptions(start.options.publicKey));

	return postJson<PasskeyCredentialDto>('/api/user/passkeys/register/finish', {
		challengeId: start.challengeId,
		name,
		credential: serializeRegistration(credential)
	});
}

async function createCredential(
	publicKey: PublicKeyCredentialCreationOptions
): Promise<PublicKeyCredential> {
	try {
		const credential = (await navigator.credentials.create({
			publicKey
		})) as PublicKeyCredential | null;

		if (!credential) {
			throw new PasskeyCancelledError('No passkey was created');
		}

		return credential;
	} catch (e) {
		throw toBrowserError(e);
	}
}

async function requestCredential(
	publicKey: PublicKeyCredentialRequestOptions
): Promise<PublicKeyCredential> {
	try {
		const credential = (await navigator.credentials.get({
			publicKey
		})) as PublicKeyCredential | null;

		if (!credential) {
			throw new PasskeyCancelledError('No passkey was chosen');
		}

		return credential;
	} catch (e) {
		throw toBrowserError(e);
	}
}

function toBrowserError(e: unknown): Error {
	if (e instanceof PasskeyCancelledError || e instanceof PasskeyError) {
		return e;
	}

	if (e instanceof DOMException && (e.name === 'NotAllowedError' || e.name === 'AbortError')) {
		return new PasskeyCancelledError(e.message);
	}

	return e instanceof Error ? e : new Error(String(e));
}

export async function getPasskeys(): Promise<PasskeyCredentialDto[]> {
	const response = await fetch('/api/user/passkeys', {
		method: 'GET',
		credentials: 'include'
	});

	if (!response.ok) {
		return readError(response);
	}

	return response.json();
}

export async function deletePasskey(credentialId: number): Promise<void> {
	const response = await fetch(`/api/user/passkeys/${credentialId}`, {
		method: 'DELETE',
		credentials: 'include'
	});

	if (!response.ok) {
		await readError(response);
	}
}

/** Delete every passkey of a user. Administrators only. */
export async function deleteUserPasskeys(userId: number): Promise<number> {
	const response = await fetch(`/api/admin/user/${userId}/passkeys`, {
		method: 'DELETE',
		credentials: 'include'
	});

	if (!response.ok) {
		return readError(response);
	}

	const body = await response.json();

	return body.deleted;
}
