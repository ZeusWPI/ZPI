<script lang="ts">
	import CheckIcon from '$lib/components/icons/CheckIcon.svelte';
	import CrossIcon from '$lib/components/icons/CrossIcon.svelte';

	let { checked, children } = $props();
</script>

<span class="inline-flex flex-row items-baseline">
	{#if checked}
		<span class="text-green-400 size-5 inline-block my-auto mr-1">
		<CheckIcon />
		</span>
	{:else}
		<span class="text-red-400 size-5 inline-block my-auto mr-1">
		<CrossIcon />
		</span>
	{/if}
	{@render children()}
</span>