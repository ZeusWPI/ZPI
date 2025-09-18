// @ts-ignore
import { env } from '$env/dynamic/public';
const BACKEND_URL = env.PUBLIC_BACKEND_URL;

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
	admin: boolean;
};

export type AchievementService = {
	id: number;
	name: string;
};

export type Achievement = {
	id: number;
	name: string;
	goals: Goal[];
};

export type Goal = {
	id: number;
	description: string;
	sequence: number;
};

export function toTitleCase(str: string) {
	if (!str) {
		return '';
	}
	return str.toLowerCase().replace(/\b\w/g, (s) => s.toUpperCase());
}

// API Query Functions //
export async function getCurrentUser(): Promise<CurrentUser> {
	const response = await fetch(BACKEND_URL + '/api/users/me', {
		credentials: 'include'
	});
	if (response.status === 401) {
		console.log('Redirecting');
		location.replace(BACKEND_URL + '/api/login');
	}
	return response.json();
}

export async function getProfile(username: string): Promise<ProfileData> {
	return fetch(`${BACKEND_URL}/api/users/${username}`, {
		credentials: 'include'
	}).then((r) => r.json());
}

export async function getBackendVersion(): Promise<String> {
	return fetch(`${BACKEND_URL}/api/version`)
		.then((r) => r.json())
		.then((r) => r.version);
}

export async function submitAbout(userId: number, about: string): Promise<Response> {
	return fetch(`${BACKEND_URL}/api/users/${userId}`, {
		credentials: 'include',
		method: 'PATCH',
		headers: {
			'Content-type': 'application/json'
		},
		body: JSON.stringify({
			about: about
		})
	});
}

export async function getAchievementServices(): Promise<AchievementService[]> {
	return fetch(`${BACKEND_URL}/api/services`, {
		credentials: 'include'
	}).then((r) => r.json());
}

export async function getAchievementsFromService(serviceId: number): Promise<Achievement[]> {
	return fetch(`${BACKEND_URL}/api/services/${serviceId}/achievements`, {
		credentials: 'include'
	}).then((r) => r.json());
}
