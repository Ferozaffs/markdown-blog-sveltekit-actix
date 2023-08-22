<script>
    export let items = [['Item 1', false], ['Item 2', false], ['Item 3', false]];
    let visible = false;

    function toggleList() {
        visible = !visible;
    }
</script>

<button on:click={() => toggleList()} class="w-32 {visible? "border border-gray-800" : ""} text-gray-800 hover:text-gray-400 font-medium rounded-md text-sm px-5 py-2.5 mr-2 mb-2">Filter ▼</button>
{#if visible == true}
    <div class="w-32 absolute border border-2 border-gray-600 bg-slate-800 rounded-md text-gray-400 text-sm font-medium">
        {#each items as [item, state]}
            <li class="list-none px-5 py-2.5">
                <label>
                    <input type="checkbox" bind:checked={state} class="accent-gray-400"/> {item}
                </label>
            </li>
        {/each}
    </div>
{/if}