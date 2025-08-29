<script lang="ts">
	import favicon from '$lib/assets/favicon.svg';
	import logoutIcon from '$lib/assets/logout.svg';

	type User = { id: number, username: string, mattermost: string, name: string }
	let user: User = {
		id: 385,
		username: 'caturn',
		name: 'Saturn Casaer',
		mattermost: 'caturn'
	};
</script>


<div class=" w-full text-white left-0 top-0 p-2 flex flex-row justify-between items-center">
	<!-- Left Content -->
	<div class="flex flex-row items-center">
		<img class="size-8" src={favicon} alt="ZPI Icon">
	</div>
	<!-- Right Content -->
	<div class="flex flex-row items-center">
		<span class="text-lg">{user.username}</span>
		<a href="/logout">
			<img class="size-6 mx-2 invert" src={logoutIcon} alt="Logout" />
		</a>
	</div>
</div>

<style>
    div {
        background-color: #ff7f00;
    }
</style>