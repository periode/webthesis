<script lang="ts">
    import type { INode } from "../../utils/types";
    import Node from "./Node.svelte";
    export let node: INode;

    const children = node.children ? node.children : [{tag:node.tag, value: node.value, children: null} as INode];
</script>

<div class="m-5 italic border-l-2">
    {#each children as node}
        <Node {node} />
    {/each}
</div>
