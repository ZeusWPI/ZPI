<script lang="ts">
	import ProfileSidebar from '$lib/components/profile-components/ProfileSidebar.svelte';
	import ProfileSummary from '$lib/components/profile-components/ProfileSummary.svelte';
	import ShowcaseDisplay from '$lib/components/showcases/ShowcaseDisplay.svelte';
	import Navbar from '$lib/components/Navbar.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import ProfileImage from '$lib/components/profile-components/ProfileImage.svelte';
	import type { CreateQueryResult } from '@tanstack/svelte-query';
	import { createQuery } from '@tanstack/svelte-query';

	let { username } = $props();
	let userId = 391;

	let query: CreateQueryResult;
	query = createQuery({
			queryKey: [`profile-${userId}`],
			queryFn: async () => {
				return fetch(`http://localhost:3000/api/users/${userId}`, {
					credentials: 'include'
				}).then((r) => r.json());
			}
		}
	);

</script>

{#if $query.isSuccess}
	<div class="grid grid-cols-1 md:grid-cols-4 gap-8 w-4/5 justify-center m-auto items-end">
		<div class="md:col-1 flex justify-center">
			<ProfileImage {userId} />
		</div>
		<div class="md:col-start-2 md:col-span-3">
			<ProfileSummary user={$query.data} />
		</div>
	</div>
	<div class="grid grid-cols-1 md:grid-cols-4 gap-8 w-4/5 items-start justify-center m-auto">

		<div class="md:col-1">
			<ProfileSidebar user={$query.data} />
		</div>
		<div class="md:col-start-2 md:col-span-3">
			<ShowcaseDisplay />
		</div>
	</div>
{/if}

