<script lang="ts">
	import '../app.css';
	import { QueryClient, QueryClientProvider } from '@tanstack/svelte-query';
	import { browser } from '$app/environment';
	import Navbar from '$lib/components/Navbar.svelte';
	import Footer from '$lib/components/Footer.svelte';
	import type { PageData } from './$types';
	import type { Snippet } from 'svelte';

	let { data, children }: { data: PageData, children: Snippet } = $props();

	const queryClient = new QueryClient({
		defaultOptions: {
			queries: {
				enabled: browser
			}
		}
	});

</script>

<QueryClientProvider client={queryClient}>
	<div class="flex flex-col items-center w-full min-h-screen">
		<Navbar user={data.user} />
		<div class="grow w-full">
			{@render children?.()}
		</div>
		<Footer />
	</div>
</QueryClientProvider>
