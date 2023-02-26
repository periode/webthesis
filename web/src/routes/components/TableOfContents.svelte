<script lang="ts">
    import type { IToCNode } from "../../utils/types";
    import Heading from "./Heading.svelte";
    import toc_data from "../toc.json";
    const toc = toc_data as Array<IToCNode>;
    const includes = toc[0].children as Array<IToCNode>;    
</script>

<div class="w-full lg:w-6/12 md:w-8/12 m-auto my-4 py-2">
    <h3 class="my-4 text-2xl">Table of contents</h3>
    <ol class="list-decimal list-inside">
        {#each includes as heading}
            <Heading {heading}/>
        {/each}
    </ol>
</div>

