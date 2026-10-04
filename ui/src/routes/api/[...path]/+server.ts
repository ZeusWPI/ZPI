import type { RequestHandler } from './$types';
import { BACKEND_URL } from '$app/env/private';


const proxy: RequestHandler = async ({ request, params, url }) => {

	const headers = new Headers(request.headers);
	headers.set('accept-encoding', 'identity');

	const init: RequestInit & { duplex?: 'half' } = {
		method: request.method,
		headers,
		redirect: 'manual'
	};

	if (request.body) {
		init.body = request.body;
		init.duplex = 'half';
	}

	const resp = await fetch(
		`${BACKEND_URL}/api/${params.path + url.search}`,
		init
	);

	return new Response(resp.body, {
		status: resp.status,
		statusText: resp.statusText,
		headers: resp.headers
	});
};

export const GET: RequestHandler = proxy;
export const POST: RequestHandler = proxy;