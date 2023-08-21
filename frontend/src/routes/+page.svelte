<script>
	import ButtonRow from '$lib/ButtonRow.svelte';
    import PostsListView from '$lib/PostsListView.svelte';
    import About from '$lib/About.svelte';
	import ProjectOverview from '$lib/ProjectOverview.svelte';
	import { currentContent } from '$lib/store.js';
    import { ContentArea } from '$lib/Constants.svelte';

    function showArticles(){
        $currentContent = ContentArea.Posts;
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
    
    <div class="flex h-fit justify-center bg-gray-600">
        <ButtonRow buttonNames={['Posts', 'Projects', 'About']} buttonCallbacks={[showArticles, showProjects, showAbout]} />
    </div>
    
    <div class="flex-1 bg-gray-600">
        {#if $currentContent === ContentArea.Posts}
            <PostsListView/>
        {:else if $currentContent === ContentArea.Projects}
            <ProjectOverview/>
        {:else}
            <About/>
        {/if}
    </div>
</div>
