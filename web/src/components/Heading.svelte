<script lang="ts">
    import { findToCNodeByLabel } from "../utils/find";
    import { slide } from "svelte/transition";
    import type { IToCNode } from "../utils/types";
    import { fade } from "svelte/transition";

    export let heading: IToCNode;
    export let max_depth: number;
    export let depth: number;
    export let current_label: string = "";

    const type = heading.label.split(":")[0];
    $: value = heading.label.split(":")[1];
    const styles: { [index: string]: string } = {
        chap: "mb-12 text-2xl font-semibold",
        sec: "my-4 text-xl font-normal ml-4",
        subsec: "my-1 text-base italic ml-4",
        subsubsec: "text-sm ml-4",
    };
    const style = styles[type];
    let isExpanded = false;
    let path: string = "";

    $: if (type == "chap") path = `/${value}`;
    else if (type === "sec") {
        const p = findToCNodeByLabel(heading.label);
        if (p) path = `/${p.parent?.label.split(":")[1]}/${value}`;
    } else if (type == "subsec") {
        const sec = findToCNodeByLabel(heading.label);
        let chap;
        if (sec && sec.parent) chap = findToCNodeByLabel(sec.parent?.label);
        if (sec && chap)
            path = `/${chap.parent?.label.split(":")[1]}/${
                sec.parent?.label.split(":")[1]
            }#${value}`;
    }

    const toggleExpansion = () => {
        isExpanded = !isExpanded;
    };
</script>

<li class={`${style} flex justify-between]`} id={heading.label}>
    <div class="">
        {#if type == "chap" || type == "sec"}
            <div class="flex">
                <div class="mr-2">{heading.index.join(".")}</div>
                <a
                    href={path}
                    class={`underline ${
                        current_label == heading.label ? "font-bold" : ""
                    }`}>{heading.value}</a
                >
            </div>
        {:else}
            <div
                on:click={toggleExpansion}
                on:keydown={toggleExpansion}
                class={`flex flex-row ${
                    heading.children && depth >= max_depth
                        ? "cursor-pointer"
                        : ""
                }`}
            >
                <div class="w-6">
                    {#if heading.children && !isExpanded && depth >= max_depth}
                        <span transition:fade={{ duration: 50 }}>+</span>
                    {/if}
                </div>
                <div
                    class={`${
                        current_label == heading.label ? "font-bold" : "flex"
                    }`}
                >
                    <div class="mr-2">{heading.index.join(".")}</div>
                    {#if type === "subsec"}
                        <a href={path} class="hover:underline">
                            {heading.value}
                        </a>
                    {:else}
                        <span>{heading.value}</span>
                    {/if}
                </div>
            </div>
        {/if}

        {#if heading.children && (depth < max_depth || isExpanded)}
            <ol transition:slide class="mt-2 mb-4">
                {#each heading.children as sec}
                    <svelte:self
                        heading={sec}
                        depth={depth + 1}
                        {max_depth}
                        {current_label}
                    />
                {/each}
            </ol>
        {/if}
    </div>
</li>
