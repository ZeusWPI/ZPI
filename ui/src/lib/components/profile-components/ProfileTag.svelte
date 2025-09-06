<script lang="ts">
	let { tag } = $props();

	let tagTypeColors = new Map([
		["bestuur", "255, 128, 0"],
		["bestuursfunctie", '255, 0, 0'],
		["topper", "0, 180, 0"]
	])

	const getColor = (tagCategory:string) => tagTypeColors.get(tagCategory) || "160, 160, 160"
</script>


<span class="border-2 rounded-2xl px-3 mx-1 inline-block" style="--color: {getColor(tag.category)};">
	{tag.name}
</span>

<style>
	span {
			color: rgb(var(--color));
			border-color: rgb(var(--color));
			background-color: rgba(var(--color), 0.25);
	}
</style>