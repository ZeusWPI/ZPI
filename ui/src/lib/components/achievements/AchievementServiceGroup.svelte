<script lang="ts">
	import { getAchievementsFromService, toTitleCase } from '$lib/globalFunctions-Types';
	import type { Achievement as AchievementPayload } from '$lib/globalFunctions-Types';
	import ArrowNoBaseIcon from '$lib/components/icons/ArrowNoBaseIcon.svelte';
	import Achievement from '$lib/components/achievements/Achievement.svelte';
	import { createQuery, type CreateQueryResult } from '@tanstack/svelte-query';
	import { slide } from 'svelte/transition';
	import PencilIcon from '$lib/components/icons/PencilIcon.svelte';

	let { service, editAllowed, editModal } = $props();

	let isOpen = $state(false);

	function toggle() {
		isOpen = !isOpen;
	}

	let query: CreateQueryResult<AchievementPayload[]> = createQuery({
		queryKey: [`service-${service.id}-achievements`],
		queryFn: async () => getAchievementsFromService(service.id),
		retry: false
	});

	function keyClickHandler(event: any, clickFn: Function) {
		if (event.key === 'Enter' || event.key === ' ') {
			event.preventDefault();
			clickFn();
		}
	}

	function openEdit() {
		toggle();
		editModal.open();
	}
</script>

{#if $query.isSuccess}
	<div class="w-full">
		<div role="button"
				 class="flex flex-row justify-between items-center w-full py-4 font-bold text-lg md:text-2xl"
				 onclick={toggle}
				 onkeydown={(ev) => keyClickHandler(ev, toggle)}
				 tabindex="0"
		>
			<span>
			{toTitleCase(service.name)}
				{#if editAllowed}
					<button class="cursor-pointer rounded-md text-orange-900 bg-orange-200 hover:bg-orange-300 p-1 mx-2"
									onclick={openEdit}
									onkeydown={(ev) => keyClickHandler(ev, openEdit)}>
			<span class="flex justify-center items-center size-4">
			 <PencilIcon />
			</span>
	</button>
				{/if}
				</span>
			<span
				class="size-10 transition-transform duration-300 {isOpen ? 'rotate-0' : 'rotate-180'}"
			>
				<ArrowNoBaseIcon />
			</span>
		</div>

		{#if isOpen}
			<div transition:slide class="overflow-hidden">
				{#each $query.data as achievement }
					<Achievement {achievement} />
				{/each}
			</div>
		{/if}
	</div>
{/if}
