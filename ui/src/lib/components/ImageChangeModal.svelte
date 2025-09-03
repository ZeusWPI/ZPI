<script lang="ts">
	import { onMount } from 'svelte';
	import { createDialog } from 'svelte-headlessui';
	import Transition from 'svelte-transition';

	let { userId } = $props();

	const dialog = createDialog({ label: 'Change Profile Image' });

	export function close() {
		dialog.close();
	}

	export function open() {
		dialog.open();
	}
	
	let loadFile = function(event: any) {
		let previewImage = document.getElementById('previewImg') as HTMLImageElement;
		previewImage.src = URL.createObjectURL(event.target.files[0]);
		console.log('setSource');
		previewImage.onload = function() {
			URL.revokeObjectURL(previewImage.src);
		};
	};

	function resetImage() {
		fetch('http://localhost:3000/api/image', {
			method: 'DELETE',
			credentials: 'include'
		}).then(() => location.reload());
	}

	function uploadImage() {
		fetch('http://localhost:3000/api/image', {
			method: 'POST',
			headers: {},
			body: (document.getElementById('file-upload') as HTMLInputElement).files?.[0],
			credentials: 'include'
		}).then(() => location.reload());
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
						<h3 class="text-lg leading-6 font-medium text-gray-900">Change Profile Image</h3>
						<div class="mt-2 flex flex-row justify-center">
							<img id="previewImg" class="size-32 rounded-xl m-7" src="http://localhost:3000/api/image/{userId}"
									 alt="Preview">
							<div class="flex flex-col justify-center">
								<label
									class="my-1 rounded-md border border-transparent bg-orange-100 px-4 py-2 text-lg font-medium text-orange-900 hover:bg-orange-200 focus:outline-hidden focus-visible:ring-2 focus-visible:ring-orange-500 focus-visible:ring-offset-2"
									for="file-upload">Select Image</label>
								<input class="hidden" id="file-upload" type="file"
											 name="image-file"
											 accept="image/jpeg,image/png,image/gif,image/webp" onchange={loadFile}>
								<button
									class="my-1 justify-center rounded-md border-2 border-orange-900  px-4 py-2 text-lg font-medium text-orange-900 hover:bg-orange-200 focus:outline-hidden focus-visible:ring-2 focus-visible:ring-orange-500 focus-visible:ring-offset-2"
									onclick={resetImage}
								>Reset
								</button>
							</div>
						</div>

						<div class="mt-4 flex flex-row justify-between">
							<button
								type="button"
								class="inline-flex justify-center rounded-md border-2 border-orange-900  px-4 py-2 text-sm font-medium text-orange-900 hover:bg-orange-200 focus:outline-hidden focus-visible:ring-2 focus-visible:ring-orange-500 focus-visible:ring-offset-2"
								onclick={dialog.close}
							>
								Close
							</button>
							<button
								type="button"
								class="inline-flex justify-center rounded-md border border-transparent bg-orange-100 px-4 py-2 text-sm font-medium text-orange-900 hover:bg-orange-200 focus:outline-hidden focus-visible:ring-2 focus-visible:ring-orange-500 focus-visible:ring-offset-2"
								onclick={uploadImage}
							>
								Confirm
							</button>
						</div>
					</div>
				</Transition>
			</div>
		</div>
	</Transition>
</div>

