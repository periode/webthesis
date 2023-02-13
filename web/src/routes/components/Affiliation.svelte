<script lang="ts">
    import type { INode } from "../../utils/types";

    export let node: INode;

    const value = node.children ? node.children[0].value : "Missing affiliation";
</script>

<div class="italic">
    {` ${value} `}
</div>
