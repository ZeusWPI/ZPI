<script>
	import ProfileSidebar from '$lib/components/ProfileSidebar.svelte';
	import ProfileSummary from '$lib/components/ProfileSummary.svelte';
	import ShowcaseDisplay from '$lib/components/ShowcaseDisplay.svelte';
	import Navbar from '$lib/components/Navbar.svelte';
	import Footer from '$lib/components/Footer.svelte';
</script>

<Navbar />

<div class="flex flex-col w-4/5 items-center justify-center m-auto">
	<div class="flex flex-row w-full flex-wrap justify-center mb-8">
		<ProfileSummary />
	</div>
	<ProfileSidebar class="w-md max-w-full" />
	<ShowcaseDisplay />
</div>

<div class="flex flex-row justify-center mt-5 mb-3">
	<div class="w-5/6">
		<Footer />
	</div>
</div>