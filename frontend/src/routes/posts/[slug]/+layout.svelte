<script lang="ts">
    import { goto } from '$app/navigation';

    export let data;

    function gotoProject(projectId: string) {
        goto("/projects/".concat(projectId));
    }

    import { currentTags, currentContent } from '$lib/store.js';
    import { ContentArea } from '$lib/Constants.svelte';
    function gotoPosts(tag: string) {
        $currentTags = tag;
        $currentContent = ContentArea.Posts;
        goto("/");
    }

</script>

<div class="flex flex-col h-screen overflow-hidden">
    <div class="w-full bg-gray-800 h-fit py-4 text-xs text-gray-500">
    <table class="w-full">
    <tr>
    <td class="w-20">
        <button on:click={() => goto("/")} class="px-10 text-4xl text-gray-400">🠨</button>
    </td>
    <td>
        <p class="text-xl text-gray-400">{data.post.title}</p>   
        <span>Go to project:
            <button on:click={() => gotoProject(data.project.id)} class="px-1 text-gray-600 hover:text-gray-300 hover:bg-gray-500 hover:rounded-md">{data.project.name}</button>
        </span>
        <span class="px-3"> Tags:          
            {#each data.post.tags.split(",") as tag}
            <button on:click={() => gotoPosts(tag)} class="px-1 text-gray-600 hover:text-gray-300 hover:bg-gray-500 hover:rounded-md">{tag}</button>
            {/each}
        </span>
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