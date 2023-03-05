<script lang="ts">
    import { getToC } from "../../../utils/find";
    import TableOfContents from "../TableOfContents.svelte";

    let isExpanded = false;
    const toggleToC = () => {
        isExpanded = !isExpanded;
    };
    const toc = getToC();
</script>

<div
    class="bg-zinc-50 dark:bg-zinc-900 md:bg-transparent w-full md:w-3/12 fixed bottom-0 p-1 pb-2 md:p-2 border-t border-b-zinc-900 dark:border-b-zinc-300 md:border-none flex flex-col z-10"
>
    <div class={`${isExpanded ? "" : "hidden"} font-serif text-sm max-h-96 p-1 mb-3 overflow-y-scroll bg-zinc-50 text-zinc-800 dark:bg-zinc-900 dark:text-zinc-50`}>
        <TableOfContents {toc} max_depth={4}/>
    </div>
    <!-- toc -->
    <div>
        <img
            width="28"
            height="28"
            on:click={toggleToC}
            on:keydown={toggleToC}
            class={`hidden dark:inline relative m-1 p-1 cursor-pointer`}
            src={`/images/toc-line-dark.svg`}
            alt={`icon to download the pdf of the thesis`}
        />
        <img
            width="28"
            height="28"
            on:click={toggleToC}
            on:keydown={toggleToC}
            class={`inline dark:hidden relative m-1 p-1 cursor-pointer`}
            src={`/images/toc-line.svg`}
            alt={`icon to download the pdf of the thesis`}
        />
    </div>
</div>
