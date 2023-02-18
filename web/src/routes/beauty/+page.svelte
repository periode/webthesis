<script lang="ts">
    import json_data from "./../data.json";
    import Node from "./../components/Node.svelte";
    import DarkMode from "./../components/DarkMode.svelte";
    import type { INode } from "../../utils/types";
    import { findNodeByValue } from "../../utils/find";

    let data = json_data as Array<INode>;
    let root = findNodeByValue(data, "beauty.tex")
    console.log(root);
    
    let document_children = root ? root.children ? root.children : [] : []
    
</script>

<div class="container max-w-full bg-zinc-50 text-zinc-800 dark:bg-zinc-800 dark:text-zinc-50 lg:p-3 font-serif text-xl">
    {#each document_children as node}
        <Node {node} />
    {/each}
    <DarkMode />
</div>
