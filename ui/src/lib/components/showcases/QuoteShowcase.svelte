<script>
	let { showcaseProperties } = $props();
</script>

<fieldset class="flex flex-col border-solid border-2 rounded-xl border-gray-300 w-full p-3">
	<legend class="font-bold text-center ms-4">Quotes</legend>
	{#each showcaseProperties.quotes as quote}
		<blockquote>{quote}</blockquote>
	{/each}
</fieldset>

<style>
    @font-face {
        font-family: "Fira Sans";
        font-weight: normal;
        src: local("Fira Sans"), local("Fira Sans Regular"),
        url(/fonts/FiraSans-Regular.otf) format("opentype"),
        url(/fonts/FiraSans-Regular.woff2) format("woff2"),
        url(/fonts/FiraSans-Regular.woff) format("woff"),
        url(/fonts/FiraSans-Regular.svg) format("svg");
    }

    @font-face {
        font-family: "Fira Sans";
        font-weight: bold;
        src: local("Fira Sans"), local("Fira Sans Bold"),
        url(/fonts/FiraSans-Bold.otf) format("opentype"),
        url(/fonts/FiraSans-Bold.woff2) format("woff2"),
        url(/fonts/FiraSans-Bold.woff) format("woff"),
        url(/fonts/FiraSans-Bold.svg) format("svg");
    }

    blockquote::before {
        content: "“";
        font-size: 600%;
        position: absolute;
        color: #ccc;
        top: -0.1em;
        left: -0.2em;
        z-index: 1;
    }

    blockquote {
        position: relative;
        margin: 0 0.2em 2em 1em;
        padding: 1em 1em 0.9em 1.7em;
        background-color: #f8f8f8;
        box-shadow: 0 0.5em 1em rgba(0, 0, 0, 0.05);
        transition: box-shadow 0.2s, background-color 0.2s;
    }

    * {
        font-family: "Fira Sans", sans-serif;
        color: #333;
        text-rendering: optimizeLegibility;
    }


</style>