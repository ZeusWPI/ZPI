<script lang="ts">
	import type { User } from '#lib/types.ts';
	import { Button, P, Textarea } from 'flowbite-svelte';
	import { PenSolid } from 'flowbite-svelte-icons';
	import ErrorMsg from '#lib/components/ErrorMsg.svelte';

	let {user, onSuccess = () => {}}: {user: User, onSuccess?: () => void} = $props();

	let editMode = $state(false);

	let aboutEdit = $derived(user.about);

	const reset = () => {
		aboutEdit = user.about;
		editMode = false;
	};

	const submit = async () => {
		let response = await fetch(`/api/users/${user.id}`, {
			method: 'PATCH',
			body: JSON.stringify({
				about: aboutEdit
			}),
			headers: {
				"Content-Type": "application/json",
			}
		});
		console.log(response)

		if (!response) {
			backendUnavailable = true;
			return;
		}

		if (!response.ok) {
			backendError = await response.text();
		} else {
			editMode = false;
			onSuccess();
		}
	}

	let backendUnavailable = $state(false);
	let backendError: string | null = $state(null);
</script>

<span class="font-semibold text-lg">About
	{#if !editMode}
		<Button onclick={() => (editMode = true)} class="p-1 rounded-sm focus:ring-2">
			<PenSolid class="shrink-0 size-3" />
		</Button>
	{/if}
</span>

{#if !editMode}
	<P class="w-full">
		{user.about}
	</P>
{:else}
	<Textarea class="w-full" bind:value={aboutEdit} />
	{#if backendUnavailable}
		<ErrorMsg>There was an error communicating with the backend</ErrorMsg>
	{/if}
	{#if backendError}
		<ErrorMsg>{backendError}</ErrorMsg>
	{/if}
	<span>
		<Button class="p-2 py-1 rounded-md focus:ring-2" onclick={submit}>Submit</Button>
		<Button class="p-2 py-1 rounded-md focus:ring-2" color="alternative" onclick={reset}>Cancel</Button>
	</span>
{/if}