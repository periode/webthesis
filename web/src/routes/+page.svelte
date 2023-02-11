<script lang="ts">
    import json_data from "./data.json";
    import Paragraph from "./components/Paragraph.svelte";
    import type { INode } from "../utils/types";

    let data = json_data as Array<INode>;
    let nodes = data[0].children as Array<INode>;
    // let nodes = parsed_data.children;
</script>

<div class="content">
    <h1>webthesis</h1>
    {#each nodes as n}
        <Paragraph children={n.children} />
    {/each}
</div>

<style>
    .content {
        background-color: #eee;
        color: #222;
    }
</style>
