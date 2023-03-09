<script lang="ts">
    import Chapter from "./headings/Chapter.svelte";
    import Error from "./Error.svelte";
    import Section from "./headings/Section.svelte";
    import Subsection from "./headings/Subsection.svelte";
    import Subsubsection from "./headings/Subsubsection.svelte";
    import Listing from "./block/Listing.svelte";
    import List from "./block/List.svelte";
    import Figure from "./block/Figure.svelte";
    import Quote from "./block/Quote.svelte";
    import Include from "./block/Include.svelte";
    import Label from "./block/Label.svelte";
    import Paragraph from "./block/Paragraph.svelte";
    import Literal from "./inline/Literal.svelte";
    import Emph from "./inline/Emph.svelte";
    import Dots from "./inline/Dots.svelte";
    import Citation from "./inline/Citation.svelte";
    import InlineListing from "./inline/InlineListing.svelte";
    import Footnote from "./inline/Footnote.svelte";
    import Spacer from "./block/Spacer.svelte";
    import Reference from "./inline/Reference.svelte";
    import URL from "./inline/URL.svelte";
    import { NodeType, type INode } from "../../utils/types";

    export let node: INode;

    const nodes = [
        { tag: NodeType.Root, component: Paragraph },
        { tag: NodeType.Paragraph, component: Paragraph },
        { tag: NodeType.Include, component: Include },
        { tag: NodeType.Chapter, component: Chapter },
        { tag: NodeType.Listing, component: Listing },
        { tag: NodeType.List, component: List },
        { tag: NodeType.Literal, component: Literal },
        { tag: NodeType.Section, component: Section },
        { tag: NodeType.Subsection, component: Subsection },
        { tag: NodeType.Subsubsection, component: Subsubsection },
        { tag: NodeType.Emphasis, component: Emph },
        { tag: NodeType.Italic, component: Emph },
        { tag: NodeType.Dots, component: Dots },
        { tag: NodeType.Citation, component: Citation },
        { tag: NodeType.Code, component: Listing },
        { tag: NodeType.Figure, component: Figure },
        { tag: NodeType.Label, component: Label },
        { tag: NodeType.Quote, component: Quote },
        { tag: NodeType.Footnote, component: Footnote },
        { tag: NodeType.Reference, component: Reference },
        { tag: NodeType.URL, component: URL },
        { tag: NodeType.InlineListing, component: InlineListing },
        { tag: NodeType.Spacer, component: Spacer },
    ];

    const n = nodes.find((n) => n.tag === node.tag);
    const component = n ? n.component : Error;
</script>

<svelte:component this={component} {node} />
