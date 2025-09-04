<script>
	let { errorCode, errorMessage } = $props();
</script>

<div class="flex flex-col min-h-full justify-center items-center text-center grow">
	<span id="notice">There has been an error:</span>
	<span id="error">{errorCode} - {errorMessage}</span>
</div>

<style>
    #notice {
        font-size: 4rem;
        font-weight: bold;
    }

    #error {
        font-size: 3rem;
    }
</style>