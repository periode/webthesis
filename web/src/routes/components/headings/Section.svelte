<script lang="ts">
    import { findToCNodeByValue } from "../../../utils/find";
    import type { INode } from "../../../utils/types";
    export let node: INode;

    let value = node.children ? node.children[0].value : "none"
    const label_node = findToCNodeByValue(value);
    const label = label_node ? label_node.label.split(":")[1] : "MISSING";
</script>

<div class="w-full lg:w-6/12 md:w-8/12 m-auto text-3xl mt-14 mb-4" id={label}>
    {value}
</div>