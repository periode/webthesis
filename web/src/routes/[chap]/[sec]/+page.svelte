<script lang="ts">
    import type { PageData } from "./$types";
    import { page } from "$app/stores";
    import Section from "../../components/Section.svelte";
    import NotFound from "../../components/NotFound.svelte";
    import Navigation from "../../components/interface/Navigation.svelte";

    export let data: PageData;

    $: ({ chap, sec } = $page.params);
    $: ({ nodes, chapter } = data);
</script>

<div
    class="container max-w-full bg-zinc-50 text-zinc-800 dark:bg-zinc-900 dark:text-zinc-50 p-2 lg:p-3 font-serif md:text-lg"
>
    {#if nodes.length > 0}
        <div
            class="w-full lg:w-5/12 md:w-6/12 m-auto text-base mt-24 mb-8 italic"
        >
            <a href={`/${chap}`}>{chapter}</a>
        </div>
        <Section {nodes} name={sec}/>
        {#key sec}
            <Navigation chapter={`chap:${chap}`} section={`sec:${sec}`} />
        {/key}
    {:else}
        <NotFound status="404" />
    {/if}
</div>
