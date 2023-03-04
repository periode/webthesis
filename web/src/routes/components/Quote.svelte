<script lang="ts">
    import { NodeType, type ICitation, type INode } from "../../utils/types";
    import Node from "./Node.svelte";
    import Citation from "./inline/Citation.svelte";
    import { createEventDispatcher } from "svelte";
    export let node: INode;

    const dispatch = createEventDispatcher<{ citation: ICitation }>();
    const dispatchToggle = createEventDispatcher<{ showcitation: string }>();
    const dispatchHighlight = createEventDispatcher<{
        highlightcitation: string;
    }>();

    const paragraph = node.children
        ? node.children[0]
        : ({ tag: node.tag, value: node.value, children: null } as INode);

    let citation: INode;
    let nodes = paragraph.children
        ? paragraph.children.filter((c) => {
              if (c.tag === NodeType.Citation) citation = c;
              else return c;
          })
        : [];

    const handleCitation = (event: CustomEvent<ICitation>) => {
        dispatch("citation", event.detail);
    };
    const showCitation = (event: CustomEvent<string>) => {
        dispatchToggle("showcitation", event.detail);
    };

    const highlightCitation = (event: CustomEvent<string>) => {
        dispatchHighlight("highlightcitation", event.detail);
    };
</script>

<div class="w-10/12 md:w-7/12 m-auto my-12 md:my-20 italic text-zinc-500 dark:text-zinc-300">
    <blockquote>
        {#each nodes as node}
            <Node {node} />
        {/each}

        {#if citation}
            <Citation
                node={citation}
                on:citation={handleCitation}
                on:showcitation={showCitation}
                on:highlightcitation={highlightCitation}
            />
        {/if}
    </blockquote>
</div>
