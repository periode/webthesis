<script lang="ts">
    import type { INode } from "../../utils/types";

    export let node: INode;

    const value = node.children ? node.children[0].value : "Missing label";
</script>

<div class="text-slate-400 mb-12">
    {value}
</div>
