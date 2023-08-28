<script>
	import ButtonRow from '$lib/ButtonRow.svelte';
    import PostsListView from '$lib/PostsListView.svelte';
    import About from '$lib/About.svelte';
	import ProjectOverview from '$lib/ProjectOverview.svelte';
	import { currentContent, currentTags } from '$lib/store.js';
    import { ContentArea } from '$lib/Constants.svelte';
	import FilterWidget from '$lib/FilterWidget.svelte';

    let tags;
    let filterWidget;
    $: tags && loadTags();

    function loadTags() {
        filterWidget.setItems(tags);
    }

    function showArticles(){
        $currentContent = ContentArea.Posts;
        $currentTags = []
    }     
    function showProjects(){
        $currentContent = ContentArea.Projects;
    }
    function showAbout(){
        $currentContent = ContentArea.About;
    }

</script>

<div class="flex h-screen flex-col">
    <div class="flex justify-center items-center bg-gray-800">
        <p class="py-5 h-24 text-5xl text-gray-400">Markdown blog</p>
    </div>
    
    <div class="bg-gray-600">
    <table class="w-full table-fixed">
        <tr>
        <td class="w-1/6"/>
        <td class="flex h-fit justify-center">
            <ButtonRow buttonNames={['Posts', 'Projects', 'About']} buttonCallbacks={[showArticles, showProjects, showAbout]} />
        </td>
        <td class="w-1/6">
            {#if $currentContent === ContentArea.Posts}
                <FilterWidget bind:this="{filterWidget}"/>
            {/if}
        </td>
        </tr>
    </table>    
    </div>
    
    <div class="flex-1 bg-gray-600">
        {#if $currentContent === ContentArea.Posts}
            <PostsListView bind:storedTags={tags}/>
        {:else if $currentContent === ContentArea.Projects}
            <ProjectOverview/>
        {:else}
            <About/>
        {/if}
    </div>
</div>
