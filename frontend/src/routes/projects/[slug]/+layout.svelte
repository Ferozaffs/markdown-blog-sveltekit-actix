<script>
        import { goto } from '$app/navigation';
</script>

<div class="flex flex-col h-screen overflow-hidden">
        <div class="w-full bg-gray-800 h-14">
                <button on:click={() => goto("/")} class="px-10 text-5xl text-gray-400">🠨</button>
        </div>       
        <div class="flex-grow h-full w-full overflow-y-auto bg-gray-600">
                <div class="px-20 font-medium text-gray-400 prose max-w-fit prose-lg">
                        <slot />
                </div>
        </div>   
</div>