import type { LayoutServerLoad } from './$types';
import { BACKEND_URL } from '$app/env/private';

export const load: LayoutServerLoad = async ({ request, fetch }) => {
	let response: Response;

	try {
		response = await fetch(`${BACKEND_URL}/api/version`);
	} catch {
		return {
			version: null
		};
	}

	if (!response.ok) {
		return {
			version: null
		};
	}

	let version = await response.json() as { version: string };
	return {
		backendVersion: version
	};
};
