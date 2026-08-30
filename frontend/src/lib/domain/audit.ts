export enum AuditEventType {
	CreateUrl = 'create_url',
	DeleteUrl = 'delete_url',
	UserLogin = 'user_login',
	UserLogout = 'user_logout',
	UserQuotaUpdate = 'user_quota_update',
	PasskeyRegister = 'passkey_register',
	PasskeyDelete = 'passkey_delete'
}

export interface AuditEvent {
	id: number;
	eventType: AuditEventType;
	actorUserId: number;
	actorUsername: string;
	targetUserId: number;
	targetUsername: string;
	urlName: string | null;
	createdAt: number; // Unix timestamp in seconds
}

export interface AuditResponse {
	events: AuditEvent[];
	totalCount: number;
	page: number;
	perPage: number;
	totalPages: number;
}

export interface AuditFilters {
	eventType?: AuditEventType;
	userId?: number;
	urlName?: string;
	username?: string;
	dateFrom?: number; // Unix timestamp in seconds
	dateTo?: number; // Unix timestamp in seconds
}
