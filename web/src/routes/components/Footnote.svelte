<script lang="ts">
    import { clickOutside } from "../../utils/actions";
    import type { INode } from "../../utils/types";
    import Node from "./Node.svelte";
    export let node: INode;

    const children = node.children
        ? node.children
        : [{ tag: node.tag, value: node.value, children: null } as INode];

    let isVisible = false;
</script>

<span>
    <span
        on:mouseenter={() => (isVisible = true)}
        on:click={() => (isVisible = true)}
        on:touchstart={() => (isVisible = true)}
        on:keydown={() => (isVisible = true)}
        class="cursor-pointer"
    >
        <img
            class="inline relative bottom-1 left-1 pointer-events-none"
            src={`/images/footnote.svg`}
            alt={`icon to reference a footnote`}
        />&nbsp;
    </span>
    <div
        class={`absolute left-10 z-10 w-full lg:w-5/12 md:w-8/12 p-4 font-normal text-sm text-left min-h-max min-w-min border border-zinc-500 bg-zinc-100 text-zinc-500 ${
            isVisible ? "" : "hidden"
        }`}
        use:clickOutside
        on:outclick={() => {
            isVisible = false;
        }}
    >
        {#each children as node}
            <Node {node} />
        {/each}
    </div>
</span>
