<script lang="ts">
    import { clickOutside } from "../../utils/actions";
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
</script>

<div
    use:clickOutside
    on:outclick={() => {
        // citation.visible = false;
    }}
    id={citation.id}
    class={`${
        !citation.visible ? "hidden" : ""
    } w-11/12 relative top-1/2 md:top-0 md:w-full m-auto p-6 md:p-0  text-zinc-500 dark:text-zinc-300 border border-zinc-400 md:border-none bg-zinc-50 dark:bg-zinc-800 md:bg-transparent pointer-events-auto`}
>
    <div
        class="md:hidden absolute top-1 right-2 text-lg font-mono cursor-pointer"
        on:click={() => {
            citation.visible = false;
        }}
        on:keydown={() => {
            citation.visible = false;
        }}
    >
        x
    </div>
    <i>{citation.title}</i> by {people}, {date}.
</div>
