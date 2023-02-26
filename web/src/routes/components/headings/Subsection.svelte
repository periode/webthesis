<script lang="ts">
    import type { INode } from "../../../utils/types";
    export let node:INode;

    let value = node.children ? node.children[0].value : "none"
</script>

<div class="w-full lg:w-6/12 md:w-8/12 m-auto text-2xl mt-8 mb-2">
    {value}
</div>