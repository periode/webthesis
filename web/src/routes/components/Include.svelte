<script lang="ts">
    import type { INode } from "../../utils/types";
    import Paragraph from "./Paragraph.svelte";

    export let node: INode;
</script>

<div class="pb-12 md:pb-24 md:border-none">
    <Paragraph {node} />
</div>
