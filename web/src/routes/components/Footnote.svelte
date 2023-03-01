<script lang="ts">
    import { createEventDispatcher, onMount } from "svelte";
    import isMobile from "../../utils/helper";
    import type { IFootnote, INode } from "../../utils/types";
    export let node: INode;

    const dispatch = createEventDispatcher<{ footnote: IFootnote }>();
    const dispatchToggle = createEventDispatcher<{ showfootnote: string }>();
    const dispatchHighlight = createEventDispatcher<{
        highlightfootnote: string;
    }>();

    const children = node.children
        ? node.children
        : [{ tag: node.tag, value: node.value, children: null } as INode];

    let isVisible = !isMobile();

    onMount(() => {
        dispatch("footnote", {
            children: children,
            value: node.value,
            visible: isVisible,
            highlighted: false,
        } as IFootnote);
    });

    const showFootnote = () => {
        dispatchToggle("showfootnote", `footnote-${node.value}`);
    };

    const highlightFootnote = () => {
        dispatchHighlight("highlightfootnote", `footnote-${node.value}`);
    };
</script>

<span>
    <span
        on:click={showFootnote}
        on:keydown={showFootnote}
        on:mouseenter={highlightFootnote}
        on:mouseleave={highlightFootnote}
        class="cursor-pointer p-1"
    >
        <sup>{node.value}</sup>
    </span>
</span>
