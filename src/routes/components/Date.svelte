<script lang="ts">
    import type { INode } from "../../utils/types";

    export let node: INode;

    const timestamp = node.value;
    let d = new Date(parseInt(timestamp) * 1000);
</script>

<div class="text-sm">
    last updated on: {`${d.getDate()}-${
        d.getMonth() + 1 < 10 ? "0" + (d.getMonth() + 1) : d.getMonth() + 1
    }-${d.getFullYear()}`} at {`${
        d.getHours() < 10 ? "0" + d.getHours() : d.getHours()
    }:${d.getMinutes()}`}
</div>
