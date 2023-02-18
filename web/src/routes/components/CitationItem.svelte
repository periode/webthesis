<script lang="ts">
    import { clickOutside } from "../../utils/actions";
    import type { ICitation } from "../../utils/types";

    export let citation: ICitation;

    let date = citation.issued["date-parts"][0][0]
    let people = "UNKNOWN AUTHOR";
    if (citation.author)
        people = citation.author
            .map((a) => {
                return a.literal ? a.literal : `${a.given} ${a.family}`;
            })
            .join(", ")
        else if(citation.editor)
        people = citation.editor
            .map((a) => {
                return a.literal ? a.literal : `${a.given} ${a.family}`;
            })
            .join(", ")
</script>

<div
    use:clickOutside
    on:outclick={() => {
        // citation.visible = false;
    }}
    id={citation.id}
    class={`${
        !citation.visible ? "hidden" : ""
    } mb-2 text-zinc-500 dark:text-zinc-400`}
>
    <i>{citation.title}</i> by {people}, {date}.
</div>
