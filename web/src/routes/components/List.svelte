<script lang="ts">
    import { findNodeByTag } from "../../utils/find";
    import type { INode } from "../../utils/types";
    export let node: INode;
    
    const items = node.children ? node.children.map((n) => {
        const i = findNodeByTag(n.children as INode[], "literal");
        const v = i ? i.value : "n/a";
        return v;
    }) : []
</script>

<ul class="list-disc list-inside ml-3">
    {#each items as i}
        <li>{i}</li>
    {/each}
</ul>