<script lang="ts">
    import type { ICitation } from "../../utils/types";

    export let citation: ICitation;

    let date = citation.issued["date-parts"][0][0];
    let people = "UNKNOWN AUTHOR";
    if (citation.author)
        people = citation.author
            .map((a) => {
                return a.literal ? a.literal : `${a.family}, ${a.given}`;
            })
            .join(", ");
    else if (citation.editor)
        people = citation.editor
            .map((a) => {
                return a.literal ? a.literal : `${a.family} ${a.given}`;
            })
            .join(", ");

    let url: string = "";
    if (citation.publisher && citation.publisher.startsWith("http"))
        url = citation.publisher;
    else if (citation.URL) url = citation.URL;

    let inline = `${people}, <i>${citation.title}</i>, ${date}`;
</script>

<div
    id={citation.id}
    class="m-auto my-4  md:py-0 text-zinc-800 dark:text-zinc-50 bg-zinc-50 dark:bg-zinc-900 md:bg-transparent pointer-events-auto "
>
    {@html inline}{#if url !== ""}
        <span class="pl-2">
            <a href={url} target="_blank" class="underline">link</a>
        </span>
    {/if}
</div>
