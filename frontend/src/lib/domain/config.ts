export class AppConfig {
	constructor(shortUrl: number, maxUrlLength: number) {
		this.shortUrlTtl = shortUrl;
		this.maxUrlLength = maxUrlLength;
	}

	shortUrlTtl: number;
	maxUrlLength: number;
}
