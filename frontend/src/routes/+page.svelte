<script>
	import ButtonRow from '$lib/ButtonRow.svelte';
    import ArticlesListView from '$lib/ArticlesListView.svelte';
    import About from '$lib/About.svelte';
	import ProjectsOverview from '$lib/ProjectsOverview.svelte';
	import { currentContent } from '$lib/store.js';

    const ContentArea = {
        Articles: "articles",
        Projects: "projects",
        About: "about"
    } 

    function showArticles(){
        $currentContent = ContentArea.Articles;
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
        <p class="h-24 text-5xl text-gray-400">Markdown blog</p>
    </div>
    
    <div class="flex h-fit justify-center bg-gray-600">
        <ButtonRow buttonNames={['Articles', 'Projects', 'About']} buttonCallbacks={[showArticles, showProjects, showAbout]} />
    </div>
    
    <div class="flex-1 bg-gray-600">
        {#if $currentContent === ContentArea.Articles}
            <ArticlesListView/>
        {:else if $currentContent === ContentArea.Projects}
            <ProjectsOverview/>
        {:else}
            <About/>
        {/if}
    </div>
</div>
