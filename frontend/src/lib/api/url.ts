import { HttpError } from './error';

export class RegisterUrlRequest {
	constructor(url: string, name?: string) {
		this.url = url;
		this.name = name;
	}

	url: string;
	name?: string;
}

export class RegisterUrlResponse {
	constructor(url: string) {
		this.url = url;
	}

	url: string;
}

export class UrlDetailsResponse {
	constructor(url: string, ttl: number, created: number) {
		this.url = url;
		this.ttl = ttl;
		this.created = created;
	}

	url: string;
	/** TTL in seconds */
	ttl: number;
	/** Creation timestamp in seconds */
	created: number;
}

export async function fetchUrlById(urlId: string): Promise<UrlDetailsResponse> {
	const response = await fetch(`/api/url/${urlId}`, {
		method: 'GET'
	});

	if (response.status === 200) {
		return response.json();
	} else if (response.status === 400) {
		throw new HttpError(response.status, 'Not found');
	} else if (response.status !== 0) {
		throw new HttpError(response.status, response.statusText);
	} else if (response.status == 0) {
		throw new HttpError(response.status, 'Network error');
	} else {
		throw new Error('fetch error');
	}
}

export async function generateShortUrl(url: string, name?: string): Promise<RegisterUrlResponse> {
	const response = await fetch('/api/url', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify(new RegisterUrlRequest(url, name)),
		credentials: 'include'
	});

	if (response.status === 200) {
		return response.json();
	} else if (response.status === 400) {
		const data = await response.json().catch(() => ({ error: 'Invalid request' }));
		throw new Error(data.error || 'Invalid request');
	} else if (response.status === 409) {
		throw new Error('Custom name already exists or is reserved');
	} else if (response.status === 429) {
		throw new Error('Rate limit exceeded');
	} else if (response.status === 401) {
		throw new Error('Authentication required for named URLs');
	} else {
		throw new Error('Unable to generate short url');
	}
}

export interface CheckNameResponse {
	available: boolean;
}

export async function checkCustomName(name: string): Promise<boolean> {
	const response = await fetch(`/api/url/check?name=${encodeURIComponent(name)}`, {
		method: 'GET'
	});

	if (response.status === 200) {
		const data: CheckNameResponse = await response.json();
		return data.available;
	} else if (response.status === 409) {
		return false;
	} else {
		throw new Error('Failed to check name availability');
	}
}

export class UserUrlResponse {
	id: string;
	url: string;
	original_url: string;
	created: number;
	ttl: number;
	custom_name?: string;

	constructor(
		id: string,
		url: string,
		original_url: string,
		created: number,
		ttl: number,
		custom_name?: string
	) {
		this.id = id;
		this.url = url;
		this.original_url = original_url;
		this.created = created;
		this.ttl = ttl;
		this.custom_name = custom_name;
	}
}

export async function getUserUrls(): Promise<UserUrlResponse[]> {
	const response = await fetch('/api/user/urls', {
		method: 'GET',
		credentials: 'include'
	});

	if (response.status === 200) {
		return response.json();
	} else if (response.status === 401) {
		throw new HttpError(response.status, 'Unauthorized');
	} else {
		throw new HttpError(response.status, response.statusText);
	}
}

export async function deleteUrl(urlId: string): Promise<void> {
	const response = await fetch(`/api/url/${urlId}`, {
		method: 'DELETE',
		credentials: 'include'
	});

	if (response.status === 204) {
		return;
	} else if (response.status === 404) {
		throw new HttpError(response.status, 'Not found');
	} else if (response.status === 403) {
		throw new HttpError(response.status, 'Forbidden');
	} else {
		throw new HttpError(response.status, response.statusText);
	}
}
