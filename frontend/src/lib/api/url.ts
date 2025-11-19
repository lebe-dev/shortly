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
		body: JSON.stringify(new RegisterUrlRequest(url))
	});

	if (response.status === 200) {
		return response.json();
	} else {
		throw new Error('unable to generate short url');
	}
}
