<script lang="ts">
    import { createEventDispatcher, onMount } from "svelte";
    import isMobile from "../../utils/helper";
    import type { IFootnote, INode } from "../../utils/types";
    export let node: INode;

    const dispatch = createEventDispatcher<{ footnote: IFootnote }>();
    const dispatchToggle = createEventDispatcher<{showfootnote: string}>();

    const children = node.children
        ? node.children
        : [{ tag: node.tag, value: node.value, children: null } as INode];

    let isVisible = !isMobile();

    onMount(() => {
        dispatch("footnote", {children: children, value: node.value, visible: isVisible} as IFootnote);
    })

    const showFootnote = () => {        
        dispatchToggle("showfootnote", `footnote-${node.value}`);
    };
</script>

<span>
    <span
        on:click={showFootnote}
        on:touchstart={showFootnote}
        on:keydown={showFootnote}
        class="cursor-pointer"
    >
        <img
            class="inline dark:hidden relative bottom-1 left-1 pointer-events-none"
            src={`/images/footnote.svg`}
            alt={`icon to reference a footnote`}
        />
        <img
            class="relative bottom-1 left-1 pointer-events-none hidden dark:inline"
            src={`/images/footnote-dark.svg`}
            alt={`icon to reference a footnote`}
        />&nbsp;
    </span>

</span>
