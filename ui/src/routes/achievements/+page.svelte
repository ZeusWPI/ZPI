<script lang="ts">
	import { createQuery } from '@tanstack/svelte-query';
	import type { CreateQueryResult } from '@tanstack/svelte-query';
	import Navbar from '$lib/components/Navbar.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import { type CurrentUser, getCurrentUser } from '$lib/globalFunctions-Types';
	import AchievementDisplay from '$lib/components/achievements/AchievementDisplay.svelte';

	let query: CreateQueryResult<CurrentUser> = createQuery({
			queryKey: ['currentUser'],
			queryFn: getCurrentUser
		}
	);
</script>
<div class="flex flex-col min-h-screen bg-white">
	<Navbar username={$query.data?.username || ""} />
	{#if $query.isSuccess}
		<AchievementDisplay />
		<div class="grow"></div>
	{:else if $query.isLoading}
		<h1 class="flex flex-row justify-center items-center text-center grow text-5xl">
			Hold on, we're checking if you're logged in...
		</h1>
	{:else}
		<h1 class="flex flex-row justify-center items-center text-center grow text-5xl">
			Something went wrong, couldn't reach backend
		</h1>
	{/if}
	<div class="flex flex-row justify-center mt-5 mb-3">
		<div class="w-5/6">
			<Footer />
		</div>
	</div>
</div>
