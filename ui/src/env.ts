import { defineEnvVars } from '@sveltejs/kit/env';

export const variables = defineEnvVars({
	BACKEND_URL: {},
	APP_VERSION: { public: true, static: true }
});