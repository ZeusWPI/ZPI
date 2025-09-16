<script lang="ts">
	import AchievementServiceGroup from '$lib/components/achievements/AchievementServiceGroup.svelte';
	import { createQuery, type CreateQueryResult } from '@tanstack/svelte-query';
	import { type AchievementService, getAchievementServices } from '$lib/globalFunctions-Types';

	const services = ['tap', 'zodom', 'ledstrip'];

	let query: CreateQueryResult<AchievementService[]> = createQuery({
			queryKey: [`achievement-services`],
			queryFn: getAchievementServices,
			retry: false
		}
	);

</script>

{#if $query.isSuccess}
	<div class="flex flex-col items-center">
		{#each $query.data as service}
			<AchievementServiceGroup {service} />
		{/each}
	</div>
{/if}