<script lang="ts">
	import AchievementServiceGroup from '$lib/components/achievements/AchievementServiceGroup.svelte';
	import { createQuery, type CreateQueryResult } from '@tanstack/svelte-query';
	import { type AchievementService, getAchievementServices } from '$lib/globalFunctions-Types';
	import AchievementEditModal from '$lib/components/achievements/AchievementEditModal.svelte';

	const services = ['tap', 'zodom', 'ledstrip'];

	let query: CreateQueryResult<AchievementService[]> = createQuery({
			queryKey: [`achievement-services`],
			queryFn: getAchievementServices,
			retry: false
		}
	);

	const editAllowed = true;

	let editModal: AchievementEditModal;

</script>

{#if $query.isSuccess}
	<div class="flex flex-col items-center w-full md:w-4/5 mx-auto px-10">
		<div class="flex flex-row justify-end w-full mt-10">
			<button class="bg-orange-200 hover:bg-orange-300 px-4 py-2 rounded-md font-semibold text-orange-900"
							onclick={editModal.open}
			>
				Add Service
			</button>
		</div>
		{#each $query.data as service}
			<AchievementServiceGroup {service} editAllowed={editAllowed} editModal={editModal} />
		{/each}
	</div>

	{#if editAllowed}
		<AchievementEditModal bind:this={editModal} />
	{/if}
{/if}

