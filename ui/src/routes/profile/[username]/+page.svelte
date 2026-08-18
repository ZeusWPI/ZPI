<script lang="ts">
	import { page } from '$app/state';
	import Profile from '$lib/components/profile-components/Profile.svelte';
	import type { CurrentUser } from '$lib/globalFunctions-Types';

	let username = page.params.username;
	let { data }: { data: { user: CurrentUser | null } } = $props();

</script>

{#if data.user && username}
	<Profile username={username} editAllowed={username === data.user.username} />
	<div class="grow"></div>
{:else}
	<h1 class="flex flex-row justify-center items-center text-center grow text-5xl">
		Hold on, we're checking if you're logged in...
	</h1>
{/if}
