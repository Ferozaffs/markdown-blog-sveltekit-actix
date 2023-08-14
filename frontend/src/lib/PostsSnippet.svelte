<script lang="ts">
    export let title = 'Title';
    export let date = '1970-01-01';
    export let image = 'Image.jpg';
    export let imageAlt ='ImageDescription';

    import { goto } from '$app/navigation';
    function gotoPost(postId: string) {
        goto("/post/".concat(postId))
    }
</script>

<button on:click={() => gotoPost(title)} class="text-xs h-fit w-fit text-gray-400 bg-gray-800 hover:bg-gray-900 font-medium rounded-lg px-5 py-2.5 mr-2 mb-2 dark:bg-gray-800 dark:hover:bg-gray-700">
    <img src={"http://127.0.0.1:8080/images/".concat(image)}  alt={imageAlt} class="w-fit"/>
    <p class="text-xl">{title}</p>
    <p class="text-xs">{date}</p>
</button>