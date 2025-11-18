export class Url {
	constructor(id: string, originalUrl: string, ttl: number, created: number) {
		this.id = id;
		this.originalUrl = originalUrl;
		this.ttl = ttl;
		this.created = created;
	}

	id: string;
	originalUrl: string;
	ttl: number;
	created: number;
}
