<script>
	import ButtonRow from '$lib/ButtonRow.svelte';
</script>

<div class="flex h-24 justify-center items-center bg-gray-800">
    <p class="text-5xl text-gray-400">Markdown blog</p>
</div>

<div class="flex h-fit justify-center bg-gray-600">
    <ButtonRow buttons={['Blog', 'Projects', 'About']} />
</div>