<script lang="ts">
	import ProfileSidebar from '$lib/components/profile-components/ProfileSidebar.svelte';
	import ProfileSummary from '$lib/components/profile-components/ProfileSummary.svelte';
	import ShowcaseDisplay from '$lib/components/showcases/ShowcaseDisplay.svelte';
	import ProfileImage from '$lib/components/profile-components/ProfileImage.svelte';
	import type { CreateQueryResult } from '@tanstack/svelte-query';
	import { createQuery } from '@tanstack/svelte-query';
	import { PUBLIC_BACKEND_URL } from '$env/static/public';

	let { username } = $props();

	type ProfileData = {
		id: number,
		username: string,
		about: string,
		tags: any //TODO Update Tag Type
	}

	let query: CreateQueryResult<ProfileData>;
	query = createQuery({
			queryKey: [`profile-${username}`],
			queryFn: async () => {
				return fetch(`${PUBLIC_BACKEND_URL}/api/users/${username}`, {
					credentials: 'include'
				}).then((r) => r.json());
			},
			retry: false
		}
	);

</script>

{#if $query.isError}

	<h1>Oops, This Profile Could Not Be Found</h1>

{:else if $query.isLoading}

	<h1>Profile loading...</h1>

{:else if $query.isSuccess}
	<div class="grid grid-cols-1 md:grid-cols-4 gap-8 w-4/5 justify-center m-auto items-end">
		<div class="md:col-1 flex justify-center">
			<ProfileImage userId={$query.data.id} />
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

