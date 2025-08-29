<script>
	import Tooltip from '$lib/components/Tooltip.svelte';
	import svelteIcon from '$lib/assets/svelte.svg';
	import rustIcon from '$lib/assets/rust.svg';


	let backend_version = '2.0.1';
	let frontend_version = '1.0.1';
</script>


<div class="flex flex-row justify-between md:grid md:grid-cols-3">
	<!-- Versions -->
	<div class="flex flex-row items-center gap-2">
		<Tooltip text="Backend Version">
			<div class="flex flex-row items-center">
				<img class="size-4 mx-0.5" src={rustIcon} alt="Rust" />
				v{backend_version}
			</div>
		</Tooltip>
		<Tooltip text="Frontend Version">
			<div class="flex flex-row items-center">
				<img class="size-4 mx-0.5" src={svelteIcon} alt="Svelte" />
				v{frontend_version}
			</div>
		</Tooltip>
	</div>

	<!-- Center / Right on Mobile -->
	<span class="text-right md:text-center">Made with ❤️ by ZeusWPI</span>

</div>