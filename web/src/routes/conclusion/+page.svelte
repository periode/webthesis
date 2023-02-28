<script lang="ts">
    import Node from "./../components/Node.svelte";
    import { findNodeByValue } from "../../utils/find";
    import type { INode } from "../../utils/types";

    const root = findNodeByValue("conclusion.tex") as INode;
    const nodes = root.children as Array<INode>;
</script>

<div class="container max-w-full bg-zinc-50 text-zinc-800 dark:bg-zinc-900 dark:text-zinc-50 p-2 lg:p-3 font-serif text-xl">
    {#each nodes as node}
        <Node {node} />
    {/each}
</div>
