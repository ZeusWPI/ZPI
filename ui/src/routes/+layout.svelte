<script lang="ts">
	import './layout.css';
	import favicon from '#lib/assets/ZPI.svg';
	import Navbar from '#lib/components/Navbar.svelte';
	import Footer from '#lib/components/Footer.svelte';
	import type { Snippet } from 'svelte';
	import type { PageData } from './$types';

	let { data, children }: { data: PageData, children: Snippet } = $props();
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
	<title>ZPI</title>
</svelte:head>

<div class="flex flex-col items-center w-full min-h-screen">
	<Navbar />
	{@render children()}
	<div class="grow"></div>
	<Footer backendVersion={data.backendVersion} />
</div>