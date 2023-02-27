<script lang="ts">
    import { page } from "$app/stores";
    import Node from "../../components/Node.svelte";
    import { findSection } from "../../../utils/find";
    import Chapter from "../../components/headings/Chapter.svelte";

    const section = $page.params.sec;
    const include = $page.route.id as string;
    const include_val = include.split("/")[1];

    let nodes = findSection(include_val, section);
    

    // todo: find the proper chapter node, based on the route
</script>

<div class="container max-w-full bg-zinc-50 text-zinc-800 dark:bg-zinc-900 dark:text-zinc-50 p-2 lg:p-3 font-serif text-xl">
    <div class="w-full lg:w-6/12 md:w-8/12 m-auto text-4xl mt-24 mb-8">
        {include_val}
    </div>
    {#each nodes as node}
        <Node {node} />
    {/each}
</div>
