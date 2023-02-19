<script lang="ts">
    import Error from "./Error.svelte";
    import { findNodeByTag } from "../../utils/find";
    import Code from "./Code.svelte";
    import type { INode } from "../../utils/types";

    export let node: INode;

    let lang: string = "",
        path: string = "",
        caption: string = "",
        label: string = "";

    if (node.children) {
        let n = findNodeByTag(node.children, "code");
        if (n) lang = n.value;
        if (n)
            path = n.children ? n.children[n.children.length - 1].value : "n/a";

        n = findNodeByTag(node.children, "caption");
        if (n)
            caption = n.children
                ? n.children.map((c) => c.value).join(" ")
                : "";

        n = findNodeByTag(node.children, "label");
        if (n) label = n.children ? n.children[0].value : "";
    }
</script>

<div class="md:w-5/12 my-8 text-sm" id={`${encodeURIComponent(label)}`}>
    <div class="relative text-right text-sm text-zinc-400">{label}</div>
    <div
        class="border-zinc-600 border p-2 font-mono  break-words overflow-y-scroll"
    >
        <Code {lang} {path} />
    </div>
    <div class="italic text-center text-base px-4">
        {caption}
    </div>
</div>
