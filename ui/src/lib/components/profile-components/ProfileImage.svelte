<script lang="ts">
	import { createQuery, type CreateQueryResult } from '@tanstack/svelte-query';
	import { browser } from '$app/environment';
	import { onMount } from 'svelte';
	import type { Query } from '@tanstack/svelte-query';
	import Modal from '$lib/components/Modal.svelte';

	let editImageModal: any;

	const profileImg = 'https://zpi.zeus.gent/image/385';

	let { userId } = $props();


</script>
<button onclick={() => editImageModal.open()}>
	<img class="size-56 md:object-contain rounded-4xl m-6 mx-auto mb-2" src="https://zpi.zeus.gent/image/{userId}"
			 alt="Profile">
</button>

<Modal title="Edit Profile Image" bind:this={editImageModal}>
	<img class="size-20 rounded-xl" src="https://zpi.zeus.gent/image/{userId}" alt="Preview" />
</Modal>