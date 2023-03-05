<script lang="ts">
    import type { PageData } from "./$types";
    import { page } from "$app/stores";
    import Node from "../components/Node.svelte";
    import NotFound from "../components/NotFound.svelte";
    import Navigation from "../components/Navigation.svelte";
    import TableOfContents from "../components/TableOfContents.svelte";

    const chapter = $page.params.chap;
    export let data: PageData;
    const nodes = data.nodes;
    const toc = data.toc;
</script>

<div
    class="container max-w-full bg-zinc-50 text-zinc-800 dark:bg-zinc-900 dark:text-zinc-50 p-2 lg:p-3 font-serif md:text-lg"
>
    {#if nodes.length > 0}
        {#each nodes as node}
            <Node {node} />
        {/each}
        <div class="w-full lg:w-6/12 md:w-8/12 m-auto pt-12">
            <h3 class="my-4 text-2xl">Sections</h3>
            <TableOfContents {toc} />
        </div>
        
        <Navigation chapter={`chap:${chapter}`}/>
    {:else}
        <NotFound status="404" />
    {/if}
</div>
