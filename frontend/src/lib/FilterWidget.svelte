<script lang="ts">
    export let items = [];
    export let filterCallback = () => {};
    let visible = false;

    function toggleList() {
        visible = !visible;
    }

    export function setItems(newItems: string[]) {
        for (let i = 0; i < newItems.length; i++) {      
            const newItem = newItems[i];
        
            const exists = items.some(item => item[0] === newItem);

            if (!exists) {
                items.push([newItem, false]);
            }
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

<button on:click={() => toggleList()} class="{visible? "border bg-secondary-color" : ""} border-0 text-primary-color font-medium rounded-md text-sm px-5 py-2.5 mr-2 mb-2">Filter ▼</button>
{#if visible == true}
    <div class="absolute border-2 border-zinc-900 bg-secondary-color rounded-md text-secondary-color text-sm font-medium">
        {#each items as [item, state]}
            <li class="list-none px-5 py-2.5">
                <label>
                    <input type="checkbox" bind:checked={state} class="accent-zinc-400"/> {item}
                </label>
            </li>
        {/each}
    </div>
{/if}