<script lang="ts">
    import type { INode } from "../../utils/types";
    export let node: INode;
    const value = node.children ? node.children[0].value.split(":")[1] : "Missing label";    
</script>

<div
    class="relative -top-28 text-zinc-400 my-6"
    id={`${encodeURIComponent(value)}`}
/>
