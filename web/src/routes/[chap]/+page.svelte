<script lang="ts">
    import { page } from "$app/stores";
    import Node from "../components/Node.svelte";
    import { findNodeByValue } from "../../utils/find";
    import type { INode } from "../../utils/types";
    import NotFound from "../components/NotFound.svelte";

    const chapter = $page.params.chap;
    const root = findNodeByValue(`${chapter}.tex`);
    const nodes = root ? (root.children as Array<INode>) : [];
</script>

<div
    class="container max-w-full bg-zinc-50 text-zinc-800 dark:bg-zinc-900 dark:text-zinc-50 p-2 lg:p-3 font-serif md:text-lg"
>
    {#if nodes.length > 0}
        {#each nodes as node}
            <Node {node} />
        {/each}
    {:else}
        <NotFound status="404" />
    {/if}
</div>
