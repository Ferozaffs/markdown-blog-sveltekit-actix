<script lang="ts">
    import PostSnippet from '$lib/PostsSnippet.svelte';
    import { BarLoader } from 'svelte-loading-spinners';
    import { currentTags } from '$lib/store.js';

    let promise = getPosts();

    async function getPosts() {    
        const response = await self.fetch("http://localhost:8080/posts/".concat($currentTags))
        if (response.ok) {
  		    let data = response.json();	
            console.log(data);
            return data;	
		} 	
    }

    //Only trigger if tag is NOT *
    $: $currentTags !== "*" ? reload() : null;

    async function reload() {
        promise = getPosts();
    }

</script>

{#await promise}
    <div class="flex justify-center items-center">
        <BarLoader size="80" color="#2d3342 " unit="px" duration="1s" />
    </div>
{:then po}
    <div class="w-full px-20 grid content-center gap-10 md:grid-cols-2 s:grid-cols-1 xl:grid-cols-4">
        {#each po as post}
            <PostSnippet 
                id={post.id}
                title={post.title} 
                image={post.image} 
                date={post.date} 
                description={post.description}
                tags={post.tags.split(",")}
            />
        {/each}
    </div>
{:catch error}
    <p class="px-20 font-medium text-gray-400">{error}</p>
{/await}