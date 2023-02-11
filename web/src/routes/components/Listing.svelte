<script lang="ts">
    import Paragraph from "./Paragraph.svelte";
    import { NodeType, type INode } from "../../utils/types";

    export let tag: String;
    export let value: String;
    export let children: Array<INode> | null;
</script>

<div class="w-full font-mono break-words">
    <div>env: {tag} - {value} ({children ? children.length : "none"})</div>
    {#if children}
        {#each children as child}
            {child.value}
            {#if child.tag === NodeType.Literal}
                <div>
                    {child.value}
                </div>
            {:else}
                <Paragraph
                    children={child.children}
                />
            {/if}
        {/each}
    {/if}
</div>