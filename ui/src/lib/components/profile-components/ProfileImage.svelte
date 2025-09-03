<script lang="ts">

	import ImageChangeModal from '$lib/components/ImageChangeModal.svelte';
	import PencilIcon from '$lib/components/icons/PencilIcon.svelte';


	let editImageModal: any;

	let { userId } = $props();


</script>
<div class="relative size-56 m-6 mx-auto mb-2">
	<button class="cursor-pointer" onclick={() => editImageModal.open()}>
		<img class="size-56 md:object-contain rounded-4xl"
				 src="http://localhost:3000/api/image/{userId}"
				 alt="Profile">
		<!-- Overlay Icon -->
		<span class="size-10 p-2 bg-orange-200 text-orange-900 -bottom-2 -right-2 aspect-square rounded-full absolute">
			<PencilIcon />
		</span>
	</button>
</div>

<ImageChangeModal {userId} bind:this={editImageModal} />
