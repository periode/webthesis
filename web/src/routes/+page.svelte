<script lang="ts">
    import type { PageData } from "./$types";
    import Date from "./components/front/Date.svelte";
    import Author from "./components/front/Author.svelte";
    import Title from "./components/front/Title.svelte";
    import Affiliation from "./components/front/Affiliation.svelte";
    import TableOfContents from "./components/TableOfContents.svelte";
    import Abstract from "./components/front/Abstract.svelte";
    import { current_heading } from "../stores";
    import { onMount } from "svelte";

    onMount(() => {
        $current_heading = ''
    })

    export let data: PageData;
    const author = data.author;
    const title = data.title;
    const affiliation = data.affiliation;
    const date = data.date;
    const abstract = data.abstract;
    const toc = data.toc;
</script>

<div
    class="container max-w-full bg-zinc-50 text-zinc-800 dark:bg-zinc-900 dark:text-zinc-50 p-2 lg:p-8 font-serif md:text-xl"
>
    <div class="w-full lg:w-5/12 md:w-6/12 m-auto text-center pt-12">
        <Title {title} />
        <Author {author} />
        <Affiliation {affiliation} />
        <Date {date} />
    </div>

    <Abstract {abstract} />

    <div class="w-full lg:w-5/12 md:w-6/12 m-auto pt-12">
        <h2 class="my-4 text-2xl">Table of contents</h2>
        <TableOfContents {toc} max_depth={2} />
    </div>
</div>
