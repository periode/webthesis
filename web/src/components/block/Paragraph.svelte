<script lang="ts">
    import {
        NodeType,
        type ICitation,
        type IFootnote,
        type INode,
    } from "../../utils/types";
    import CitationItem from "../block/CitationItem.svelte";
    import FootnoteItem from "./FootnoteItem.svelte";
    import Literal from "../inline/Literal.svelte";
    import Emph from "../inline/Emph.svelte";
    import Citation from "../inline/Citation.svelte";
    import InlineListing from "../inline/InlineListing.svelte";
    import Footnote from "../inline/Footnote.svelte";
    import Reference from "../inline/Reference.svelte";
    import Dots from "../inline/Dots.svelte";
    import Quote from "./Quote.svelte";
    import Url from "../inline/URL.svelte";
    import Superscript from "../inline/Superscript.svelte";
    import List from "./List.svelte";

    export let node: INode;
    const nodes = node.children ? node.children : [];
    let citations: Array<ICitation> = [];
    let footnotes: Array<IFootnote> = [];

    const handleFootnote = (event: CustomEvent<IFootnote>) => {
        footnotes.push(event.detail);
        footnotes = footnotes;
    };

    const handleShowFootnote = (event: CustomEvent<string>) => {
        footnotes.forEach((f) => {
            if (event.detail === `footnote-${f.value}`) f.visible = !f.visible;
        });

        footnotes = footnotes;
    };

    const handleHighlightFootnote = (event: CustomEvent<string>) => {
        footnotes.forEach((f) => {
            if (event.detail === `footnote-${f.value}`)
                f.highlighted = !f.highlighted;
        });

        footnotes = footnotes;
    };

    const handleCitation = (event: CustomEvent<ICitation>) => {
        citations.push(event.detail);
        citations = citations;
    };

    const handleShowCitation = (event: CustomEvent<string>) => {
        citations.forEach((c) => {
            if (c.id === event.detail) c.visible = !c.visible;
        });

        citations = citations;
    };

    const handleHighlightCitation = (event: CustomEvent<string>) => {
        citations.forEach((c) => {
            if (c.id === event.detail) c.highlighted = !c.highlighted;
        });

        citations = citations;
    };
</script>

<div
    class="relative md:flex justify-between m-1 md:m-2 leading-6 md:leading-8 md:text-body"
>
    <div
        class="absolute top-0 h-full hidden md:visible md:h-auto md:relative md:flex md:flex-col justify-center lg:w-3/12 text-base overflow-y-visible pointer-events-none"
    >
        {#each footnotes as footnote}
            <FootnoteItem {footnote} />
        {/each}
    </div>
    <div class="w-full lg:w-5/12 md:w-6/12 m-auto mb-1 indent-12 hyphens">
        {#each nodes as node}
            {#if node.tag == NodeType.Literal}
                <Literal {node} />
            {:else if node.tag === NodeType.Emphasis || node.tag === NodeType.Italic}
                <Emph {node} />
            {:else if node.tag === NodeType.Citation}
                <Citation
                    {node}
                    on:citation={handleCitation}
                    on:showcitation={handleShowCitation}
                    on:highlightcitation={handleHighlightCitation}
                />
            {:else if node.tag === NodeType.Dots}
                <Dots />
            {:else if node.tag === NodeType.Superscript}
                <Superscript {node} />
            {:else if node.tag === NodeType.Reference}
                <Reference {node} />
            {:else if node.tag === NodeType.InlineListing}
                <InlineListing {node} />
            {:else if node.tag === NodeType.List}
                <List {node} />
            {:else if node.tag === NodeType.URL}
                <Url {node} />
            {:else if node.tag === NodeType.Quote}
                <Quote
                    {node}
                    on:citation={handleCitation}
                    on:showcitation={handleShowCitation}
                    on:highlightcitation={handleHighlightCitation}
                />
            {:else if node.tag === NodeType.Footnote}
                <Footnote
                    {node}
                    on:footnote={handleFootnote}
                    on:showfootnote={handleShowFootnote}
                    on:highlightfootnote={handleHighlightFootnote}
                />
            {:else}
                <b>nope! {node.tag}</b>
            {/if}
        {/each}
    </div>
    <div
        class="absolute top-0 h-full hidden md:visible md:h-auto md:relative md:flex md:flex-col justify-center lg:w-3/12 text-base overflow-y-visible pointer-events-none"
    >
        {#each citations as citation}
            <CitationItem {citation} />
        {/each}
    </div>
</div>
