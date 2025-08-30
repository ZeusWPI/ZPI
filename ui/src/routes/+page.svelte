<script>
	import ProfileSidebar from '$lib/components/ProfileSidebar.svelte';
	import ProfileSummary from '$lib/components/ProfileSummary.svelte';
	import ShowcaseDisplay from '$lib/components/ShowcaseDisplay.svelte';
	import Navbar from '$lib/components/Navbar.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import ProfileImage from '$lib/components/ProfileImage.svelte';
</script>


<div class="flex flex-col min-h-screen">
	<Navbar />

	<div class="grid grid-cols-1 md:grid-cols-4 gap-8 w-4/5 justify-center m-auto items-end">
		<div class="md:col-1">
			<ProfileImage />
		</div>
		<div class="md:col-start-2 md:col-span-3">
			<ProfileSummary />
		</div>
	</div>
	<div class="grid grid-cols-1 md:grid-cols-4 gap-8 w-4/5 items-start justify-center m-auto">
		<ProfileSidebar class="md:col-1" />
		<div class="md:col-start-2 md:col-span-3">
			<ShowcaseDisplay />
		</div>
	</div>

	<div class="grow"></div>

	<div class="flex flex-row justify-center mt-5 mb-3">
		<div class="w-5/6">
			<Footer />
		</div>
	</div>
</div>
