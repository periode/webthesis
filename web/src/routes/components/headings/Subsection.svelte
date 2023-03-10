<script lang="ts">
    import { inview } from "svelte-inview";
    import type { INode } from "../../../utils/types";
    export let node: INode;

    import { current_heading } from "../../../stores";
    import { findToCNodeByValue } from "../../../utils/find";

    let value = node.children ? node.children[0].value : "none";
    const label_node = findToCNodeByValue(value);
    const label = label_node ? label_node.label.split(":")[1] : "MISSING";

    const options = {};
</script>

<div
    use:inview={options}
    on:enter={(e) => {
        $current_heading = value;
    }}
    class="w-full lg:w-5/12 md:w-6/12 m-auto text-2xl mt-8 mb-2"
    id={label}
>
    {value}
</div>
