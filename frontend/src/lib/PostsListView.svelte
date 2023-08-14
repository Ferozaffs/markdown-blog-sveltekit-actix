<script>
    import PostSnippet from '$lib/PostsSnippet.svelte';
    import { BarLoader } from 'svelte-loading-spinners';
    //import { goto } from '$app/navigation';

    async function getPosts() {
        const response = await self.fetch("http://127.0.0.1:8080/posts/*")
        if (response.ok) {
  		    let data = response.json();	
            console.log(data);
            return data;	
		} 	
    }
    const promise = getPosts();

    //function gotoPost(postId: string) {
     //   goto("/posts/".concat(postId))
    //}
</script>

{#await promise}
    <div class="flex justify-center items-center">
        <BarLoader size="80" color="#2d3342 " unit="px" duration="1s" />
    </div>
{:then po}
    <div class="w-full px-20 grid content-center gap-10 md:grid-cols-2 s:grid-cols-1 xl:grid-cols-4">
        {#each po as post}
            <PostSnippet title={post.title} image={post.image}/>
        {/each}
    </div>
{:catch error}
    <p class="px-20 font-medium text-gray-400">{error}</p>
{/await}