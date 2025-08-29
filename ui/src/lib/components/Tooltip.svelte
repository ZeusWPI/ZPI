<script>
	let { text, children } = $props();
	let tooltipShown = $state(false);

	function showTooltip() {
		tooltipShown = true;
	}

	function hideTooltip() {
		tooltipShown = false;
	}
</script>

<div
	class="tooltip-container"
	onmouseenter={showTooltip}
	onfocus={showTooltip}
	onmouseleave={hideTooltip}
	onblur={hideTooltip}
	role="img"
>
	{@render children()}

	{#if tooltipShown}
		<div class="tooltip">
			{text}
		</div>
	{/if}
</div>

<style>
    .tooltip-container {
        position: relative;
        display: inline-block;
    }

    .tooltip {
        position: absolute;
        bottom: 100%;
        left: 50%;
        transform: translateX(-50%);
        background: black;
        color: white;
        padding: 4px 8px;
        border-radius: 3px;
        font-size: 12px;
        white-space: nowrap;
        margin-bottom: 5px;
    }

    .tooltip::after {
        content: "";
        position: absolute;
        top: 100%;
        left: 50%;
        transform: translateX(-50%);
        border: 4px solid transparent;
        border-top-color: black;
    }
</style>