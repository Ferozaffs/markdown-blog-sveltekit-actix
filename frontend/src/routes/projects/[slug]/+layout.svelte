<script>
        import { goto } from '$app/navigation';

        export let data;
</script>

<div class="flex flex-col h-screen overflow-hidden">
        <div class="w-full primary-color h-14 text-xl text-primary-color">
        <table>
        <tr>
        <td>
                <button on:click={() => goto("/")} class="px-10 text-4xl">🠨</button>
        </td>
        <td>
                <span>{data.title}</span>
                {#if data.status === 0}
                        <span class="px-2 text-primary-color">[ONGOING]</span>
                {:else if data.status === 1}
                        <span class="px-2 text-green-600 text-base">[COMPLETED]</span>
                {/if}
        </td>
        </tr>
        </table>                 
        </div>       
        <div class="flex-grow h-full w-full overflow-y-auto primary-color">
                <div class="p-20 font-medium text-primary-color prose max-w-screen-lg prose-lg">
                        <slot />
                </div>
        </div>   
</div>