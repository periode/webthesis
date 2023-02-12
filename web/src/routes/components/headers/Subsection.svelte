<script lang="ts">
    import type { INode } from "../../../utils/types";
    export let children: Array<INode> | null;

    let value = children ? children[0].value : "none"
</script>

<div class="text-2xl mt-8 mb-2">
    {value}
</div>