<script lang="ts">

	let editMode = $state(false);

	let { userDescription, userId } = $props();

	let currentDescription = $state(userDescription);

	let aboutInput: HTMLTextAreaElement | undefined = $state();

	async function editAbout() {
		const response = await fetch(
			`http://localhost:3000/api/users/${userId}`,
			{
				credentials: 'include',
				method: 'PATCH',
				headers: {
					'Content-type': 'application/json'
				},
				body: JSON.stringify({
					about: aboutInput?.value
				})
			}
		);
		if (response.ok) {
			editMode = false;
			currentDescription = await response.json().then((r) => r.about);
		}
	}


</script>

<span class="font-bold">About</span>
<br>
<div class="mb-6">
	{#if editMode }
		<form>
			<textarea bind:this={aboutInput} class="rounded-lg h-52 w-full justify-start">{currentDescription}</textarea>
			<button class="cursor-pointer rounded-3xl border-blue-200 border-2 py-0.5 px-2" onclick="{() => editMode=false}">
				Cancel
			</button>
			<button class="cursor-pointer rounded-3xl bg-blue-200 py-0.5 px-2" onclick={editAbout}>✓ Confirm</button>
		</form>
	{:else }
		<p class="mb-2">{currentDescription}</p>
		<button class="cursor-pointer rounded-3xl bg-blue-200 py-0.5 px-3" onclick="{() => editMode=true}">Edit</button>
	{/if}
</div>