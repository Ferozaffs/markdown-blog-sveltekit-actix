<script>
	export let text = 'Button';
	export let callback = () => {}

</script>

<button on:click={callback} class="md:w-24 w-18 text-primary-color primary-color font-medium rounded-full text-sm px-5 py-2.5 mr-2 mb-2">{text}</button>
