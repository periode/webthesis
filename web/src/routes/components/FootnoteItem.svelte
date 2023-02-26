<script lang="ts">
    import { clickOutside } from "../../utils/actions";
    import type { IFootnote, INode } from "../../utils/types";
    import Node from "./Node.svelte";

    export let footnote: IFootnote;
    const children = footnote.children as INode[];
</script>

<div
    id={`footnote-${footnote.value}`}
    class={`${
        footnote.visible ? "block" : "hidden"
    } w-11/12 relative top-1/2 md:top-0 md:w-10/12 m-auto p-6 md:p-0 md:py-2  text-zinc-500 dark:text-zinc-300 border border-zinc-400 md:border-none bg-zinc-50 dark:bg-zinc-900 md:bg-transparent pointer-events-auto`}
    use:clickOutside
    on:outclick={() => {
        // isVisible = false;
    }}
>
    {#each children as node}
        <Node {node} />
    {/each}
    <div
        class="md:hidden absolute top-1 right-2 text-lg font-mono cursor-pointer"
        on:click={() => {
            footnote.visible = false;
        }}
        on:keydown={() => {
            footnote.visible = false;
        }}
    >
        x
    </div>
</div>
