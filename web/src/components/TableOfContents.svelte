<script lang="ts">
    import type { IToCNode } from "../utils/types";
    import Heading from "./Heading.svelte";
    export let toc: Array<IToCNode>;
    export let max_depth: number = 3;
    export let current_label = "";
    let depth = 0;
</script>

<nav class="w-full h-full m-auto py-2">
    <ol class="list-decimal list-inside">
        {#each toc as heading}
            <Heading {heading} {depth} {max_depth} {current_label}/>
        {/each}
    </ol>
</nav>
