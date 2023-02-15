<script lang="ts">
    import type { INode } from "../../utils/types";
    export let node: INode;
    const tag = node ? node.tag : "unknown tag!"
</script>

<div class="border-red-600 border text-black p-1">
    could not render node <b>{tag}</b>
</div>