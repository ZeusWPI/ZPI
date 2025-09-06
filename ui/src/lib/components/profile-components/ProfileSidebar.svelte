<script lang="ts">
	import SocialLink from '$lib/components/profile-components/sidebar-components/SocialLink.svelte';
	import defaultIcon from '$lib/assets/default-link.svg';

	import githubIcon from '$lib/assets/brand-icons/github.svg';
	import AboutBlock from '$lib/components/profile-components/sidebar-components/AboutBlock.svelte';

	let { user, editAllowed } = $props();


	const links = {
		'GitHub': 'https://github.com/lolcatjpg',
		'MatterMost': ''
	};
	let linkImageMap = new Map([
		['github', githubIcon]
	]);

	const getIcon = (service: string) => linkImageMap.get(service.toLowerCase()) || defaultIcon;


</script>
<hr class="w-48 h-0.5 mx-auto my-2 bg-gray-100 border-0 rounded-sm md:my-8 dark:bg-gray-700">
<AboutBlock userDescription={user.about} userId={user.id} {editAllowed} />
<div class="flex">
	{#each Object.entries(links) as [service, link]}
		<SocialLink icon={getIcon(service)} {service} {link} />
	{/each}
</div>


