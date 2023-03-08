<script lang="ts">
    import type { INode } from "../../utils/types";

    export let node: INode = {tag: "spacer", children: null, value: `- - -`};
    if(node.value.length === 0) node.value = `- - -`
</script>

<div class="w-full lg:w-5/12 md:w-6/12 m-auto flex flex-col text-center justify-center h-24">
    <div>{node.value}</div>
</div>