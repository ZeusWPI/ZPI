<script>
	import ProfileSidebar from '$lib/components/ProfileSidebar.svelte';
	import ProfileSummary from '$lib/components/ProfileSummary.svelte';
</script>


<div class="flex flex-col w-4/5 items-center justify-center m-auto">
	<ProfileSummary />
	<ProfileSidebar class="w-md max-w-full" />
</div>