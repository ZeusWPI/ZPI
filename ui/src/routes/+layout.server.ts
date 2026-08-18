import type { LayoutServerLoad } from './$types';
import { env } from '$env/dynamic/public';
import { redirect } from '@sveltejs/kit';
const BACKEND_URL = env.PUBLIC_BACKEND_URL;

export const load: LayoutServerLoad = async ({ request, fetch }) => {
	let response: Response;

	try {
		response = await fetch(`${BACKEND_URL}/api/users/me`, {
			headers: {
				cookie: request.headers.get('cookie') ?? ''
			}
		});
	} catch {
		return {
			user: null
		};
	}

	if (response.status === 401) {
		redirect(302, `${BACKEND_URL}/api/login`);
	}

	if (!response.ok) {
		return {
			user: null
		};
	}

	return {
		user: await response.json()
	};
};
