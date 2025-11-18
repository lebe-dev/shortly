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

export async function fetchUrlById(urlId: string): Promise<RegisterUrlResponse> {
	const response = await fetch(`/api/url/${urlId}`, {
		method: 'GET'
	});

	if (response.status === 200) {
		return response.json();
	} else {
		throw new Error('config load error');
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
