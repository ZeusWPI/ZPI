<script lang="ts">
	import PencilIcon from '$lib/components/icons/PencilIcon.svelte';
	import { submitAbout } from '$lib/globalFunctions-Types';

	let editMode = $state(false);

	let { userDescription, userId, editAllowed } = $props();

	let currentDescription = $state(userDescription);

	let aboutInput: HTMLTextAreaElement | undefined = $state();

	async function editAbout() {
		if (aboutInput?.value) {
			const response = await submitAbout(userId, aboutInput?.value);
			if (response.ok) {
				editMode = false;
				currentDescription = await response.json().then((r) => r.about);
			}
		}
	}


</script>

<span class="font-bold flex flex-row items-center">About
	{#if !editMode && editAllowed}
	<button class="cursor-pointer rounded-md text-orange-900 bg-orange-200 hover:bg-orange-300 p-1 mx-2"
					onclick="{() => editMode=true}">
			<span class="flex justify-center items-center size-4">
			 <PencilIcon />
			</span>
	</button>
{/if}
	</span>
<div class="mb-6">
	{#if editMode }
		<form>
			<textarea bind:this={aboutInput} class="rounded-lg h-52 w-full justify-start">{currentDescription}</textarea>
			<button
				class="cursor-pointer rounded-md text-orange-900 border-orange-900 hover:bg-orange-200 border-2 py-0.5 px-2"
				onclick="{() => editMode=false}">
				Cancel
			</button>
			<button
				class="cursor-pointer rounded-md text-orange-900 bg-orange-100 border-1 border-orange-100 hover:bg-orange-200 hover:border-orange-200 py-0.5 px-2"
				onclick={editAbout}>✓
				Confirm
			</button>
		</form>
	{:else }
		<p class="mb-2">{currentDescription}</p>
	{/if}
</div>