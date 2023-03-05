<script lang="ts">
    import type { PageData } from "./$types";
    import Date from "./components/Date.svelte";
    
    import Author from "./components/Author.svelte";
    import Title from "./components/Title.svelte";
    import Affiliation from "./components/Affiliation.svelte";
    import TableOfContents from "./components/TableOfContents.svelte";
    import Abstract from "./components/Abstract.svelte";

    export let data: PageData;
    const author = data.author;
    const title = data.title;
    const affiliation = data.affiliation;
    const date = data.date;
    const abstract = data.abstract;
    const toc = data.toc;
</script>

<div
    class="container max-w-full bg-zinc-50 text-zinc-800 dark:bg-zinc-900 dark:text-zinc-50 p-2 lg:p-8 font-serif text-xl"
>
    <div class="w-full lg:w-6/12 md:w-8/12 m-auto text-center pt-12">
        <Title {title} />
        <Author {author} />
        <Affiliation {affiliation} />
        <Date {date} />
    </div>

    <Abstract {abstract} />
    <TableOfContents {toc}/>
</div>
