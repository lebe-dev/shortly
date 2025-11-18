import { error } from '@sveltejs/kit';
import type { PageLoad } from './$types';

export const load: PageLoad = ({ params }) => {
	if (params.urlId) {
		return {
			urlId: params.urlId
		};
	}

	throw error(404, 'Not found');
};
