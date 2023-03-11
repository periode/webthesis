<script lang="ts">
    import type { IFootnote, INode } from "../../../utils/types";
    import Node from "./../Node.svelte";

    export let footnote: IFootnote;
    const children = footnote.children as INode[];   
</script>

<aside
    id={`footnote-${footnote.value}`}
    class={`md:${
        footnote.visible ? "block" : "hidden"
    } w-11/12 relative top-1/2 md:top-0 md:w-10/12 m-auto p-4 italic md:p-0 md:py-2  text-zinc-500 dark:text-zinc-300 border-r bg-zinc-50 dark:bg-zinc-900 md:bg-transparent pointer-events-auto ${
        footnote.highlighted
            ? "border-r-zinc-300 dark:border-r-zinc-300"
            : "border-r-zinc-50 dark:border-r-zinc-900"
    }`}
>
    <sup>{footnote.value}</sup>
    {#each children as node}
        <Node {node} />
    {/each}
</aside>
