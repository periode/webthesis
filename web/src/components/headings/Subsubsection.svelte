<script lang="ts">
    import { inview } from "svelte-inview";
    import type { INode } from "../../utils/types";
    export let node: INode;

    import { current_heading } from "../../stores";

    let value = node.children ? node.children[0].value : "none";

    const options = {};
</script>

<h4
    use:inview={options}
    on:enter={(e) => {
        $current_heading = value;
    }}
    class="w-full lg:w-5/12 md:w-6/12 m-auto mt-6 font-semibold"
>
    {value}
</h4>

