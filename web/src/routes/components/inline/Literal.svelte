<script lang="ts">
    import type { INode } from "../../../utils/types";
    export let node: INode;
    const value = node.value ? node.value : "Missing literal";
</script>

<span>
    {value}
</span>
