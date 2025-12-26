export class AppConfig {
	constructor(
		shortUrlTtl: number,
		maxUrlLength: number,
		baseUrl: string,
		features: FeaturesConfig,
		auth: AuthConfig,
		scheduler: SchedulerConfig,
		admin?: AdminDataDto
	) {
		this.shortUrlTtl = shortUrlTtl;
		this.maxUrlLength = maxUrlLength;
		this.baseUrl = baseUrl;
		this.features = features;
		this.auth = auth;
		this.scheduler = scheduler;
		this.admin = admin;
	}

	shortUrlTtl: number;
	maxUrlLength: number;
	baseUrl: string;
	features: FeaturesConfig;
	auth: AuthConfig;
	scheduler: SchedulerConfig;
	admin?: AdminDataDto;
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
	constructor(enabled: boolean, authType: AuthType, note?: string, gitlab?: GitlabProvider) {
		this.enabled = enabled;
		this.authType = authType;
		this.note = note;
		this.gitlab = gitlab;
	}

	enabled: boolean;
	authType: AuthType;
	note?: string;
	gitlab?: GitlabProvider;
}

export class GitlabProvider {
	constructor(baseUrl: string, applicationId: string) {
		this.baseUrl = baseUrl;
		this.applicationId = applicationId;
	}

	baseUrl: string;
	applicationId: string;
}

export class SchedulerConfig {
	constructor(cleanupExpiredUrls: string) {
		this.cleanupExpiredUrls = cleanupExpiredUrls;
	}

	cleanupExpiredUrls: string;
}

export interface AdminUrlDto {
	id: string;
	originalUrl: string;
	created: number;
	ttl: number;
	userId: number | null;
	username: string | null;
	customName: string | null;
}

export interface AdminUserDto {
	id: number;
	username: string;
	email?: string;
	avatarUrl?: string;
	createdAt: number;
	urlCount: number;
	maxUrlsPerUser: number;
	maxUrlsPerDay: number;
	isAdmin: boolean;
}

export interface AdminDataDto {
	allUrls: AdminUrlDto[];
	users: AdminUserDto[];
}
