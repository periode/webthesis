<script lang="ts">
    import { NodeType, type INode } from "../../utils/types";
    import Node from './Node.svelte';
    import Literal from "./inline/Literal.svelte";
    import Emph from "./inline/Emph.svelte";
    import Citation from "./inline/Citation.svelte";
    import InlineListing from "./inline/InlineListing.svelte";
    import Footnote from "./inline/Footnote.svelte";
    import Reference from "./inline/Reference.svelte";

    export let node: INode;
    const children = node.children ? node.children : [];

    const isLiteralParagraph = (children: Array<INode> | null) => {
        if (!children || children.length === 0) return false; // no children
        if (children.length === 1 && children[0].tag === NodeType.Literal)
            // a single line paragraph
            return true;
        else if (
            // a paragraph that starts either with a literal or an emphasis
            children.length > 1 &&
            (children[0].tag === NodeType.Literal ||
                children[0].tag === NodeType.Emphasis)
        )
            return true;
        else return false;
    };
</script>

<div class="m-1 md:m-2 leading-8">
    {#if !isLiteralParagraph(children)}
        <!-- <div>({children ? children.length : "none"})</div> -->
        {#each children as child}
            <Node node={child}/>
        {/each}
        <!--         
        {#each children as child}
            
            {#if child.tag === NodeType.Root}
                <svelte:self children={child.children} />
            {:else if child.tag === NodeType.Include}
                <Include children={child.children} />
            {:else if child.tag === NodeType.Title}
                <Title node={child} />
            {:else if child.tag === NodeType.Author}
                {#if child.children}
                    <Author node={child} />
                {/if}
            {:else if child.tag === NodeType.Affiliation}
                {#if child.children}
                    <Affiliation node={child} />
                {/if}
            {:else if child.tag === NodeType.Date}
                {#if child.children}
                    <Date node={child} />
                {/if}
            {:else if child.tag === NodeType.Chapter}
                <Chapter node={child} />
            {:else if child.tag === NodeType.Section}
                <Section node={child} />
            {:else if child.tag === NodeType.Subsection}
                <Subsection node={child} />
            {:else if child.tag === NodeType.Subsubsection}
                <Subsubsection node={child} />
            {:else if child.tag === NodeType.Listing}
                <Listing node={child} />
            {:else if child.tag === NodeType.Quote}
                <Quote node={child} />
            {:else if child.tag === NodeType.Emphasis}
                {#if child.children}
                    <Emph node={child} />
                {/if}
            {:else if child.tag === NodeType.Label}
                {#if child.children}
                    <Label node={child} />
                {/if}
            {:else if child.tag === NodeType.Paragraph}
                <svelte:self children={child.children} />
            {:else}
                <Error tag={child.tag}>
                    <svelte:self children={child.children} />
                </Error>
            {/if}
        {/each} -->
    {:else}
        <div class="w-full lg:w-5/12 md:w-8/12 mb-1 indent-12">
            {#each children as child}
                {#if child.tag == NodeType.Literal}
                    <Literal node={child} />
                {:else if (child.tag === NodeType.Emphasis || child.tag === NodeType.Italic)}
                    <Emph node={child} />
                {:else if child.tag === NodeType.Citation}
                    <Citation node={child} />
                {:else if child.tag === NodeType.Reference}
                    <Reference node={child} />
                {:else if child.tag === NodeType.InlineListing}
                    <InlineListing node={child} />
                {:else if child.tag === NodeType.Footnote}
                    <Footnote node={child} />
                {:else}
                    <b>nope! {child.tag}</b>
                {/if}
            {/each}
        </div>
    {/if}
</div>
