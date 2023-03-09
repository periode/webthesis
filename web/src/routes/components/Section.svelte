<script lang="ts">
    import type { INode } from "../../utils/types";
    import Node from "../components/Node.svelte";
    
    export let nodes: INode[];
</script>

<div>
    {#each nodes as node (node)}
        <Node {node} />
    {/each}
</div>
