<script lang="ts">
	import SocialLink from '$lib/components/sidebar-components/SocialLink.svelte';
	import defaultIcon from '$lib/assets/default-link.svg';

	import githubIcon from '$lib/assets/brand-icons/github.svg';

	const userDescription = 'This is a placeholder for the about section of a profile. Here, you would typically find a personal introduction and a short biography. The user might describe their professional journey, their core interests and passions, and what they are currently working on. This text serves as a way to present a narrative beyond just a list of skills or job titles.';

	const links = {
		'GitHub': 'https://github.com/lolcatjpg',
		'MatterMost': ''
	};
	let linkImageMap = new Map([
		['github', githubIcon]
	]);

	const getIcon = (service: string) => linkImageMap.get(service.toLowerCase()) || defaultIcon;


</script>
<div class={`${$$props.class || ''}`}>
	<hr class="w-48 h-0.5 mx-auto my-2 bg-gray-100 border-0 rounded-sm md:my-8 dark:bg-gray-700">
	<span class="font-bold">About</span>
	<br>
	<p>{userDescription}</p>
	<br>
	<div class="flex">
		{#each Object.entries(links) as [service, link]}
			<SocialLink icon={getIcon(service)} {service} {link} />
		{/each}
	</div>
</div>


