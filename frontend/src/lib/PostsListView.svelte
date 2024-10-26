<script lang="ts">
    import PostSnippet from '$lib/PostsSnippet.svelte';
    import { BarLoader } from 'svelte-loading-spinners';
    import { currentTags } from '$lib/store.js';
    import { PUBLIC_API_URL } from '$env/static/public'

    let promise = getPosts();
    export let storedTags = [];

    async function getPosts() {    
        const response = await self.fetch(PUBLIC_API_URL.concat("/posts/*"))
        if (response.ok) {
  		    let data = await response.json();

            let foundTags = [];
            for (let i = 0; i < data.length; i++){
                storeTags(foundTags, data[i].tags.split(","))
            }
            storedTags = foundTags;
	
            return data;	
		} 	
    }

    function filterPost(tags: string) {
        let splitTags = tags.split(",");

        if (splitTags.some(r=> $currentTags.includes(r))) {
            return true;
        }

        return false;
    }

    function storeTags(foundTags: string[], tags: string[]) {
        
        for(let i = 0; i < tags.length; i++) {
            if (foundTags.indexOf(tags[i]) === -1) {
                foundTags.push(tags[i])
            }
        }
    }

    export function getTags() {
        return storedTags;
    }

</script>

{#await promise}
    <div class="flex justify-center items-center">
        <BarLoader size="80" color="#2d3342 " unit="px" duration="1s" />
    </div>
{:then po}
    <div class="w-full px-20 grid content-center gap-10 md:grid-cols-2 s:grid-cols-1 xl:grid-cols-4">
        {#each po as post}
            {#if $currentTags.length === 0 || filterPost(post.tags)}
                <PostSnippet 
                    id={post.id}
                    title={post.title} 
                    image={post.image} 
                    date={post.date} 
                    description={post.description}
                    tags={post.tags.split(",")}
                />                         
            {/if}
        {/each}
    </div>
{:catch error}
    <p class="px-20 font-medium text-primary-color">{error}</p>
{/await}