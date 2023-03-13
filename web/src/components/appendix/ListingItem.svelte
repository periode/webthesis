<script lang="ts">
    import { fade, slide } from "svelte/transition";
    import { findNodesByTag, getBundle } from "../../utils/find";
    import type { IListingsNode, INode } from "../../utils/types";
    import Listing from "../block/Listing.svelte";

    export let listing: IListingsNode;
    let node: INode = { tag: "none", value: "", children: null };
    let ls = findNodesByTag("listing", getBundle(listing.chapter_label));

    if (ls) {
        let tmp = ls.find((el) => el.value == listing.label);
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
            <div class="flex w-full">
                <div class="mr-3">{listing.index}.</div>

                <div class="flex flex-col">
                    <span
                        on:click={toggleExpansion}
                        on:keydown={toggleExpansion}
                        class="cursor-pointer underline"
                        >{listing.label.split(":")[1]}</span
                    >

                    {#if isExpanded}
                        <div transition:slide>
                            <div class="max-w-xs md:max-w-xl">
                                <Listing {node} />
                            </div>
                        </div>
                    {/if}
                </div>
            </div>

            {#if isExpanded}
                <div
                    transition:fade={{ duration: 100 }}
                    class="flex justify-start ml-4 mb-8"
                >
                    <a
                        href={`/${listing.chapter_label}/${
                            listing.section_label
                        }#${listing.label.split(":")[1]}`}
                        class="underline md:no-underline hover:underline text-right flex flex-col"
                    >
                        <div class="text-sm">
                            {listing.section}
                        </div>
                    </a>
                    <div class="ml-3 cursor-pointer hover:underline">→</div>
                </div>
            {/if}
        </div>
    </div>
</li>
