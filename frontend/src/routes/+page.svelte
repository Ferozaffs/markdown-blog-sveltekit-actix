<script>
	import ButtonRow from '$lib/ButtonRow.svelte';
    import PostsListView from '$lib/PostsListView.svelte';
    import About from '$lib/About.svelte';
	import ProjectOverview from '$lib/ProjectOverview.svelte';
	import { currentContent, currentTags } from '$lib/store.js';
    import { ContentArea } from '$lib/Constants.svelte';
	import FilterWidget from '$lib/FilterWidget.svelte';
    import { onMount } from 'svelte';
    import { BarLoader } from 'svelte-loading-spinners';

    let tags;
    let filterWidget = null
    $: tags && loadTags();
    $: $currentTags && updateFilter();

    function loadTags() {    
        if (filterWidget !== null)
        {
            filterWidget.setItems(tags);
        }
    }

    function filterTags() {  
        if (filterWidget !== null)
        {
            let items = filterWidget.getActiveItems();
            if (items.toString() !== $currentTags.toString() ) {
                $currentTags = items;
            }
        }
    }

    function updateFilter() {
        if (filterWidget !== null)
        {
            filterWidget.setActiveItems($currentTags);
        }
    }

    function showArticles(){
        $currentContent = ContentArea.Posts;
        $currentTags = [];
    }     
    function showProjects(){
        $currentContent = ContentArea.Projects;
    }
    function showAbout(){
        $currentContent = ContentArea.About;
    }

    let title = ''
    let about = ''

    let themeLoaded = false
    onMount(async () => {
        await loadTheme();
        themeLoaded = true
    });

    async function loadTheme() {
        const response = await fetch('/configs/theme.json');
        const data = await response.json();

        document.documentElement.style.setProperty('--background-primary-color', data.backgroundprimary);
        document.documentElement.style.setProperty('--background-secondary-color', data.backgroundsecondary);
        document.documentElement.style.setProperty('--primary-color', data.primary);
        document.documentElement.style.setProperty('--primary-color-hover', data.primaryhover);
        document.documentElement.style.setProperty('--secondary-color', data.secondary);
        document.documentElement.style.setProperty('--secondary-color-hover', data.secondaryhover);
        document.documentElement.style.setProperty('--text-primary-color', data.textprimary);
        document.documentElement.style.setProperty('--text-primary-color-hover', data.textprimaryhover);
        document.documentElement.style.setProperty('--text-secondary-color', data.textsecondary);

        title = data.title;
        about = data.about;
    }

</script>

{#if themeLoaded == false}
    <div class="flex justify-center items-center">
        <BarLoader size="80" color="#2d3342 " unit="px" duration="1s" />
    </div>
{:else}
<div class="flex h-screen flex-col">
    <div class="bg-secondary-color flex justify-center items-center">
        <p class="py-5 h-24 text-5xl text-secondary-color">{title}</p>
    </div>
    
    <div class="bg-primary-color">
    <table class="w-full table-fixed">
        <tr>
        <td class="md:w-1/6 w-1/12"/>
        <td class="flex h-fit justify-center">
            <ButtonRow buttonNames={['Posts', 'Projects', 'About']} buttonCallbacks={[showArticles, showProjects, showAbout]} />
        </td>
        <td class="md:w-1/6 w-1/5">
            {#if $currentContent === ContentArea.Posts}
                <FilterWidget filterCallback={filterTags} bind:this="{filterWidget}"/>
            {/if}
        </td>
        </tr>
    </table>  
    </div>
    
    <div class="flex-1 bg-primary-color">
        {#if $currentContent === ContentArea.Posts}
            <PostsListView bind:storedTags={tags}/>
        {:else if $currentContent === ContentArea.Projects}
            <ProjectOverview/>
        {:else}
            <About bind:about={about}/>
        {/if}
    </div>
</div>
{/if}
