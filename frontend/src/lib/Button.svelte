<script>
	export let text = 'Button';
	export let callback = () => {}

</script>

<button on:click={callback} class="min-w-24 text-gray-400 bg-gray-800 hover:bg-gray-900 font-medium rounded-full text-sm px-5 py-2.5 mr-2 mb-2 dark:bg-gray-800 dark:hover:bg-gray-700">{text}</button>
