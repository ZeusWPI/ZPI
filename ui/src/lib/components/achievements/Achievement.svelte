<script lang="ts">
	import CheckedLabel from '$lib/components/CheckedLabel.svelte';

	let { achievement } = $props();

	let goalsUnlocked = [];
	for (let i = 0; i < achievement.goals.length; i++) {
		goalsUnlocked.push(Math.random() >= 0.5);
	}

	let imgSrc = 'https://cdn.fastly.steamstatic.com/steamcommunity/public/images/apps/1903340/893a5719f74928a4706ad295b4ab42cf0a2ffacb.jpg';

	const achievementUnlocked = () => goalsUnlocked.every(g => g);

	const progressBarStyle = `width: ${goalsUnlocked.filter(g => g).length * 100 / goalsUnlocked.length}%`;

</script>

<div class="grid-cols-3 xl:grid-cols-7 grid grid-rows-1 px-6 gap-3 lg:gap-16 mb-6">
	{#if achievementUnlocked()}
		<img src={imgSrc} alt="achievement" class="col-start-1 rounded-xl">
	{:else}
		<img src={imgSrc} alt="achievement" class="col-start-1 rounded-xl grayscale">
	{/if}
	<div class="col-start-2 col-span-6 flex flex-col lg:justify-between">
		<span class="font-semibold mb-2 text-2xl lg:text-center">{achievement.name}</span>
		<div class="flex flex-col lg:flex-row lg:flex-wrap lg:gap-2 lg:justify-between lg:mr-6 lg:mb-4">
			{#each achievement.goals as goal, i}
				<div class="grow">
					<CheckedLabel checked={goalsUnlocked[i]}>{goal.description}</CheckedLabel>
				</div>
			{/each}
		</div>

		<div class="w-full bg-gray-200 rounded-full h-4 dark:bg-gray-700 hidden lg:block lg:mb-6">
			<div class="bg-orange-400 h-4 rounded-full" style={progressBarStyle}></div>
		</div>

	</div>
</div>