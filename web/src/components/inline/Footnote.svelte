<script lang="ts">
    import { createEventDispatcher, onMount } from "svelte";
    import isMobile from "../../utils/helper";
    import type { IFootnote, INode } from "../../utils/types";
    import FootnoteItem from "../block/FootnoteItem.svelte";
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
    let isMobileVisible = false;

    let footnote = {
        children: children,
        value: node.value,
        visible: isVisible,
        highlighted: false,
    } as IFootnote;

    onMount(() => {
        dispatch("footnote", footnote);
    });

    const showFootnote = () => {
        if(isMobile())
            isMobileVisible = !isMobileVisible;
        else
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
    <div class={isMobileVisible ? 'block' : 'hidden'}>
        <FootnoteItem {footnote} />
    </div>
    
</span>
