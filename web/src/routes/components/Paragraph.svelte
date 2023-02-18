<script lang="ts">
    import { NodeType, type ICitation, type INode } from "../../utils/types";
    import Node from "./Node.svelte";
    import CitationItem from "./CitationItem.svelte";
    import Literal from "./inline/Literal.svelte";
    import Emph from "./inline/Emph.svelte";
    import Citation from "./inline/Citation.svelte";
    import InlineListing from "./inline/InlineListing.svelte";
    import Footnote from "./Footnote.svelte";
    import Reference from "./inline/Reference.svelte";
    import Dots from "./inline/Dots.svelte";

    export let node: INode;
    const nodes = node.children ? node.children : [];
    let citations: Array<ICitation> = [];

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

    const handleCitation = (event: CustomEvent<ICitation>) => {
        citations.push(event.detail);
        citations = citations;
    };

    const handleShowCitation = (event: CustomEvent<string>) => {
        citations.forEach(c => {
            if(c.id === event.detail)
                c.visible = !c.visible;
        });

        citations = citations;
    }
</script>

<div class="m-1 md:m-2 leading-9">
    {#if isLiteralParagraph(nodes)}
        <div class="flex justify-between">
            <div class="w-full lg:w-6/12 md:w-8/12 mb-1 indent-12">
                {#each nodes as node}
                    {#if node.tag == NodeType.Literal}
                        <Literal {node} />
                    {:else if node.tag === NodeType.Emphasis || node.tag === NodeType.Italic}
                        <Emph {node} />
                    {:else if node.tag === NodeType.Citation}
                        <Citation {node} on:citation={handleCitation} on:showcitation={handleShowCitation}/>
                    {:else if node.tag === NodeType.Dots}
                        <Dots />
                    {:else if node.tag === NodeType.Reference}
                        <Reference {node} />
                    {:else if node.tag === NodeType.InlineListing}
                        <InlineListing {node} />
                    {:else if node.tag === NodeType.Footnote}
                        <Footnote {node} />
                    {:else}
                        <b>nope! {node.tag}</b>
                    {/if}
                {/each}
            </div>
            <div class="flex flex-col justify-center lg:w-3/12 text-base overflow-y-scroll">
                {#each citations as citation}
                    <CitationItem {citation}/>
                {/each}
            </div>
        </div>
    {:else}
        {#each nodes as node}
            <Node {node} />
        {/each}
    {/if}
</div>
