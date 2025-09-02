<script lang="ts">
	import ProfileTag from '$lib/components/profile-components/ProfileTag.svelte';
	import mattermostIcon from '$lib/assets/mattermost.svg';

	let { user } = $props();

	type User = { id: number, username: string, mattermost: string, name: string }
	let placeholderUser: User = {
		id: 385,
		username: 'caturn',
		name: 'Saturn Casaer',
		mattermost: 'caturn'
	};


	const tags = [{ 'label': 'Bestuur', 'color': '255, 128, 0' }, { 'label': 'Sysadmin', 'color': '255, 0, 0' }];
</script>


<div class="flex flex-col justify-center items-center md:grow md:items-start md:justify-end md:p-2">

	<!-- Name -->
	<span
		class="text-4xl grow min-w-64 text-center md:text-left md:grow-0"
	>{ placeholderUser.name }</span>

	<!-- ZAuth & Mattermost -->
	<div class="flex flex-row justify-center gap-3 mb-4 text-xl">
		<span>@{user.username}</span>
		<span class="flex flex-row justify-center items-center">
			<img class="size-3 mr-0.5" src={mattermostIcon} alt="Mattermost" />
			{placeholderUser.mattermost}
		</span>
	</div>

	<!-- Tags -->
	<div>
		{#each tags as tag}
			<ProfileTag {tag} />
		{/each}
	</div>
</div>

