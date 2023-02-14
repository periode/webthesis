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

<div
    class="flex flex-col md:w-5/12 my-5 border border-slate-600 p-2"
    id={encodeURIComponent(label)}
>
    <img src={`/images/${image?.value}`} alt={caption?.value} srcset="" />
    <div class="italic text-center px-4">{caption?.value}</div>
    <div class="text-right text-sm">{label}</div>
</div>
