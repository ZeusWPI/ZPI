<script lang="ts">
	import ProfileTag from '$lib/components/ProfileTag.svelte';
	import mattermostIcon from '$lib/assets/mattermost.svg';

	type User = { id: number, username: string, mattermost: string, name: string }
	let user: User = {
		id: 385,
		username: 'caturn',
		name: 'Saturn Casaer',
		mattermost: 'caturn'
	};

	const profileImg = 'https://zpi.zeus.gent/image/385';

	const tags = [{ 'label': 'Bestuur', 'color': '255, 128, 0' }, { 'label': 'Sysadmin', 'color': '255, 0, 0' }];
</script>


<img class="size-56 md:size-96 rounded-4xl m-6 mb-2" src="{profileImg}" alt="Profile">
<div class="flex flex-col justify-center items-center">
	<span
		class="text-4xl grow min-w-64 text-center md:text-left"
	>{ user.name }</span>
	<div class="flex flex-row justify-center gap-3 mb-4 text-xl">
		<span>@{user.username}</span>
		<span class="flex flex-row justify-center items-center">
			<img class="size-3 mr-0.5" src={mattermostIcon} alt="Mattermost" />
			{user.mattermost}
		</span>
	</div>

	<div>
		{#each tags as tag}
			<ProfileTag {tag} />
		{/each}
	</div>
</div>

