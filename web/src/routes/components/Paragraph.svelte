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

    const concatenateLiteralContent = (nodes: Array<INode>) => {
        let c = "";

        nodes.map((n) => {
            if (n.tag == NodeType.Literal) {
                c += n.value;
            } else if (n.tag === NodeType.Emphasis && n.children) {
                c += ` <span class="italic">${n.children[0].value}</span>`;
            } else if (n.tag === NodeType.Citation && n.children) {
                c += ` (<span class="bold">${n.children[0].value}</span>)`;
            } else if (n.tag === NodeType.Reference && n.children) {
                c += ` (<b class="text">${n.children[0].value}</b>) `;
            } else if (n.tag === NodeType.InlineListing && n.children) {
                c += `<span class="text-sm">${n.children
                    .map((c) => c.value)
                    .join(" ")}</span>`;
            } else if (n.tag === NodeType.Footnote && n.children) {
                c += `<sup class="text-sm">${n.children
                    .map((c) => c.value)
                    .join(" ")}</sup>`;
            } else {
                c = `<b>nope! ${n.tag}</b>`;
            }
        });

        return c;
    };

    let literal_content: string | null = null;
    if (isLiteralParagraph(children)) {
        literal_content = concatenateLiteralContent(children as INode[]);
    }
</script>

<div class="paragraph m-2">
    {#if children && literal_content === null}
        <!-- <div>({children ? children.length : "none"})</div> -->
        {#each children as child}
            <!-- {child.value} -->
            {#if child.tag === NodeType.Root}
                <svelte:self children={child.children} />
            {:else if child.tag === NodeType.Include}
                <Include children={child.children} />
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
    {:else}
        <div class="lg:w-5/12 md:w-8/12 mb-1 indent-12">
            {@html literal_content}
        </div>
    {/if}
</div>
