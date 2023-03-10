<script lang="ts">
    import { findToCNodeByValue, getToC } from "../../../utils/find";
    import TableOfContents from "../TableOfContents.svelte";
    import { slide } from "svelte/transition";
    import { current_heading } from "../../../stores";

    let current_label = "";
    let isExpanded = false;
    const toggleToC = () => {
        isExpanded = !isExpanded;

        if (isExpanded && current_label !== "")
            setTimeout(() => {
                let el = document.getElementById(current_label);
                if (el) el.scrollIntoView({ behavior: 'smooth', block: 'center' });
            }, 600);
    };
    const toc = getToC();

    current_heading.subscribe((v) => {
        const l = findToCNodeByValue(v);

        if (l) {
            current_label = l.label;
            let el = document.getElementById(current_label);
            if (el) el.scrollIntoView({ behavior: 'smooth', block: 'center' });
        }
    });
</script>

<div
    class="bg-zinc-50 dark:bg-zinc-900 md:bg-transparent w-full md:w-3/12 fixed bottom-0 p-1 pb-2 md:p-2 border-t border-b-zinc-900 dark:border-b-zinc-300 md:border-none flex flex-col z-10"
>
    {#if isExpanded}
        <div
            in:slide={{duration: 400}}
            class={`font-serif text-sm max-h-96 p-1 mb-3 overflow-y-scroll bg-zinc-50 dark:bg-zinc-900 text-zinc-800 dark:text-zinc-50`}
        >
                <TableOfContents {toc} max_depth={4} {current_label}/>
        </div>
    {/if}
    <!-- toc -->
    <div
        on:click={toggleToC}
        on:keydown={toggleToC}
        class="flex items-center bg-zinc-50 dark:bg-zinc-900 text-zinc-800 dark:text-zinc-50 cursor-pointer"
    >
        <div>
            <img
                width="28"
                height="28"
                class={`hidden dark:inline relative m-1 p-1 cursor-pointer`}
                src={`/images/toc-line-dark.svg`}
                alt={`icon to download the pdf of the thesis`}
            />
            <img
                width="28"
                height="28"
                class={`inline dark:hidden relative m-1 p-1 cursor-pointer`}
                src={`/images/toc-line.svg`}
                alt={`icon to download the pdf of the thesis`}
            />
        </div>
        <div class="font-serif ml-3">{$current_heading}</div>
    </div>
</div>
