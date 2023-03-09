<script lang="ts">
    import bib from "../../../data/bib.json";
    import type { ICitation, INode } from "../../../utils/types";
    import { createEventDispatcher, onMount } from "svelte";
    import isMobile from "../../../utils/helper";
    import CitationItem from "../block/CitationItem.svelte";

    const dispatch = createEventDispatcher<{ citation: ICitation }>();
    const dispatchToggle = createEventDispatcher<{ showcitation: string }>();
    const dispatchHighlight = createEventDispatcher<{
        highlightcitation: string;
    }>();

    export let node: INode;
    const value = node.children ? node.children[0].value : "Missing citation";
    const keys = value.split(",");
    let citations: Array<ICitation> = [];
    let isVisible = !isMobile();

    const short = keys.map((k) => {
        const b = bib.find((b) => b.id === k);
        const typed = b ? (b as unknown as ICitation) : undefined;

        if (typed === undefined)
            return { author: "author", year: "year", id: "id" };

        typed.visible = false;
        citations.push(typed);

        let author = "UNKNOWN AUTHOR",
            editor = "UNKNOWN EDITOR";
        if (typed.author !== undefined)
            author = typed.author
                ? typed.author[0]
                    ? typed.author[0].family
                        ? typed.author[0].family
                        : typed.author[0].literal
                    : "UNKNOWN NAME"
                : "UNKNOWN AUTHOR";
        else if (typed.editor !== undefined)
            editor = typed.editor
                ? typed.editor[0]
                    ? typed.editor[0].family
                        ? typed.editor[0].family
                        : typed.editor[0].literal
                    : "UNKNOWN NAME"
                : "UNKNOWN EDITOR";

        const year = typed.issued
            ? typed.issued["date-parts"][0]
                ? typed.issued["date-parts"][0][0]
                : "UNKNOWN DATE"
            : "UNKNOWN ISSUANCE";

        return { author: author, editor: editor, year: year, id: typed.id };
    });

    onMount(() => {
        keys.forEach((k) => {
            const b = bib.find((b) => b.id === k);
            let cit = b as unknown as ICitation;
            cit.visible = isVisible;
            dispatch("citation", cit);
        });
    });

    const showCitation = (id: string) => {
        if (isMobile()) {
            for (let cit of citations) {
                if (cit.id === id) cit.visible = !cit.visible;
            }
            citations = citations;
        } else {
            dispatchToggle("showcitation", id);
        }
    };

    const highlightCitation = (id: string) => {
        dispatchHighlight("highlightcitation", id);
    };
</script>

<span class="font-medium">
    &nbsp;({#each short as cit}
        <span
            on:click={() => {
                showCitation(cit.id);
            }}
            on:keydown={() => {
                showCitation(cit.id);
            }}
            on:mouseenter={() => {
                highlightCitation(cit.id);
            }}
            on:mouseleave={() => {
                highlightCitation(cit.id);
            }}
            class="citation cursor-pointer hover:underline"
            >{cit
                ? `${cit.author !== "" ? cit.author : cit.editor}, ${cit.year}`
                : "MISSING CITATION"}</span>{/each})
    <span class="md:hidden">
        {#each citations as citation}
            <CitationItem {citation} />
        {/each}
    </span>
</span>

<style>
    .citation ~ .citation::before {
        content: ", ";
    }
</style>
