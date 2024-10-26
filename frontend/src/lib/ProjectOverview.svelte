<script lang="ts">
    import { BarLoader } from 'svelte-loading-spinners';
    import { goto } from '$app/navigation';
    import { PUBLIC_API_URL } from '$env/static/public'

    async function getProjectOverview() {
        const response = await self.fetch(PUBLIC_API_URL.concat("/projectoverview"))
        if (response.ok) {
  		    let data = response.json();	
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
            <div class="font-medium text-secondary-color">
                <p class="text-2xl">{category.title}</p>
                <p class="text-xs">{category.description}</p>
                <br>
                <ul>
                    {#each category.children as child}
                        <li>
                            <button on:click={() => gotoProject(child.id)} class="primary-color text-primary-color rounded-lg w-full text-left">
                                <table>
                                    <tr>
                                        <td>
                                            <img src={PUBLIC_API_URL.concat("/images/").concat(child.image)} alt="" class="display: inline border-2 border-gray-800 dark:border-gray-800 w-12 h-12 rounded-lg overflow-hidden"/>
                                        </td>
                                        <td>
                                            <p class="px-1">{child.title}</p>
                                            {#if child.status === 0}
                                                <p class="px-1 text-secondary-color text-sm">[ONGOING]</p>
                                            {:else if child.status === 1}
                                                <p class="px-1 text-secondary-color text-sm">[COMPLETED]</p>
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
<p class="px-20 font-medium text-primary-color">{error}</p>
{/await}

