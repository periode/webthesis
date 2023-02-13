<script lang="ts">
    import { NodeType, type INode } from "../../utils/types";
    import Chapter from "./headers/Chapter.svelte";
    import Error from "./Error.svelte";
    import Section from "./headers/Section.svelte";
    import Subsection from "./headers/Subsection.svelte";
    import Subsubsection from "./headers/Subsubsection.svelte";
    import Listing from "./Listing.svelte";
    import Quote from "./Quote.svelte";
    import Include from "./Include.svelte";
    import Label from "./Label.svelte";
    import Literal from "./inline/Literal.svelte";
    import Emph from "./inline/Emph.svelte";
    import Citation from "./inline/Citation.svelte";
    import InlineListing from "./inline/InlineListing.svelte";
    import Footnote from "./inline/Footnote.svelte";
    import Reference from "./inline/Reference.svelte";
    import Title from "./Title.svelte";
    import Author from "./Author.svelte";
    import Affiliation from "./Affiliation.svelte";
    import Date from "./Date.svelte";

    export let children: Array<INode> | null;

    const isLiteralParagraph = (children: Array<INode> | null) => {
        if (!children) return false; // no children
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
    {#if children && !isLiteralParagraph(children)}
        <!-- <div>({children ? children.length : "none"})</div> -->
        {#each children as child}
            <!-- {child.value} -->
            {#if child.tag === NodeType.Root}
                <svelte:self children={child.children} />
            {:else if child.tag === NodeType.Include}
                <Include children={child.children} />
            {:else if child.tag === NodeType.Title}
                {#if child.children}
                    <Title value={child.children[0].value} />
                {/if}
            {:else if child.tag === NodeType.Author}
                {#if child.children}
                    <Author value={child.children[0].value} />
                {/if}
            {:else if child.tag === NodeType.Affiliation}
                {#if child.children}
                    <Affiliation value={child.children[0].value} />
                {/if}
            {:else if child.tag === NodeType.Date}
                {#if child.children}
                    <Date timestamp={child.value} />
                {/if}
            {:else if child.tag === NodeType.Chapter}
                <Chapter children={child.children} />
            {:else if child.tag === NodeType.Section}
                <Section children={child.children} />
            {:else if child.tag === NodeType.Subsection}
                <Subsection children={child.children} />
            {:else if child.tag === NodeType.Subsubsection}
                <Subsubsection children={child.children} />
            {:else if child.tag === NodeType.Listing}
                <Listing
                    children={child.children}
                    tag={child.tag}
                    value={child.value}
                />
            {:else if child.tag === NodeType.Quote}
                <Quote children={child.children} />
            {:else if child.tag === NodeType.Emphasis}
                {#if child.children}
                    <Emph value={child.children[0].value} />
                {/if}
            {:else if child.tag === NodeType.Label}
                {#if child.children}
                    <Label value={child.children[0].value} />
                {/if}
            {:else if child.tag === NodeType.Paragraph}
                <svelte:self children={child.children} />
            {:else}
                <Error tag={child.tag}>
                    <svelte:self children={child.children} />
                </Error>
            {/if}
        {/each}
    {:else if children}
        <div class="w-full lg:w-5/12 md:w-8/12 mb-1 indent-12">
            {#each children as child}
                {#if child.tag == NodeType.Literal}
                    <Literal value={child.value} />
                {:else if (child.tag === NodeType.Emphasis || child.tag === NodeType.Italic) && child.children}
                    <Emph value={child.children[0].value} />
                {:else if child.tag === NodeType.Citation && child.children}
                    <Citation value={child.children[0].value} />
                {:else if child.tag === NodeType.Reference && child.children}
                    <Reference value={child.children[0].value} />
                {:else if child.tag === NodeType.InlineListing && child.children}
                    <InlineListing
                        value={child.children.map((c) => c.value).join("")}
                    />
                {:else if child.tag === NodeType.Footnote && child.children}
                    <Footnote
                        value={child.children.map((c) => c.value).join("")}
                    />
                {:else}
                    <b>nope! {child.tag}</b>
                {/if}
            {/each}
        </div>
    {/if}
</div>
