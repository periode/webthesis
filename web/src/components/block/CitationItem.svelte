<script lang="ts">
    import type { ICitation } from "../../utils/types";

    export let citation: ICitation;

    let date = citation.issued["date-parts"][0][0];
    let people = "UNKNOWN AUTHOR";
    if (citation.author)
        people = citation.author
            .map((a) => {
                return a.literal ? a.literal : `${a.given} ${a.family}`;
            })
            .join(", ");
    else if (citation.editor)
        people = citation.editor
            .map((a) => {
                return a.literal ? a.literal : `${a.given} ${a.family}`;
            })
            .join(", ");

    let url: string = "";
    if (citation.publisher && citation.publisher.startsWith("http"))
        url = citation.publisher;
    else if (citation.URL) url = citation.URL;
</script>

<div
    id={citation.id}
    class={`${
        citation.visible ? "block" : "hidden"
    }  w-11/12 relative top-1/2 md:top-0 md:w-10/12 m-auto py-12 px-2  md:py-0  text-zinc-500 dark:text-zinc-300 bg-zinc-50 dark:bg-zinc-900 md:bg-transparent pointer-events-auto md:border-l  ${
        citation.highlighted
            ? "border-l-zinc-900 dark:border-l-zinc-300"
            : "border-l-zinc-50 dark:border-l-zinc-900"
    }`}
>
    {citation.title} by {people}, {date}.{#if url !== ""}&nbsp;[<a
            href={url}
            target="_blank"
            class="underline">link</a
        >]
    {/if}
</div>
