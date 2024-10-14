<script>
    import ButtonDefault from '$lib/ButtonDefault.svelte';
	export let buttonNames = ['Button 1', 'Button 2'];
    export let buttonCallbacks = [() => {}, () => {}]
</script>

<ul class="p-4 overflow-hidden">
    {#each buttonNames as buttonText, index}
        <li class="md:px-2 md:min-w-24 w-fit inline-block">
            <ButtonDefault text={buttonText} callback={buttonCallbacks[index]}/>
        </li>
    {/each}
</ul>