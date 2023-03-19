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
    const label = l && l.children ? l.children[0].value.split(":")[1] : "";
</script>

<div class="w-full m-auto my-8 text-sm" id={encodeURIComponent(label)}>
    <div class="relative text-right text-zinc-400">{label}</div>
    <figure class="flex flex-col">
        <img
            class="max-h-fit border-zinc-600 border p-2"
            src={`/images/figures/${image?.value}`}
            alt={caption?.value}
            srcset=""
        />
        <figcaption
            class="max-w-max my-4 m-auto italic text-sm text-left text-zinc-500 dark:text-zinc-300"
        >
            {caption?.value}
        </figcaption>
    </figure>
</div>
