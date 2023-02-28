<script lang="ts">
    import { page } from "$app/stores";
    import Node from "../../components/Node.svelte";
    import { findChapterInInclude, findSection } from "../../../utils/find";
    import NotFound from "../../components/NotFound.svelte";

    const include = $page.params.chap;
    const chap_node = findChapterInInclude(include);
    const chap_value = chap_node.children
        ? chap_node.children[0].value
        : "MISSING CHAP";

    const section = $page.params.sec;
    let nodes = findSection(include, section);
</script>

<div
    class="container max-w-full bg-zinc-50 text-zinc-800 dark:bg-zinc-900 dark:text-zinc-50 p-2 lg:p-3 font-serif text-xl"
>
    {#if nodes.length > 0}
        <div
            class="w-full lg:w-6/12 md:w-8/12 m-auto text-base mt-24 mb-8 italic"
        >
            <a href={`/${include}`}>{chap_value}</a>
        </div>
        {#each nodes as node}
            <Node {node} />
        {/each}
    {:else}
        <NotFound status="404" />
    {/if}
</div>
