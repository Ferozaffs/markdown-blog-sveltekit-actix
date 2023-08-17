<script lang="ts">
    export let id = '12345'
    export let title = 'Title';
    export let date = '1970-01-01';
    export let image = 'Image.jpg';
    export let imageAlt ='ImageDescription';
    export let description = 'Description';
    export let tags = ['tag1', 'tag2'];

    import { goto } from '$app/navigation';
    function gotoPost(postId: string) {
        goto("/post/".concat(postId))
    }

    import { currentTags } from '$lib/store.js';
    function setTag(tag: string) {
         $currentTags = tag;
    }
</script>

<div class="h-fit w-fit text-gray-500 bg-gray-800 hover:bg-gray-900 font-medium rounded-lg px-5 py-2.5 mr-2 mb-2 dark:bg-gray-800 dark:hover:bg-gray-700">
    <button on:click={() => gotoPost(id)} class="h-fit w-fit" >
        <img src={"http://localhost:8080/images/".concat(image)}  alt={imageAlt} class="w-fit"/>
        <p class="text-xl text-gray-400">{title}</p>
        <p class="py-2 text-left">{description}</p>
    </button>
    <table class="w-full">
        <tr>
            <td>
                <p class="text-xs text-left">{date}</p>
            </td>
            <td class="flex items-end justify-end">
                {#each tags as tag}
                    <button on:click={() => setTag(tag)} class="px-1 hover:text-gray-300 hover:bg-gray-500 hover:rounded-md text-xs">{tag}</button>
                {/each}
            </td>
        </tr>
    </table>
</div>

