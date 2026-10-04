<script lang="ts">
	import { APP_VERSION } from '$app/env/public';
	import { Tooltip } from 'flowbite-svelte';
	import SvelteIcon from '#lib/assets/svelte.svg';
	import RustIcon from '#lib/assets/rust.svg';

	let { backendVersion }: { backendVersion?: { version: string } } = $props();
</script>

<div class="mt-10 mb-3 flex w-full flex-row justify-between px-6">
    <span>
        <a href="https://git.zeus.gent/ZeusWPI/ZEG/src/branch/main/backend" class="hover:underline">
        <img class="inline size-4" src={RustIcon} alt="Rust" />
        v{backendVersion ? backendVersion.version : "?.?.?"}
        </a>
        <Tooltip>Rust Backend</Tooltip>
        <a href="https://git.zeus.gent/ZeusWPI/ZEG/src/branch/main/frontend" class="hover:underline">
            <img class="inline size-4" src={SvelteIcon} alt="Svelte" />
            v{APP_VERSION}
        </a>
        <Tooltip>Svelte Frontend</Tooltip>
    </span>
	<span
	>Made with ❤️ by <img
		class="ml-1 inline size-8"
		src="https://zinc.zeus.gent/zeus"
		alt="Zeus"
	/></span
	>
	<span class="min-w-24"></span>
</div>