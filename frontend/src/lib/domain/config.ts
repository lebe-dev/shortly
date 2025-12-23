export class AppConfig {
	constructor(
		shortUrlTtl: number,
		maxUrlLength: number,
		features: FeaturesConfig,
		auth: AuthConfig
	) {
		this.shortUrlTtl = shortUrlTtl;
		this.maxUrlLength = maxUrlLength;
		this.features = features;
		this.auth = auth;
	}

	shortUrlTtl: number;
	maxUrlLength: number;
	features: FeaturesConfig;
	auth: AuthConfig;
}

export class FeaturesConfig {
	constructor(createUrlEnabled: boolean, createUrlAuthOnly: boolean) {
		this.createUrlEnabled = createUrlEnabled;
		this.createUrlAuthOnly = createUrlAuthOnly;
	}

	createUrlEnabled: boolean;
	createUrlAuthOnly: boolean;
}

export type AuthType = 'gitlab';

export class AuthConfig {
	constructor(enabled: boolean, authType: AuthType) {
		this.enabled = enabled;
		this.authType = authType;
	}

	enabled: boolean;
	authType: AuthType;
}
