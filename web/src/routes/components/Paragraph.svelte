<script lang="ts">
    import { NodeType, type INode } from "../../utils/types";
    import Chapter from "./Chapter.svelte";
    import Listing from "./Listing.svelte";

    export let children: Array<INode> | null;

    //if the first child is not an environment or a command
    //if there is more than one child?
    let literal_content = "";
    if (children && (children.length > 1 || children[0].tag === NodeType.Literal)) {
        children.map((c) => (literal_content += c.value));
    }
</script>

<div class="paragraph">
    {#if children && literal_content === ""}
        <div>({children ? children.length : "none"})</div>
        {#each children as child}
            {child.value}
            {#if child.tag === NodeType.Chapter}
                <Chapter children={child.children} />
            {:else if child.tag === NodeType.Listing}
                <Listing
                    children={child.children}
                    tag={child.tag}
                    value={child.value}
                />
            {:else}
                <svelte:self
                    children={child.children}
                />
            {/if}
        {/each}
    {:else}
        {literal_content}
    {/if}
</div>

<style>
    .paragraph {
        margin-bottom: 0.5em;
        text-indent: 1em;
    }
</style>
