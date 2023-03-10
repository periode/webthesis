<script lang="ts">
    import type { INode } from "../../utils/types";
    import Node from "../components/Node.svelte";
    
    export let nodes: INode[];
    export let name: string = "";
</script>

<div class={name}>
    {#each nodes as node (node)}
        <Node {node} />
    {/each}
</div>
