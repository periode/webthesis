<script lang="ts">
    import type { INode } from "../../utils/types";
    import Node from "./Node.svelte";
    export let node: INode;

    const children = node.children ? node.children : [{tag:node.tag, value: node.value, children: null} as INode];
</script>

<div class="w-2/3 m-4 ml-24 border-l pl-4 italic text-zinc-800 dark:text-zinc-300">
    {#each children as node}
        <Node {node} />
    {/each}
</div>
