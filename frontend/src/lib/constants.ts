/**
 * Base reserved names that are always protected regardless of configuration.
 * These names are hardcoded to prevent conflicts with system routes and endpoints.
 */
export const BASE_RESERVED_NAMES: readonly string[] = [
	'links',
	'api',
	'login',
	'logout',
	'assets',
	'static',
	'health',
	'metrics',
	'auth',
	'admin'
];
