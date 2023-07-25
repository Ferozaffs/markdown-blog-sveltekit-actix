<script>
    import Button from '$lib/Button.svelte';
	export let buttons = ['Button 1', 'Button 2', 'Button 3'];
</script>

<ul class="p-4 whitespace-nowrap no-scrollbar overflow-x-auto overflow-y-hidden">
    {#each buttons as buttonText}
        <li class="px-2 min-w-24 w-fit inline-block">
            <Button text={buttonText}/>
        </li>
    {/each}
</ul>