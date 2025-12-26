import type { AuditResponse, AuditFilters } from '$lib/domain/audit';

export async function fetchAuditEvents(
	page: number = 1,
	perPage: number = 20,
	filters?: AuditFilters
): Promise<AuditResponse> {
	const params = new URLSearchParams();
	params.append('page', page.toString());
	params.append('per_page', perPage.toString());

	if (filters?.eventType) {
		params.append('event_type', filters.eventType);
	}
	if (filters?.userId) {
		params.append('user_id', filters.userId.toString());
	}
	if (filters?.urlName) {
		params.append('url_name', filters.urlName);
	}
	if (filters?.username) {
		params.append('username', filters.username);
	}
	if (filters?.dateFrom) {
		params.append('date_from', filters.dateFrom.toString());
	}
	if (filters?.dateTo) {
		params.append('date_to', filters.dateTo.toString());
	}

	const response = await fetch(`/api/admin/audit?${params.toString()}`, {
		method: 'GET',
		credentials: 'include'
	});

	if (response.status === 200) {
		const data = await response.json();
		return data as AuditResponse;
	} else if (response.status === 403) {
		throw new Error('Admin access required');
	} else if (response.status === 401) {
		throw new Error('Authentication required');
	} else {
		throw new Error(`Failed to fetch audit events: ${response.statusText}`);
	}
}
