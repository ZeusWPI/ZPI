<script>
	import Tooltip from '$lib/components/Tooltip.svelte';

	let { showcaseProperties } = $props();

	const achievements = new Map([
		['quotes:quoted', {
			'icon': 'https://upload.wikimedia.org/wikipedia/commons/8/88/Quotes_icon-icons.svg',
			'name': 'Quoted',
			'description': 'Got Quoted By Someone.'
		}],
		['gamification:contributor', {
			'icon': 'https://cdn-icons-png.freepik.com/512/5431/5431386.png',
			'name': 'Contributor',
			'description': 'Contributed to a ZeusWPI Repository'
		}]
	]);

</script>

<fieldset class="flex flex-wrap border-solid border-2 rounded-xl border-gray-300 w-full p-3">
	<legend class="font-bold text-center ms-4">Achievements</legend>
	{#each showcaseProperties.unlocked as achievement}
		<div class="mx-2">
			<Tooltip text={achievements.get(achievement)?.description}>
				<img class="size-15"
						 src={achievements.get(achievement)?.icon} alt="Achievement Icon" />
				<span>{achievements.get(achievement)?.name}</span>
			</Tooltip>
		</div>
	{/each}
</fieldset>