<script lang="ts">
    import type { INode } from "../../utils/types";
    import Paragraph from "./Paragraph.svelte";

    export let node: INode;
</script>

<div class="border-b-2 pb-48 border-black">
    <Paragraph {node} />
</div>
