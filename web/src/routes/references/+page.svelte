<script lang="ts">
    import CitationItem from "../../components/appendix/CitationItem.svelte";
    import type { PageData } from "../references/$types";

    export let data: PageData;
</script>

<div
    class="container max-w-full bg-zinc-50 text-zinc-800 dark:bg-zinc-900 dark:text-zinc-50 p-2 lg:p-3 font-serif md:text-lg"
>
    <h1 class="w-full lg:w-5/12 md:w-6/12 m-auto text-4xl mt-24 mb-8">
        References
    </h1>
    <hr class="w-8/12 lg:w-5/12 md:w-6/12 m-auto my-10" />

    <div class="w-full lg:w-5/12 md:w-6/12 m-auto">
        {#each data.citations as citation}
            <CitationItem {citation} />
        {/each}
    </div>
</div>
