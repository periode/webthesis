<script lang="ts">
    import Chapter from "./headers/Chapter.svelte";
    import Error from "./Error.svelte";
    import Section from "./headers/Section.svelte";
    import Subsection from "./headers/Subsection.svelte";
    import Subsubsection from "./headers/Subsubsection.svelte";
    import Listing from "./Listing.svelte";
    import List from "./List.svelte";
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
    import Footnote from "./Footnote.svelte";
    import Spacer from "./Spacer.svelte";
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
        { tag: NodeType.Citation, component: Citation },
        { tag: NodeType.Code, component: Listing },
        { tag: NodeType.Figure, component: Figure },
        { tag: NodeType.Label, component: Label },
        { tag: NodeType.Quote, component: Quote },
        { tag: NodeType.Footnote, component: Footnote },
        { tag: NodeType.Reference, component: Reference },
        { tag: NodeType.URL, component: URL },
        { tag: NodeType.InlineListing, component: InlineListing },
        { tag: NodeType.Title, component: Title },
        { tag: NodeType.Author, component: Author },
        { tag: NodeType.Affiliation, component: Affiliation },
        { tag: NodeType.Date, component: Date },
        { tag: NodeType.Spacer, component: Spacer },
    ];

    const n = nodes.find((n) => n.tag === node.tag);
    const component = n ? n.component : Error;
</script>

<svelte:component this={component} {node} />
