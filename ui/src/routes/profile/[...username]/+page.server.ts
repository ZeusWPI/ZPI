import type { PageServerLoad } from './$types';
import { BACKEND_URL } from '$app/env/private';
import type { User } from '#lib/types.ts';
import { redirect } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ params, request }) => {

	const headers = new Headers(request.headers);
	headers.set('accept-encoding', 'identity');

	const init: RequestInit & { duplex?: 'half' } = {
		method: request.method,
		headers,
		redirect: 'manual'
	};

	const userResp = await fetch(
		`${BACKEND_URL}/api/users/${params.username}`,
		init
	);

	if (!userResp.ok) {
		console.log(userResp);
	}

	if (userResp.status === 401) {
		redirect(303, '/login');
	}


	return {
		user: await userResp.json() as User
	};
};