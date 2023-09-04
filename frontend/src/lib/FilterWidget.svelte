<script lang="ts">
    export let items = [['Item 1', false], ['Item 2', false], ['Item 3', false]];
    export let filterCallback = () => {};
    let visible = false;

    function toggleList() {
        visible = !visible;
    }

    export function setItems(newItems: string[]) {
        items = [];
        for (let i = 0; i < newItems.length; i++) {      
            items.push([newItems[i], false])
        }
    }

    export function setActiveItems(activeItems: string[]) {
        for (let i = 0; i < items.length; i++) {      
            if (activeItems.includes(items[i][0])) {
                items[i][1] = true;
            }
            else {
                items[i][1] = false;
            }
        }
    }

    export function getActiveItems() {
        let activeItems = [];
        for (let i = 0; i < items.length; i++) {  
            if (items[i][1] === true) {
                activeItems.push(items[i][0]);
            }
        }
   
        return activeItems;
    }

    $: items && filterCallback();

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