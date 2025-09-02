<script lang="ts">
	import { page } from '$app/state';
	import Profile from '$lib/components/profile-components/Profile.svelte';
	import { createQuery } from '@tanstack/svelte-query';
	import type { CreateQueryResult } from '@tanstack/svelte-query';
	import Navbar from '$lib/components/Navbar.svelte';
	import Footer from '$lib/components/Footer.svelte';

	type CurrentUser = {
		id: number,
		username: string
	}

	let username = page.params.username;

	let query: CreateQueryResult<CurrentUser>;

	query = createQuery({
			queryKey: ['currentUser'],
			queryFn: async () => {
				const response = await fetch('http://localhost:3000/api/users/me', {
					credentials: 'include'
				});
				if (response.status === 401) {
					console.log('Redirecting');
					location.replace('http://localhost:3000/api/login');
				}
				return response.json();
			}
		}
	);


</script>
{#if $query.isSuccess}

	<div class="flex flex-col min-h-screen">
		<Navbar username={$query.data.username} />

		<Profile {username} />

		<div class="flex flex-row justify-center mt-5 mb-3">
			<div class="w-5/6">
				<Footer />
			</div>
		</div>
	</div>
{/if}
