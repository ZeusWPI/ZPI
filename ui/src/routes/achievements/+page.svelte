<script lang="ts">
	import { type CurrentUser } from '$lib/globalFunctions-Types';
	import AchievementDisplay from '$lib/components/achievements/AchievementDisplay.svelte';

	let { data }: { data: { user: CurrentUser | null } } = $props();
</script>
{#if data.user}
	<AchievementDisplay admin={data.user.admin} />
{/if}