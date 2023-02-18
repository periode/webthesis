<script lang="ts">
    import bib from "../../bib.json";
    import { clickOutside } from "../../../utils/actions";

    import type { ICitation, INode } from "../../../utils/types";
    export let node: INode;
    const value = node.children ? node.children[0].value : "Missing citation";
    const keys = value.split(",");
    const citations = keys.map((k) => {
        const b = bib.find((b) => b.id === k);
        const typed = b ? (b as unknown as ICitation) : undefined;

        if (typed === undefined) {
            console.warn("undefined", k);
        } else {
            const author = typed.author
                ? typed.author[0]
                    ? typed.author[0].family
                        ? typed.author[0].family
                        : typed.author[0].literal
                    : "UNKNOWN NAME"
                : "UNKNOWN AUTHOR";
            const year = typed.issued
                ? typed.issued["date-parts"][0]
                    ? typed.issued["date-parts"][0][0]
                    : "UNKNOWN DATE"
                : "UNKNOWN ISSUANCE";

                const title = typed.title
                ? typed.title
                : "UNKNOWN TITLE";

                const publisher = typed.publisher
                ? typed.publisher
                : "UNKNOWN PUB";

            const short = `${author}, ${year}`;
            const long = `${title}, written by ${author} at ${publisher}`;
            return [short, long];
        }
    });

    let isVisible = true;
</script>

<span class="font-medium">
    &nbsp;({#each citations as cit}
        <span
            on:mouseenter={() => (isVisible = true)}
            on:click={() => (isVisible = true)}
            on:touchstart={() => (isVisible = true)}
            on:keydown={() => (isVisible = true)}
            class="citation cursor-pointer hover:underline"
            >{cit ? cit[0] : "nope"}</span
        >{/each})&nbsp;
</span>

<div
    class={`absolute flex flex-col right-10 z-10 w-full lg:w-4/12 md:w-8/12 p-4 border-zinc-500 dark:border-zinc-400 border-l font-normal text-left min-h-max min-w-min ${
        isVisible ? "" : "hidden"
    }`}
    use:clickOutside
    on:outclick={() => {
        isVisible = false;
    }}
>
    {#each citations as cit}
        <div class="bg-zinc-100 dark:bg-zinc-900 " />
        <div class=" py-4 px-8 mx-4 text-sm text-zinc-500 dark:text-zinc-400">
            {cit ? cit[1] : "nope"}
        </div>
    {/each}
</div>

<style>
    .citation ~ .citation::before {
        content: ", ";
    }
</style>
