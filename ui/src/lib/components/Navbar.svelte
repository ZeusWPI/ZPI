<script lang="ts">
	import logoutIcon from '$lib/assets/logout.svg';
	import { env } from '$env/dynamic/public';
	
	const BACKEND_URL = env.PUBLIC_BACKEND_URL;


	let { username } = $props();
</script>


<div class="w-full text-white left-0 top-0 p-2 flex flex-row justify-between items-center">
	<!-- Left Content -->
	<div class="flex flex-row items-center">
		<img class="size-8" src="/favicon.svg" alt="ZPI Icon">
		<span class="ml-2 text-2xl font-extrabold">ZPI</span>
	</div>
	<!-- Right Content -->
	<div class="flex flex-row items-center">
		{#if username !== ""}
			<span class="text-lg">{username}</span>
			<a href="{BACKEND_URL}/api/logout">
				<img class="size-6 mx-2 invert" src={logoutIcon} alt="Logout" />
			</a>
		{/if}
	</div>
</div>

<style>
    div {
        background-color: #ff7f00;
    }
</style>