<script lang="ts">
    import type { INode } from "../../utils/types";
    import Node from "../components/Node.svelte";

    export let node: INode;
    const children = node.children as Array<INode>;
</script>

<div class="my-20">
    <h3 class="w-full lg:w-6/12 md:w-8/12 m-auto mb-4 text-2xl">Abstract</h3>
    {#each children as child}
        <Node node={child} />
    {/each}
</div>
