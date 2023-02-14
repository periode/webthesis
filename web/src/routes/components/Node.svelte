<script lang="ts">
    import Chapter from "./headers/Chapter.svelte";
    import Error from "./Error.svelte";
    import Section from "./headers/Section.svelte";
    import Subsection from "./headers/Subsection.svelte";
    import Subsubsection from "./headers/Subsubsection.svelte";
    import Listing from "./Listing.svelte";
    import Figure from "./Figure.svelte";
    import Quote from "./Quote.svelte";
    import Include from "./Include.svelte";
    import Label from "./Label.svelte";
    import Title from "./Title.svelte";
    import Author from "./Author.svelte";
    import Affiliation from "./Affiliation.svelte";
    import Date from "./Date.svelte";
    import Paragraph from "./Paragraph.svelte";
    import Literal from "./inline/Literal.svelte";
    import Emph from "./inline/Emph.svelte";
    import Citation from "./inline/Citation.svelte";
    import InlineListing from "./inline/InlineListing.svelte";
    import Footnote from "./inline/Footnote.svelte";
    import Reference from "./inline/Reference.svelte";
    import type { INode } from "../../utils/types";

    export let node: INode;

    const nodes = [
        { tag: "root", component: Paragraph },
        { tag: "paragraph", component: Paragraph },
        { tag: "include", component: Include },
        { tag: "chapter", component: Chapter },
        { tag: "listing", component: Listing },
        { tag: "literal", component: Literal },
        { tag: "section", component: Section },
        { tag: "subsection", component: Subsection },
        { tag: "subsubsection", component: Subsubsection },
        { tag: "emph", component: Emph },
        { tag: "italic", component: Emph },
        { tag: "citation", component: Citation },
        { tag: "code", component: Listing },
        { tag: "figure", component: Figure },
        { tag: "label", component: Label },
        { tag: "include", component: Include },
        { tag: "quote", component: Quote },
        { tag: "footnote", component: Footnote },
        { tag: "reference", component: Reference },
        { tag: "inline_listing", component: InlineListing },
        { tag: "title", component: Title },
        { tag: "author", component: Author },
        { tag: "affiliation", component: Affiliation },
        { tag: "date", component: Date },
    ];

    const n = nodes.find((n) => n.tag === node.tag);
    const component = n ? n.component : Error;
</script>

<svelte:component this={component} {node} />
