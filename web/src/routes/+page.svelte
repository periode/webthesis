<script lang="ts">
    import text_data from "./text.json";
    import Date from "./components/Date.svelte";
    import type { INode } from "../utils/types";
    import { findNodeByTag } from "../utils/find";
    import Author from "./components/Author.svelte";
    import Title from "./components/Title.svelte";
    import Affiliation from "./components/Affiliation.svelte";
    import TableOfContents from "./components/TableOfContents.svelte";
    import Abstract from "./components/Abstract.svelte";

    const data = text_data as Array<INode>;
    const author_node = findNodeByTag(data, "author") as INode;
    const title_node = findNodeByTag(data, "title") as INode;
    const affiliation_node = findNodeByTag(data, "affiliation") as INode;
    const date_node = findNodeByTag(data, "date") as INode;
    const abstract_node = findNodeByTag(data, "abstract") as INode;
</script>

<div
    class="container max-w-full bg-zinc-50 text-zinc-800 dark:bg-zinc-800 dark:text-zinc-50 p-1 lg:p-8 font-serif text-xl"
>
    <div class="w-full lg:w-6/12 md:w-8/12 m-auto text-center pt-12">
        <Title node={title_node} />
        <Author node={author_node} />
        <Affiliation node={affiliation_node} />
        <Date node={date_node} />
    </div>

    <Abstract node={abstract_node} />
    <TableOfContents />
</div>
