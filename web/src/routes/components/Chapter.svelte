<script lang="ts">
    import type { INode } from "../../utils/types";
    export let children: Array<INode> | null;

    let value = children ? children[0].value : "none"
</script>

<div class="chapter">
    {value}
</div>

<style>
    .chapter{
        font-size: 2em;
    }
</style>