import { HttpError } from './error';

export class RegisterUrlRequest {
	constructor(url: string) {
		this.url = url;
	}

	url: string;
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

export async function generateShortUrl(url: string): Promise<RegisterUrlResponse> {
	const response = await fetch('/api/url', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify(new RegisterUrlRequest(url)),
		credentials: 'include'
	});

	if (response.status === 200) {
		return response.json();
	} else {
		throw new Error('unable to generate short url');
	}
}

export class UserUrlResponse {
	id: string;
	url: string;
	original_url: string;
	created: number;
	ttl: number;

	constructor(id: string, url: string, original_url: string, created: number, ttl: number) {
		this.id = id;
		this.url = url;
		this.original_url = original_url;
		this.created = created;
		this.ttl = ttl;
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
