<script lang="ts">
    import type { INode } from "../../../utils/types";
    export let node: INode;

    const raw = node.children ? node.children[0].value : "NONE";
    const cleaned = raw
        .replaceAll("\\{", "{")
        .replaceAll("\\}", "}")
        .replaceAll("\\$", "$")
        .replaceAll("\\#", "#")
        .replaceAll("\\@", "@")
        .replaceAll("\\%", "%")
        .replaceAll("\\_", "_");
    let url = new URL(cleaned);

    console.log(url);
</script>

&nbsp;<a href={`${url.href}`} target="_blank" rel="noopener noreferrer" class="underline cursor-pointer">{url.host}</a>&nbsp;
