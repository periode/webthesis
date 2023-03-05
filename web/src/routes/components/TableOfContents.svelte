<script lang="ts">
    import type { IToCNode } from "../../utils/types";
    import Heading from "./Heading.svelte";
    export let toc: Array<IToCNode>;
    export let max_depth: number = 3;
    let depth = 0;
</script>

<div class="w-full h-full m-auto py-2">
    <ol class="list-decimal list-inside">
        {#each toc as heading}
            <Heading {heading} {depth} {max_depth}/>
        {/each}
    </ol>
</div>
