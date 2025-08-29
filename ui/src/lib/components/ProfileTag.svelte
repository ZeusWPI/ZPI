<script>
	let { tag } = $props();
</script>


<span class="border-2 rounded-2xl p-1 px-3 mx-1" style="--color: {tag.color};">
	{tag.label}
</span>

<style>
	span {
			color: rgb(var(--color));
			border-color: rgb(var(--color));
			background-color: rgba(var(--color), 0.25);
	}
</style>