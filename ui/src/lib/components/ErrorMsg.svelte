<script lang="ts">
	import { ExclamationCircleOutline } from 'flowbite-svelte-icons';
	import type { Snippet } from 'svelte';


	let { children }: { children: Snippet } = $props();
</script>

<div class="bg-red-200 border-2 border-red-500 text-red-500 p-3 rounded-lg w-fit flex gap-1 items-center">
	<ExclamationCircleOutline class="shrink-0 h-6 w-6" />
	{@render children()}
</div>