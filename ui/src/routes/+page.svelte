<script lang="ts">
	import Profile from '$lib/components/profile-components/Profile.svelte';
	import type { CurrentUser } from '$lib/globalFunctions-Types';

	let { data }: { data: { user: CurrentUser | null } } = $props();
</script>

{#if data.user}
	<Profile username={data.user.username} editAllowed={true} />
	<div class="grow"></div>
{:else}
	<h1 class="flex flex-row justify-center items-center text-center grow text-5xl">
		Hold on, we're checking if you're logged in...
	</h1>
{/if}
