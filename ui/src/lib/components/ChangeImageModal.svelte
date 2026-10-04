<script lang="ts">
	import { Avatar, Button, Modal, P } from 'flowbite-svelte';
	import { CloudArrowUpOutline, PenSolid, RefreshOutline } from 'flowbite-svelte-icons';
	import type { User } from '#lib/types.ts';
	import ErrorMsg from '#lib/components/ErrorMsg.svelte';

	let { user, onSuccess }: { user: User, onSuccess: () => void } = $props();

	let isOpen = $state(false);

	let fileInput: HTMLInputElement;
	let previewUrl = $derived(`/image/${user.id}`);

	let isResettingImage = $state(false);

	function handleFileChange(event: Event) {
		const input = event.currentTarget as HTMLInputElement;
		const file = input.files?.[0];

		if (!file) return;

		if (!file.type.startsWith('image/')) {
			return;
		}

		if (previewUrl.startsWith('blob:')) {
			URL.revokeObjectURL(previewUrl);
		}

		previewUrl = URL.createObjectURL(file);
		isResettingImage = false;
	}

	function resetImage() {
		isResettingImage = true;
		previewUrl = `/image/${user.id}`;
	}

	async function submit() {
		noImageError = false;
		backendError = null;

		let response: Response | null = null;
		if (isResettingImage) {
			response = await fetch('/api/image', {
				method: 'DELETE'
			});

			fileInput.value = '';
			previewUrl = `/image/${user.id}#${Math.random()}`;
			isOpen = false;
			onSuccess();
		} else {
			if (fileInput.files && fileInput.files.length > 0) {
				response = await fetch('/api/image', {
					method: 'POST',
					body: fileInput.files[0]
				});

				fileInput.value = '';
				previewUrl = `/image/${user.id}#${Math.random()}`;
			} else {
				noImageError = true;
				return;
			}
		}

		if (!response) {
			backendUnavailable = true;
			return;
		}

		if (!response.ok) {
			backendError = await response.text();
		} else {
			isOpen = false;
			onSuccess();
		}
	}

	let noImageError = $state(false);
	let backendUnavailable = $state(false);
	let backendError: string | null = $state(null);
</script>

<Button onclick={() => (isOpen = true)} class="p-2">
	<PenSolid class="shrink-0 size-8" />
</Button>
<Modal title="Change Profile Image" bind:open={isOpen}>
	<div class="grid grid-cols-2 gap-6">
		<!-- User Image -->
		<Avatar src={previewUrl} class="size-50 rounded-xl justify-self-center" border />
		<div class="flex flex-col self-center gap-2">
			<input type="file" class="hidden" bind:this={fileInput} accept="image/*" onchange={handleFileChange} />

			<Button onclick={() => (fileInput?.click())}>
				<CloudArrowUpOutline class="shrink-0 h-6 w-6" />
				Upload
			</Button>
			<Button color="alternative" onclick={resetImage}>
				<RefreshOutline class="shrink-0 h-6 w-6" />
				Reset
			</Button>
		</div>
	</div>

	{#if noImageError}
		<ErrorMsg>No File Selected</ErrorMsg>
	{/if}
	{#if backendUnavailable}
		<ErrorMsg>There was an error communicating with the backend</ErrorMsg>
	{/if}
	{#if backendError}
		<ErrorMsg>{backendError}</ErrorMsg>
	{/if}


	{#snippet footer()}
		<span class="w-full flex justify-end gap-1">
			<Button onclick={submit}>Submit</Button>
			<Button color="alternative" onclick={() => isOpen = false}>Cancel</Button>
		</span>
	{/snippet}
</Modal>