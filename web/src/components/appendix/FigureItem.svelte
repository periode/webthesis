<script lang="ts">
    import { fade, slide } from "svelte/transition";
    import type { IListingsNode, INode } from "../../utils/types";
    import Figure from "../block/Figure.svelte";

    export let figure: IListingsNode;
    export let node: INode;
    let isExpanded = false;

    const toggleExpansion = () => {
        isExpanded = !isExpanded;
    };
</script>

<li class="flex w-full my-4">
    <div class="flex flex-row w-full justify-between">
        <div class="w-full flex flex-row justify-between">
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
                        <div transition:slide class="flex flex-row">
                            <div class="flex flex-col">
                                <Figure {node} />
                                <span class="text-sm">{figure.value}</span>
                            </div>
                        </div>
                    {/if}
                </div>
            </div>

            {#if isExpanded}
                <div transition:fade={{duration: 100}} class="flex justify-start ml-4">
                    <a
                        href={`/${figure.chapter_label}/${
                            figure.section_label
                        }#${figure.label.split(":")[1]}`}
                        class="hover:underline text-right flex flex-col"
                    >
                        <div>
                            {figure.chapter}
                        </div>
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
