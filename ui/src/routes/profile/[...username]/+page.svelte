<script lang="ts">
	import type { PageData } from './$types';
	import { Avatar, Heading, Hr } from 'flowbite-svelte';

	let { data }: { data: PageData } = $props();
	let user = $derived(data.user);
</script>

<div class="grid grid-cols-7 w-6/7 gap-x-12 gap-y-4">
	<Avatar src="/image/{data.user.id}" class="col-span-2 justify-self-end size-72 rounded rounded-3xl" border />
	<Heading class="col-span-5 self-end pb-3 text-7xl">{user.username}</Heading>

	<Hr class="col-span-7 mx-60 my-3" />

	<div class="col-span-2">
		<!-- About -->
	</div>

	<div class="col-span-2">
		<!-- Achievements -->
	</div>
</div>