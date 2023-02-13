<script lang="ts">
    import type { INode } from "../../../utils/types";
    export let node:INode;

    let value = node.children ? node.children[0].value : "none"
</script>

<div class="font-bold">
    {value}
</div>