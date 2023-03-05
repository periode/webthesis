<script lang="ts">
    import type { IToCNode } from "../../utils/types";
    import Heading from "./Heading.svelte";
    export let toc: Array<IToCNode>;
    export let max_depth: number = 3;
    let depth = 0;
</script>

<div class="w-full lg:w-6/12 md:w-8/12 m-auto my-4 py-2">
    <h3 class="my-4 text-2xl">Table of contents</h3>
    <ol class="list-decimal list-inside">
        {#each toc as heading}
            <Heading {heading} {depth} {max_depth}/>
        {/each}
    </ol>
</div>
