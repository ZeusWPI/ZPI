<script lang="ts">
	import AchievementShowcase from '$lib/components/showcases/AchievementShowcase.svelte';
	import QuoteShowcase from '$lib/components/showcases/QuoteShowcase.svelte';

	type Showcase = typeof AchievementShowcase | typeof QuoteShowcase;

	let showcases = [
		{
			'type': 'achievement',
			'properties': {
				'unlocked': [
					'quotes:quoted',
					'gamification:contributor'
				]
			}
		}, {
			'type': 'quote',
			'properties': {
				'quotes': [
					'Wow Nathan Is Een Zeer Coole Jongen',
					'Ik heb dat wel niet gezegd eh.',
					'Voor sommige mensen is links en rechts hetzelfde als boven en beneden.'
				]
			}
		}
	];

	const componentMap: Map<string, Showcase> = new Map();
	componentMap.set('achievement', AchievementShowcase);
	componentMap.set('quote', QuoteShowcase);


	function getComponent(type: string): Showcase {
		return componentMap.get(type) || AchievementShowcase;
	}
</script>
<div id="showcases">
	{#each showcases as showcase}
		<hr class="w-48 h-0.5 mx-auto my-2 bg-gray-100 border-0 rounded-sm md:w-5/6 md:my-8 dark:bg-gray-700">
		<div class="showcase">
			<svelte:component this={getComponent(showcase.type)} showcaseProperties={showcase.properties} />
		</div>
	{/each}
</div>