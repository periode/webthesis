<script lang="ts">
    import text_data from "./../text.json";
    import Node from "./../components/Node.svelte";
    import DarkMode from "./../components/DarkMode.svelte";
    import type { INode } from "../../utils/types";
    import { findNodeByValue } from "../../utils/find";

    let data = text_data as Array<INode>;
    let root = findNodeByValue(data, "ideals.tex");

    let document_children = root ? (root.children ? root.children : []) : [];
</script>

<div
    class="container max-w-full bg-zinc-50 text-zinc-800 dark:bg-zinc-900 dark:text-zinc-50 lg:p-3 font-serif text-xl"
>
    {#each document_children as node}
        <Node {node} />
    {/each}
</div>
