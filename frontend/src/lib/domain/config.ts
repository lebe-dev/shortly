export class AppConfig {
	constructor(shortUrl: number) {
		this.shortUrlTtl = shortUrl;
	}

	shortUrlTtl: number;
}
