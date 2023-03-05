<script lang="ts">
    import type { PageData } from "./$types";
    import { page } from "$app/stores";
    import Node from "../components/Node.svelte";
    import NotFound from "../components/NotFound.svelte";
    import Navigation from "../components/Navigation.svelte";

    const chapter = $page.params.chap;
    export let data: PageData;
    const nodes = data.nodes;
</script>

<div
    class="container max-w-full bg-zinc-50 text-zinc-800 dark:bg-zinc-900 dark:text-zinc-50 p-2 lg:p-3 font-serif md:text-lg"
>
    {#if nodes.length > 0}
        {#each nodes as node}
            <Node {node} />
        {/each}
        <Navigation chapter={`chap:${chapter}`}/>
    {:else}
        <NotFound status="404" />
    {/if}
</div>
