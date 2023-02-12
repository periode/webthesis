<script lang="ts">
    import Error from "./Error.svelte";
    import { NodeType, type INode } from "../../utils/types";
    import { findNode } from "../../utils/find";

    export let tag: string;
    export let value: string;
    export let children: Array<INode> | null;

    //-- has a content
    let lang: string,
        caption: string,
        label: string = "";
    if (children) {
        let n = findNode(children, "code");
        if (n) lang = n.value;

        n = findNode(children, "caption");
        if (n) caption = n.children ? n.children.map(c => c.value).join(" ") : "";

        n = findNode(children, "label");
        if (n) label = n.children ? n.children[0].value : "";
    }
</script>

<div class="w-5/12 my-5 text-sm">
    {#if children}
        {#each children as paragraph}
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
                    <Error tag={paragraph.tag} />
                {/if}
            {/if}
        {/each}
    {/if}
</div>
