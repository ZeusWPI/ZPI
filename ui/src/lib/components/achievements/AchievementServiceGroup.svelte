<script lang="ts">
	import { getAchievementsFromService, toTitleCase } from '$lib/globalFunctions-Types';
	import type { Achievement as AchievementPayload } from '$lib/globalFunctions-Types';
	import ArrowNoBaseIcon from '$lib/components/icons/ArrowNoBaseIcon.svelte';
	import Achievement from '$lib/components/achievements/Achievement.svelte';
	import { createQuery, type CreateQueryResult } from '@tanstack/svelte-query';
	import { slide } from 'svelte/transition';

	let { service } = $props();

	let isOpen = $state(false);

	function toggle() {
		isOpen = !isOpen;
	}

	let query: CreateQueryResult<AchievementPayload[]> = createQuery({
		queryKey: [`service-${service.id}-achievements`],
		queryFn: async () => getAchievementsFromService(service.id),
		retry: false
	});
</script>

{#if $query.isSuccess}
	<div class="w-full md:w-4/5">
		<button
			class="flex flex-row justify-between items-center w-full px-10 py-4 font-bold text-lg md:text-2xl"
			onclick={toggle}
			tabindex="0"
		>
			{toTitleCase(service.name)}
			<span
				class="size-10 transition-transform duration-300 {isOpen ? 'rotate-0' : 'rotate-180'}"
			>
				<ArrowNoBaseIcon />
			</span>
		</button>

		{#if isOpen}
			<div transition:slide class="overflow-hidden">
				{#each $query.data as achievement }
					<Achievement {achievement} />
				{/each}
			</div>
		{/if}
	</div>
{/if}
