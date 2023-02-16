<script lang="ts">
    import type { INode } from "../../../utils/types";
    export let node: INode;
    const value = node.children ? node.children[0].value : "Missing citation";
    const citations = value.split(",")
</script>

<span class="font-medium">
    &nbsp;({#each citations as cit}
            <span class="citation">{cit}</span>{/each})&nbsp;
</span>

<style>
    .citation ~ .citation::before{
        content: ", ";
    }
</style>
