<script lang="ts">
    import { findNodeByTag } from "../../utils/find";
    import type { INode } from "../../utils/types";

    export let node: INode;
    const contents = node.children ? node.children : [];
    const i = findNodeByTag("image", contents);
    const image = i && i.children ? findNodeByTag("literal", i.children) : null;
    const c = findNodeByTag("caption", contents);
    const caption = c && c.children ? c.children[0] : null;
    const l = findNodeByTag("label", contents);
    const label = l && l.children ? l.children[0].value : "";
</script>

<div class="w-full lg:w-6/12 m-auto  md:w-5/12 my-8 " id={encodeURIComponent(label)}>
    <div class="relative text-zinc-400 text-right text-sm">{label}</div>
    <div class="flex flex-col border border-zinc-600 p-2">
        <img src={`/images/${image?.value}`} alt={caption?.value} srcset="" />
        <div class="italic text-center px-4">{caption?.value}</div>
    </div>
</div>
