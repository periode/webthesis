<script lang="ts">
    import json_data from "./data.json";
    import Node from "./components/Node.svelte";
    import type { INode } from "../utils/types";

    let data = json_data as Array<INode>;
    let nodes = data[0].children as Array<INode>;
    let root_children = nodes[0].children as Array<INode>;
    let document_children = root_children[0].children as Array<INode>;
</script>

<div class="container max-w-full bg-slate-50 text-slate-800 lg:p-3 font-serif">
    {#each document_children as node}
        <Node {node} />
    {/each}
</div>
