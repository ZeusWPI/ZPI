<script lang="ts">
	import CheckedLabel from '$lib/components/CheckedLabel.svelte';

	let { achievement } = $props();

	let goalsUnlocked = [];
	for (let i = 0; i < achievement.goals.length; i++) {
		goalsUnlocked.push(Math.random() >= 0.5);
	}

	let imgSrc = 'https://cdn.fastly.steamstatic.com/steamcommunity/public/images/apps/1903340/893a5719f74928a4706ad295b4ab42cf0a2ffacb.jpg';

	const achievementUnlocked = () => goalsUnlocked.every(g => g);
</script>

<div class="grid-cols-3 grid grid-rows-1 px-6 gap-3 mb-6">
	{#if achievementUnlocked()}
		<img src={imgSrc} alt="achievement" class="col-start-1 rounded-xl">
	{:else}
		<img src={imgSrc} alt="achievement" class="col-start-1 rounded-xl grayscale">
	{/if}
	<div class="col-start-2 col-span-2 flex flex-col">
		<span class="font-semibold mb-2">{achievement.name}</span>
		<div class="flex flex-col">
			{#each achievement.goals as goal, i}
				<CheckedLabel checked={goalsUnlocked[i]}>{goal.description}</CheckedLabel>
			{/each}
		</div>
	</div>
</div>