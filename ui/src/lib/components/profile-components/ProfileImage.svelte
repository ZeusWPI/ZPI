<script lang="ts">

	import ImageChangeModal from '$lib/components/profile-components/ImageChangeModal.svelte';
	import PencilIcon from '$lib/components/icons/PencilIcon.svelte';
	import { PUBLIC_BACKEND_URL } from '$env/static/public';
	import { setContext } from 'svelte';


	let editImageModal: any = $state();

	let { userId, editAllowed } = $props();

	let imgSrc = $state(`${PUBLIC_BACKEND_URL}/api/image/${userId}`);

	function reloadImage() {
		const currentSrc = imgSrc.split('?')[0];
		imgSrc = currentSrc + '?t=' + new Date().getTime();

		console.log('Reloaded Image');
	}

	setContext('imageReload', { reloadImage });

</script>
{#if editAllowed}
	<div class="relative size-56 m-6 aspect-square mx-auto mb-2">
		<button class="cursor-pointer" onclick={() => editImageModal.open()}>
			<img class="size-56 md:object-contain rounded-4xl"
					 src="{imgSrc}"
					 alt="Profile">
			<!-- Overlay Icon -->
			<span
				class="size-10 p-2 bg-orange-200 hover:bg-orange-300 text-orange-900 -bottom-2 -right-2 aspect-square rounded-xl absolute">
			<PencilIcon />
		</span>
		</button>
	</div>

	<ImageChangeModal {userId} {imgSrc} bind:this={editImageModal} />

{:else}
	<div class="relative size-56 m-6 mx-auto mb-2">
		<img id="profile-image"
				 class="size-56 md:object-contain rounded-4xl"
				 src="{imgSrc}"
				 alt="Profile">
	</div>
{/if}