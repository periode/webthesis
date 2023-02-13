<script lang="ts">
    import type { INode } from "../../utils/types";
    import Paragraph from "./Paragraph.svelte";
    export let node: INode;

    const children = node.children ? node.children : [];
</script>

<div class="m-5 italic border-l-2">
    <Paragraph node={node} />
</div>
