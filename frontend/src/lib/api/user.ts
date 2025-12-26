import { HttpError } from './error';

export interface UpdateUserQuotasRequest {
	maxUrlsPerUser?: number;
	maxUrlsPerDay?: number;
}

export interface UpdateUserQuotasResponse {
	id: number;
	username: string;
	maxUrlsPerUser: number;
	maxUrlsPerDay: number;
	updatedAt: number;
}

export async function updateUserQuotas(
	userId: number,
	quotas: UpdateUserQuotasRequest
): Promise<UpdateUserQuotasResponse> {
	const response = await fetch(`/api/user/${userId}/quotas`, {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify(quotas)
	});

	if (response.status === 200) {
		return response.json();
	} else if (response.status === 403) {
		throw new HttpError(response.status, 'Admin access required');
	} else if (response.status === 404) {
		throw new HttpError(response.status, 'User not found');
	} else if (response.status === 400) {
		const error = await response.json();
		throw new HttpError(response.status, error.message || 'Invalid request');
	} else if (response.status !== 0) {
		throw new HttpError(response.status, response.statusText);
	} else if (response.status == 0) {
		throw new HttpError(response.status, 'Network error');
	} else {
		throw new Error('fetch error');
	}
}
