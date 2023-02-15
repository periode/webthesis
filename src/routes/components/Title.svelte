<script lang="ts">
    import type { INode } from "../../utils/types";

    export let node: INode;

    const value = node.children ? node.children[0].value : "Missing title!";
</script>

<div class="text-3xl">
    {` ${value} `}
</div>