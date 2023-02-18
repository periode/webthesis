<script lang="ts">
    import json_data from "./data.json";
    import Date from "./components/Date.svelte";
    import DarkMode from "./components/DarkMode.svelte";
    import type { INode } from "../utils/types";
    import { findNodeByTag } from "../utils/find";
    import Author from "./components/Author.svelte";
    import Title from "./components/Title.svelte";
    import Affiliation from "./components/Affiliation.svelte";
    import TableOfContents from "./components/TableOfContents.svelte";
    import Abstract from "./components/Abstract.svelte";

    let data = json_data as Array<INode>;
    let nodes = data[0].children as Array<INode>;
    let root_children = nodes[0].children as Array<INode>;
    let document_children = root_children[0].children as Array<INode>;

    const author_node = findNodeByTag(data, "author") as INode;
    const title_node = findNodeByTag(data, "title") as INode;
    const affiliation_node = findNodeByTag(data, "affiliation") as INode;
    const date_node = findNodeByTag(data, "date") as INode;
    const abstract_node = findNodeByTag(data, "abstract") as INode;    
</script>

<div
    class="container max-w-full bg-zinc-50 text-zinc-800 dark:bg-zinc-800 dark:text-zinc-50 p-1 lg:p-8 font-serif text-xl"
>
    <Title node={title_node}/>
    <Author node={author_node}/>
    <Affiliation node={affiliation_node}/>
    <Date node={date_node}/>
    <Abstract node={abstract_node}/>
    <TableOfContents/>
</div>
