<script lang="ts">
    import { BarLoader } from 'svelte-loading-spinners';
    import { goto } from '$app/navigation';

    async function getProjectOverview() {
        const response = await self.fetch("http://localhost:8080/projectoverview")
        if (response.ok) {
  		    let data = response.json();	
            console.log(data);
            return data;	
		} 	
    }
    const promise = getProjectOverview();

    function gotoProject(projectId: string) {
        goto("/projects/".concat(projectId))
    }
    
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
                <p class="text-xs">{category.description}</p>
                <br>
                <ul>
                    {#each category.children as child}
                        <li>
                            <button on:click={() => gotoProject(child.id)} class="hover:bg-gray-900 dark:hover:bg-gray-700 rounded-lg w-full text-left">
                                <table>
                                    <tr>
                                        <td>
                                            <img src={"http://localhost:8080/images/".concat(child.image)} alt="" class="display: inline border-2 border-gray-800 dark:border-gray-800 w-12 h-12 rounded-lg overflow-hidden"/>
                                        </td>
                                        <td>
                                            <p class="px-1">{child.name}</p>
                                            {#if child.status === 0}
                                                <p class="px-1 text-gray-500 text-sm">[ONGOING]</p>
                                            {:else if child.status === 1}
                                                <p class="px-1 text-green-600 text-sm">[COMPLETED]</p>
                                            {/if}
                                        </td>
                                    </tr>
                                </table>
                            </button>
                        </li>
                    {/each}
                </ul>
            </div>
        {/each}
</div>
{:catch error}
<p class="px-20 font-medium text-gray-400">{error}</p>
{/await}

