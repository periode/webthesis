<script lang="ts">
    import { findNodeByTag } from "../../../utils/find";
    import Code from "./Code.svelte";
    import type { INode } from "../../../utils/types";

    export let node: INode;

    let lang: string = "",
        path: string = "",
        caption: string = "",
        label: string = "";

    if (node.children) {
        let n = findNodeByTag("code", node.children);
        if (n) lang = n.value;
        if (n)
            path = n.children ? n.children[n.children.length - 1].value : "n/a";

        n = findNodeByTag("caption", node.children);
        if (n)
            caption = n.children
                ? n.children.map((c) => c.value).join(" ")
                : "";

        n = findNodeByTag("label", node.children);
        if (n) label = n.children ? n.children[0].value.split(":")[1] : "";
    }
</script>

<div class="md:w-5/12 m-auto my-8 text-sm" id={`${encodeURIComponent(label)}`}>
    <div class="relative text-right text-sm text-zinc-400">{label}</div>
    <div
        class="border-zinc-600 border p-2 font-mono break-words overflow-y-hidden"
    >
        <Code {lang} {path} />
    </div>
    <div class="max-w-max my-4 m-auto italic text-sm text-left text-zinc-500 dark:text-zinc-300">
        {caption}
    </div>
</div>
