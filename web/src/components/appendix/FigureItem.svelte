<script lang="ts">
    import { fade, slide } from "svelte/transition";
    import { findNodesByTag, getBundle } from "../../utils/find";
    import type { IListingsNode, INode } from "../../utils/types";
    import Figure from "../block/Figure.svelte";

    export let figure: IListingsNode;
    let node: INode = { tag: "none", value: "", children: null };
    let ls = findNodesByTag("figure", getBundle(figure.chapter_label));

    if (ls) {
        let tmp = ls.find((el) => el.value == figure.label);
        if (tmp) node = tmp;
    }

    let isExpanded = false;

    const toggleExpansion = () => {
        isExpanded = !isExpanded;
    };
</script>

<li class="flex w-full my-4">
    <div class="flex flex-row w-full justify-between">
        <div class="w-full flex flex-col md:flex-row justify-between">
            <div class="flex">
                <div class="mr-3">{figure.index}.</div>

                <div class="flex flex-col">
                    <span
                        on:click={toggleExpansion}
                        on:keydown={toggleExpansion}
                        class="cursor-pointer underline"
                        >{figure.label.split(":")[1]}</span
                    >

                    {#if isExpanded}
                        <div transition:slide>
                            <div class="max-w-xs md:max-w-xl">
                                <Figure {node} />
                            </div>
                        </div>
                    {/if}
                </div>
            </div>

            {#if isExpanded}
                <div
                    transition:fade={{ duration: 100 }}
                    class=" flex justify-start ml-4 items-baseline"
                >
                    <a
                        href={`/${figure.chapter_label}/${
                            figure.section_label
                        }#${figure.label.split(":")[1]}`}
                        class="hover:underline text-right flex flex-col"
                    >
                        <div class="text-sm">
                            {figure.section}
                        </div>
                    </a>
                    <div class="ml-3 cursor-pointer hover:underline">→</div>
                </div>
            {/if}
        </div>
    </div>
</li>
