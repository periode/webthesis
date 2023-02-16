<script lang="ts">
    import { findNode } from "../../utils/find";
    import type { INode } from "../../utils/types";

    export let node: INode;
    const contents = node.children ? node.children : [];
    const i = findNode(contents, "image");
    const image = i && i.children ? findNode(i.children, "literal") : null;
    const c = findNode(contents, "caption");
    const caption = c && c.children ? c.children[0] : null;
    const l = findNode(contents, "label");
    const label = l && l.children ? l.children[0].value : "";
</script>

<div class="md:w-5/12 my-8 " id={encodeURIComponent(label)}>
    <div class="relative text-zinc-400 text-right text-sm">{label}</div>
    <div class="flex flex-col border border-zinc-600 p-2">
        <img src={`/images/${image?.value}`} alt={caption?.value} srcset="" />
        <div class="italic text-center px-4">{caption?.value}</div>
    </div>
</div>
