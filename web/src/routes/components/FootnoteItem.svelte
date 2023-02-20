<script lang="ts">
    import { clickOutside } from "../../utils/actions";
    import type { INode } from "../../utils/types";
    import Node from "./Node.svelte";

    export let footnote: INode;
    const children = footnote.children
        ? footnote.children
        : [{ tag: footnote.tag, value: footnote.value, children: null } as INode];
</script>

<div
id={`footnote-${footnote.value}`}
class={`w-11/12 relative top-1/2 md:top-0 md:w-10/12 m-auto p-6 md:p-0 md:py-2  text-zinc-500 dark:text-zinc-300 border border-zinc-400 md:border-none bg-zinc-50 dark:bg-zinc-800 md:bg-transparent pointer-events-auto`}
use:clickOutside
on:outclick={() => {
    // isVisible = false;
}}
>
{#each children as node}
    <Node {node} />
{/each}
</div>