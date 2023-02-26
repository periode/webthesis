<script lang="ts">
    import text_data from "./../text.json";
    import Node from "./../components/Node.svelte";
    import type { INode } from "../../utils/types";
    import { findNodeByValue } from "../../utils/find";
    import { onMount } from "svelte";

    let data = text_data as Array<INode>;
    let root = findNodeByValue(data, "introduction.tex");

    let document_children = root ? (root.children ? root.children : []) : [];
    
    onMount(() => {
        console.log(location.hash);
        
        if(location.hash !== "")
            document.getElementById(location.hash.substring(1))?.scrollIntoView()
    })
</script>

<div
    class="container max-w-full bg-zinc-50 text-zinc-800 dark:bg-zinc-900 dark:text-zinc-50 lg:p-3 font-serif text-xl"
>
    {#each document_children as node}
        <Node {node} />
    {/each}
</div>
