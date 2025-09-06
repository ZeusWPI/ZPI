import { PUBLIC_BACKEND_URL } from '$env/static/public';

export type Tag = {
	name: string;
	category: string;
};
export type ProfileData = {
	id: number;
	username: string;
	about: string;
	tags: Tag[];
};
export type CurrentUser = {
	id: number;
	username: string;
};

export function toTitleCase(str: string) {
	if (!str) {
		return '';
	}
	return str.toLowerCase().replace(/\b\w/g, (s) => s.toUpperCase());
}

// API Query Functions //
export async function getCurrentUser(): Promise<CurrentUser> {
	const response = await fetch(PUBLIC_BACKEND_URL + '/api/users/me', {
		credentials: 'include'
	});
	if (response.status === 401) {
		console.log('Redirecting');
		location.replace(PUBLIC_BACKEND_URL + '/api/login');
	}
	return response.json();
}

export async function getProfile(username: string): Promise<ProfileData> {
	return fetch(`${PUBLIC_BACKEND_URL}/api/users/${username}`, {
		credentials: 'include'
	}).then((r) => r.json());
}
