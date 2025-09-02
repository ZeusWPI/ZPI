<script lang="ts">
	import { onMount } from 'svelte';
	import { createDialog } from 'svelte-headlessui';
	import Transition from 'svelte-transition';

	let { title, children } = $props();

	const dialog = createDialog({ label: title });

	export function close() {
		dialog.close();
	}

	export function open() {
		dialog.open();
	}

</script>
<div class="relative z-10">
	<Transition show={$dialog.expanded}>
		<Transition
			enter="ease-out duration-300"
			enterFrom="opacity-0"
			enterTo="opacity-100"
			leave="ease-in duration-200"
			leaveFrom="opacity-100"
			leaveTo="opacity-0"
		>
			<button class="fixed inset-0 bg-black/25" aria-label="close" onclick={dialog.close}></button>
		</Transition>

		<div class="fixed inset-0 overflow-y-auto">
			<div class="flex min-h-full items-center justify-center p-4 text-center">
				<Transition
					enter="ease-out duration-300"
					enterFrom="opacity-0 scale-95"
					enterTo="opacity-100 scale-100"
					leave="ease-in duration-200"
					leaveFrom="opacity-100 scale-100"
					leaveTo="opacity-0 scale-95"
				>
					<div
						class="w-full max-w-md transform overflow-hidden rounded-2xl bg-white p-6 text-left align-middle shadow-xl transition-all"
						use:dialog.modal
					>
						<h3 class="text-lg leading-6 font-medium text-gray-900">{title}</h3>
						<div class="mt-2">
							{@render children()}
						</div>

						<div class="mt-4">
							<button
								type="button"
								class="inline-flex justify-center rounded-md border border-transparent bg-blue-100 px-4 py-2 text-sm font-medium text-blue-900 hover:bg-blue-200 focus:outline-hidden focus-visible:ring-2 focus-visible:ring-blue-500 focus-visible:ring-offset-2"
								onclick={dialog.close}
							>
								Close
							</button>
						</div>
					</div>
				</Transition>
			</div>
		</div>
	</Transition>
</div>

