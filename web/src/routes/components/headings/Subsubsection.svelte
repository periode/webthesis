<script lang="ts">
    import type { INode } from "../../../utils/types";
    export let node:INode;

    let value = node.children ? node.children[0].value : "none"
</script>

<div class="w-full lg:w-4/12 md:w-6/12 m-auto mt-6 font-bold">
    {value}
</div>