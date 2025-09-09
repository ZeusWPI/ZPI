<script lang="ts">
	import { page } from '$app/state';
	import Navbar from '$lib/components/Navbar.svelte';
	import Error from '$lib/components/Error.svelte';
	import Footer from '$lib/components/Footer.svelte';
</script>
<div id="pageContent">
	<Navbar username="" />
	<Error errorCode={page.status} errorMessage={page.error?.message} />
	<Footer />
</div>

<style>
    #pageContent {
        display: flex;
        flex-direction: column;
        min-height: 100vh;
    }
</style>