<script lang="ts">
    import type { PageData } from "./$types";
    import { page } from "$app/stores";
    import Node from "../../components/Node.svelte";
    import NotFound from "../../components/NotFound.svelte";
    import Navigation from "../../components/Navigation.svelte";

    export let data: PageData;
    const chap = $page.params.chap;
    const sec = $page.params.sec;

    const chap_literal = data.chapter;
    const nodes = data.nodes;
</script>

<div
    class="container max-w-full bg-zinc-50 text-zinc-800 dark:bg-zinc-900 dark:text-zinc-50 p-2 lg:p-3 font-serif md:text-lg"
>
    {#if nodes.length > 0}
        <div
            class="w-full lg:w-6/12 md:w-8/12 m-auto text-base mt-24 mb-8 italic"
        >
            <a href={`/${chap}`}>{chap_literal}</a>
        </div>
        {#each nodes as node}
            <Node {node} />
        {/each}
        <Navigation chapter={`chap:${chap}`} section={`sec:${sec}`}/>
    {:else}
        <NotFound status="404" />
    {/if}
</div>
