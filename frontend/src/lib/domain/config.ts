export class AppConfig {
	constructor(
		shortUrlTtl: number,
		maxUrlLength: number,
		baseUrl: string,
		features: FeaturesConfig,
		auth: AuthConfig
	) {
		this.shortUrlTtl = shortUrlTtl;
		this.maxUrlLength = maxUrlLength;
		this.baseUrl = baseUrl;
		this.features = features;
		this.auth = auth;
	}

	shortUrlTtl: number;
	maxUrlLength: number;
	baseUrl: string;
	features: FeaturesConfig;
	auth: AuthConfig;
}

export class NamedUrlsConfig {
	constructor(enabled: boolean, minLength: number, maxLength: number, reservedNames: string[]) {
		this.enabled = enabled;
		this.minLength = minLength;
		this.maxLength = maxLength;
		this.reservedNames = reservedNames;
	}

	enabled: boolean;
	minLength: number;
	maxLength: number;
	reservedNames: string[];
}

export class CreateUrlConfig {
	constructor(
		enabled: boolean,
		authOnly: boolean,
		maxPerUser: number,
		maxPerDay: number,
		currentUrls?: number,
		currentUrlsToday?: number
	) {
		this.enabled = enabled;
		this.authOnly = authOnly;
		this.maxPerUser = maxPerUser;
		this.maxPerDay = maxPerDay;
		this.currentUrls = currentUrls;
		this.currentUrlsToday = currentUrlsToday;
	}

	enabled: boolean;
	authOnly: boolean;
	maxPerUser: number;
	maxPerDay: number;
	currentUrls?: number;
	currentUrlsToday?: number;
}

export class FeaturesConfig {
	constructor(createUrl: CreateUrlConfig, namedUrls: NamedUrlsConfig) {
		this.createUrl = createUrl;
		this.namedUrls = namedUrls;
	}

	createUrl: CreateUrlConfig;
	namedUrls: NamedUrlsConfig;
}

export type AuthType = 'gitlab';

export class AuthConfig {
	constructor(enabled: boolean, authType: AuthType, note?: string) {
		this.enabled = enabled;
		this.authType = authType;
		this.note = note;
	}

	enabled: boolean;
	authType: AuthType;
	note?: string;
}
