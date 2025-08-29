<script>
	import ProfileTag from '$lib/components/ProfileTag.svelte';

	const profileImg = 'https://zpi.zeus.gent/image/385';
	const username = 'caturn';
	const tags = [{ 'label': 'Bestuur', 'color': '255, 128, 0' }, { 'label': 'Sysadmin', 'color': '255, 0, 0' }];
</script>

<div class="flex flex-row w-full flex-wrap justify-center mb-8">
	<img class="size-56 md:size-96 rounded-4xl m-6 mb-2" src="{profileImg}" alt="Profile">

	<span
		class="text-4xl grow min-w-64 text-center md:text-left mb-2"
	>{ username.charAt(0).toUpperCase() + username.slice(1) }</span>
	
	<div>
		{#each tags as tag}
			<ProfileTag {tag} />
		{/each}
	</div>

</div>