<script lang="ts">
	import ProfileTag from '$lib/components/profile-components/ProfileTag.svelte';
	import mattermostIcon from '$lib/assets/mattermost.svg';
	import { toTitleCase } from '$lib/globalFunctions-Types.js';

	let { user } = $props();

	const mattermostPlaceholder = 'caturn';

</script>


<div class="flex flex-col justify-center items-center md:grow md:items-start md:justify-end md:p-2">

	<!-- Name -->
	<span
		class="text-4xl grow min-w-64 text-center md:text-left md:grow-0"
	>{ toTitleCase(user.username) }</span>

	<!-- ZAuth & Mattermost -->
	<div class="flex flex-row justify-center gap-3 mb-4 text-xl">
		<span>@{user.username}</span>
		<span class="flex flex-row justify-center items-center">
			<img class="size-3 mr-0.5" src={mattermostIcon} alt="Mattermost" />
			{mattermostPlaceholder}
		</span>
	</div>

	<!-- Tags -->
	<div>
		{#each user.tags as tag}
			<ProfileTag {tag} />
		{/each}
	</div>
</div>

