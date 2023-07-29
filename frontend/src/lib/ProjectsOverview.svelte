<script>
    import { onMount } from "svelte";

    const Category = {
        title: "Category",
        description: "This category contains children",
        children:[
            "Child 1",
            "Child 2",
        ]
    }

    const category1 = Object.create(Category);
    category1.title = "Category 1";
    const category2 = Object.create(Category);
    category2.title = "Category 2";
    const category3 = Object.create(Category);
    category3.title = "Category 3";
    const category4 = Object.create(Category);
    category4.title = "Category 4";

    let categories = [category1, category2, category3, category4];

    onMount(async () => {
        fetch("http://localhost:8080/projectoverview")
        .then(response => response.json())
        .then(data => {
                console.log(data);
        }).catch(error => {
            console.log(error);
            return [];
        });
    });
</script>

<div class="w-full px-20 content-center grid gap-10 sm:grid-cols-1 md:grid-cols-2 xl:grid-cols-4">
    {#each categories as category}
        <div class="font-medium text-gray-400">
            <p class="text-2xl">{category.title}</p>
            <p class="text-xs">{category.description}</p>
            <br>
            <ul class="list-disc px-4">
                {#each category.children as child}
                    <li>{child}</li>
                {/each}
            </ul>
        </div>
    {/each}
</div>

