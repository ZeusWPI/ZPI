<script lang="ts">
	import Profile from '$lib/components/Profile.svelte';
	import { createQuery } from '@tanstack/svelte-query';
	import type { CreateQueryResult } from '@tanstack/svelte-query';
	import Navbar from '$lib/components/Navbar.svelte';
	import Footer from '$lib/components/Footer.svelte';

	type CurrentUser = {
		id: number,
		username: string
	}

	let query: CreateQueryResult<CurrentUser>;

	query = createQuery({
			queryKey: ['currentUser'],
			queryFn: async () => {
				const response = await fetch('http://localhost:3000/users/me', {
					credentials: 'include'
				});
				if (response.status === 401) {
					console.log('Redirecting');
					location.replace('http://localhost:3000');
				}
				return response.json();
			}
		}
	);
</script>
{#if $query.isSuccess}

	<div class="flex flex-col min-h-screen">
		<Navbar username={$query.data.username} />

		<Profile userId="385" />

		<div class="flex flex-row justify-center mt-5 mb-3">
			<div class="w-5/6">
				<Footer />
			</div>
		</div>
	</div>
{/if}
