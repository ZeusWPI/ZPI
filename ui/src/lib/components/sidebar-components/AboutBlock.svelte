<script>
	let editMode = $state(false);

	let { userDescription } = $props();
</script>

<span class="font-bold">About</span>
<br>
{#if editMode }
	<form>
		<textarea class="h-52 w-full justify-start"></textarea>
		<button class="rounded-3xl bg-blue-200 py-0.5 px-2">✓ Confirm</button>
	</form>
{:else }
	<p>{userDescription}</p>
{/if}
<br>