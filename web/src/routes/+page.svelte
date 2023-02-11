<script lang="ts">
    import json_data from "./data.json";
    import Paragraph from "./components/Paragraph.svelte";
    import type { INode } from "../utils/types";

    let data = json_data as Array<INode>;
    let nodes = data[0].children as Array<INode>;
    let document_children = nodes[0].children as Array<INode>;
</script>

<div class="w-full bg-slate-50 text-slate-800 p-3">
    <h1>webthesis</h1>
    {#each document_children as n}
        <Paragraph children={n.children} />
    {/each}
</div>
