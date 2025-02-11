<script>
    import PostSnippet from '$lib/PostsSnippet.svelte';
    import { BarLoader } from 'svelte-loading-spinners';
      import { PUBLIC_API_URL } from '$env/static/public'

  export let data;

  let promise = getPosts();
  async function getPosts() {    
        const response = await self.fetch(PUBLIC_API_URL.concat("/posts/*"))
        if (response.ok) {
  		    return await response.json();	
    }
  }

</script>

<table>
  <tr>
    <td class="flex-col justify-center items-center">
      <div class="p-20">{@html data.content}</div>
    </td>
    <td class="px-5 border-x-2 align-top min-w-36">
      <div class="min-w-36 text-xl">Posts</div>
      {#await promise}
      <div class="">
          <BarLoader size="80" color="#2d3342 " unit="px" duration="1s" />
      </div>
      {:then po}
          <div class="py-5">
              {#each po as post}
                  {#if post.project_id === data.slug}
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
          <p class="font-medium text-primary-color">{error}</p>
      {/await}
    </td>
  </tr>
</table>






