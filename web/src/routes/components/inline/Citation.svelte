<script lang="ts">
    import bib from "../../bib.json";
    import { clickOutside } from "../../../utils/actions";
    import type { ICitation, INode } from "../../../utils/types";
    import { createEventDispatcher, onMount } from "svelte";
    import isMobile from "../../../utils/helper";

    const dispatch = createEventDispatcher<{ citation: ICitation }>();
    const dispatchToggle = createEventDispatcher<{ showcitation: string }>();
    const dispatchHighlight = createEventDispatcher<{
        highlightcitation: string;
    }>();

    export let node: INode;
    const value = node.children ? node.children[0].value : "Missing citation";
    const keys = value.split(",");

    const short = keys.map((k) => {
        const b = bib.find((b) => b.id === k);
        const typed = b ? (b as unknown as ICitation) : undefined;

        if (typed === undefined)
            return { author: "author", year: "year", id: "id" };

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

    let isVisible = !isMobile();

    onMount(() => {
        keys.forEach((k) => {
            const b = bib.find((b) => b.id === k);
            const typed = b ? (b as unknown as ICitation) : undefined;
            if (typed) typed.visible = isVisible;
            dispatch("citation", typed);
        });
    });

    const showCitation = (id: string) => {
        dispatchToggle("showcitation", id);
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
                : "nope"}</span
        >{/each})&nbsp;
</span>

<style>
    .citation ~ .citation::before {
        content: ", ";
    }
</style>
