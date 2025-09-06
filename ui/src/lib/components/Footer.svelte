<script lang="ts">
	import Tooltip from '$lib/components/Tooltip.svelte';
	import svelteIcon from '$lib/assets/svelte.svg';
	import rustIcon from '$lib/assets/rust.svg';
	import type { CreateQueryResult } from '@tanstack/svelte-query';
	import { createQuery } from '@tanstack/svelte-query';
	import { getBackendVersion } from '$lib/globalFunctions-Types';
	import { version } from '$app/environment';


	let query: CreateQueryResult<String> = createQuery({
		queryKey: ['version'],
		queryFn: getBackendVersion
	});

</script>


<div class="flex flex-row justify-between md:grid md:grid-cols-3">
	<!-- Versions -->
	<div class="flex flex-row items-center gap-2">
		<Tooltip text="Backend Version">
			<div class="flex flex-row items-center">
				<img class="size-4 mx-0.5" src={rustIcon} alt="Rust" />
				v{$query.data}
			</div>
		</Tooltip>
		<Tooltip text="Frontend Version">
			<div class="flex flex-row items-center">
				<img class="size-4 mx-0.5" src={svelteIcon} alt="Svelte" />
				v{version}
			</div>
		</Tooltip>
	</div>

	<!-- Center / Right on Mobile -->
	<span class="text-right md:text-center">Made with ❤️ by ZeusWPI</span>

</div>