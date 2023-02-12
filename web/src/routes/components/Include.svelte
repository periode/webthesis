<script lang="ts">
    import type { INode } from "../../utils/types";
    import Paragraph from "./Paragraph.svelte";

    export let children: Array<INode> | null;
</script>

<div class="border-b-2 pb-48 border-black">
    {#if children}
        <Paragraph {children} />
    {/if}
</div>
