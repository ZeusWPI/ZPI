<script lang="ts">
	import ProfileSidebar from '$lib/components/profile-components/ProfileSidebar.svelte';
	import ProfileSummary from '$lib/components/profile-components/ProfileSummary.svelte';
	import ShowcaseDisplay from '$lib/components/showcases/ShowcaseDisplay.svelte';
	import ProfileImage from '$lib/components/profile-components/ProfileImage.svelte';
	import type { CreateQueryResult } from '@tanstack/svelte-query';
	import { createQuery } from '@tanstack/svelte-query';
	import { getProfile, type ProfileData } from '$lib/globalFunctions-Types';

	let { username, editAllowed = false } = $props();

	let query: CreateQueryResult<ProfileData> = createQuery({
			queryKey: [`profile-${username}`],
			queryFn: async () => getProfile(username),
			retry: false
		}
	);

</script>

{#if $query.isSuccess}
	<div class="grid grid-cols-1 md:grid-cols-4 gap-8 w-4/5 justify-center m-auto items-end">
		<div class="md:col-1 flex justify-center">
			<ProfileImage userId={$query.data.id} {editAllowed} />
		</div>
		<div class="md:col-start-2 md:col-span-3">
			<ProfileSummary user={$query.data} />
		</div>
	</div>
	<div class="grid grid-cols-1 md:grid-cols-4 gap-8 w-4/5 items-start justify-center m-auto">

		<div class="md:col-1">
			<ProfileSidebar user={$query.data} {editAllowed} />
		</div>
		<div class="md:col-start-2 md:col-span-3">
			<ShowcaseDisplay />
		</div>
	</div>

{:else if $query.isLoading}
	<h1 class="flex flex-row justify-center items-center text-center grow text-5xl">
		Fetching Profile...
	</h1>
{:else}
	<h1 class="flex flex-row justify-center items-center text-center grow text-5xl">
		Something went wrong, couldn't fetch profile
	</h1>
{/if}

