<script lang="ts">
    import Error from "./Error.svelte";
    import { NodeType, type INode } from "../../utils/types";
    import { findNode } from "../../utils/find";

    export let node: INode;

    let lang: string,
        caption: string,
        label: string = "";
    if (node.children) {
        let n = findNode(node.children, "code");
        if (n) lang = n.value;

        n = findNode(node.children, "caption");
        if (n)
            caption = n.children
                ? n.children.map((c) => c.value).join(" ")
                : "";

        n = findNode(node.children, "label");
        if (n) label = n.children ? n.children[0].value : "";
    }
</script>

<div class="md:w-5/12 my-5 text-sm" id={encodeURIComponent(label)}>
    {#if node.children}
        {#each node.children as paragraph}
            {#if paragraph.children}
                {#if paragraph.children[0].tag === NodeType.Code}
                    <div
                        class="border-slate-600 border-2 p-2 font-mono bg-slate-200 break-words overflow-y-scroll"
                    >
                        {#if paragraph.children[0].children}
                            {#each paragraph.children[0].children as c}
                                <div>{c.value}</div>
                            {/each}
                        {/if}
                    </div>
                {:else if paragraph.children[0].tag === NodeType.Caption}
                    <div class="italic text-center px-4">
                        {caption}
                    </div>
                {:else if paragraph.children[0].tag === NodeType.Label}
                    <div class="text-right text-sm">{label}</div>
                {:else}
                    <Error node={paragraph} />
                {/if}
            {/if}
        {/each}
    {/if}
</div>
