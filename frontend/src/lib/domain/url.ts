export class Url {
	constructor(
		id: string,
		originalUrl: string,
		ttl: number,
		created: number,
		lastAccessed?: number | null
	) {
		this.id = id;
		this.originalUrl = originalUrl;
		this.ttl = ttl;
		this.created = created;
		this.lastAccessed = lastAccessed;
	}

	id: string;
	originalUrl: string;
	ttl: number;
	created: number;
	lastAccessed?: number | null;
}
