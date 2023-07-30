<script>
    import { BarLoader } from 'svelte-loading-spinners';

    async function getProjectOverview() {
        const response = await self.fetch("http://localhost:8080/projectoverview")
        if (response.ok) {
  		    let data = response.json();	
            console.log(data);
            return data;	
		} 	
    }
    const promise = getProjectOverview();
</script>

{#await promise}
<div class="flex justify-center items-center">
    <BarLoader size="80" color="#2d3342 " unit="px" duration="1s" />
</div>
{:then po}
    <div class="w-full px-20 content-center grid gap-10 sm:grid-cols-1 md:grid-cols-2 xl:grid-cols-4">
        {#each po.categories as category}
            <div class="font-medium text-gray-400">
                <p class="text-2xl">{category.title}</p>
                <p class="text-xs px-1">{category.description}</p>
                <br>
                <ul>
                    {#each category.children as child}
                        <li>
                            <div class="hover:bg-gray-900 dark:hover:bg-gray-700 rounded-lg">
                                <img src="/images/testImage.jpg" alt="" class="display: inline border-2 border-gray-800 dark:border-gray-800 w-12 h-12 rounded-lg overflow-hidden"/>
                                <span class="px-1">{child.name}</span>
                                <span>[{child.status}]</span>
                            </div>
                        </li>
                    {/each}
                </ul>
            </div>
        {/each}
</div>
{:catch error}
<p class="px-20 font-medium text-gray-400">{error}</p>
{/await}

