<script>
    import ButtonDefault from '$lib/ButtonDefault.svelte';
	export let buttonNames = ['Button 1', 'Button 2'];
    export let buttonCallbacks = [() => {}, () => {}]
</script>

<ul class="p-4 whitespace-nowrap no-scrollbar overflow-x-auto overflow-y-hidden">
    {#each buttonNames as buttonText, index}
        <li class="px-2 min-w-24 w-fit inline-block">
            <ButtonDefault text={buttonText} callback={buttonCallbacks[index]}/>
        </li>
    {/each}
</ul>