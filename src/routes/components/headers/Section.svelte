<script lang="ts">
    import type { INode } from "../../../utils/types";
    export let node: INode;

    let value = node.children ? node.children[0].value : "none"
</script>

<div class="text-3xl mt-14 mb-4">
    {value}
</div>