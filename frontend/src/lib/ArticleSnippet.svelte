<script>
    export let title = 'Title';
    export let date = '1970-01-01';
    export let description = 'This is some template description'
    export let image = '/images/testImage.jpg';
    export let imageAlt ='ImageDescription';

</script>

<div class="text-xs h-fit w-fit text-gray-400 bg-gray-800 hover:bg-gray-900 font-medium rounded-lg px-5 py-2.5 mr-2 mb-2 dark:bg-gray-800 dark:hover:bg-gray-700">
    <img src={image} alt={imageAlt} class="w-fit"/>
    <p class="text-xl">{title}</p>
    <p class="text-xs">{date}</p>
    <br>
    <p class="text-xs">{description}</p>
</div>