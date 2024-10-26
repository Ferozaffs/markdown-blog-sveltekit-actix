<script lang="ts">
    import { PUBLIC_API_URL } from '$env/static/public'
    
    export let id = '12345'
    export let title = 'Title';
    export let date = '1970-01-01';
    export let image = 'Image.jpg';
    export let imageAlt ='ImageDescription';
    export let description = 'Description';
    export let tags = ['tag1', 'tag2'];

    import { goto } from '$app/navigation';
    function gotoPost(postId: string) {
        goto("/posts/".concat(postId));
    }

    import { currentTags } from '$lib/store.js';
    function setTag(tag: string) {
         $currentTags = [tag];
    }
</script>

<div class="h-fit w-fit primary-color  font-medium rounded-lg px-5 py-2.5 mr-2 mb-2 dark:primary-color">
    <button on:click={() => gotoPost(id)} class="h-fit w-fit" >
        <img src={PUBLIC_API_URL.concat("/images/").concat(image)}  alt={imageAlt} class="w-fit"/>
        <p class="text-xl text-primary-color">{title}</p>
        <p class="py-2 text-secondary-color text-left">{description}</p>
    </button>
    <table class="w-full">
        <tr>
            <td>
                <p class="text-secondary-color text-xs text-left">{date}</p>
            </td>
            <td class="flex items-end justify-end">
                {#each tags as tag}
                    <button on:click={() => setTag(tag)} class="px-1 m-1 text-primary-color secondary-color rounded-md text-xs">{tag}</button>
                {/each}
            </td>
        </tr>
    </table>
</div>

