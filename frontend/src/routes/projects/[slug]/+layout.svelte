<script>
        import { goto } from '$app/navigation';

        export let data;
</script>

<div class="flex flex-col h-screen overflow-hidden">
        <div class="w-full bg-gray-800 h-14 text-xl text-gray-400">
        <table>
        <tr>
        <td>
                <button on:click={() => goto("/")} class="px-10 text-4xl">🠨</button>
        </td>
        <td>
                <span>{data.name}</span>
                {#if data.status === 0}
                        <span class="px-2 text-gray-500">[ONGOING]</span>
                {:else if data.status === 1}
                        <span class="px-2 text-green-600 text-base">[COMPLETED]</span>
                {/if}
        </td>
        </tr>
        </table>                 
        </div>       
        <div class="flex-grow h-full w-full overflow-y-auto bg-gray-600">
                <div class="p-20 font-medium text-gray-400 prose max-w-screen-lg prose-lg">
                        <slot />
                </div>
        </div>   
</div>