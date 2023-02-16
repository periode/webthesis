<script lang="ts">
    import type { INode } from "../../../utils/types";
    export let node: INode;
    const value = node.value ? node.value : "Missing literal";

    const cleaned = value.replaceAll("\\{", "{").replaceAll("\\}", "}").replaceAll("\\$", "$").replaceAll("\\#", "#").replaceAll("\\@", "@").replaceAll("\\%", "%").replaceAll("\\_", "_")
</script>

<span>
    {cleaned}
</span>
