<script lang="ts">
    import type { INode } from "../../../utils/types";
    export let node: INode;
    let value = node.children ? node.children[0].value : "none"
</script>

<div class="w-full lg:w-5/12 md:w-6/12 m-auto text-4xl mt-24 mb-8">
    {value}
</div>