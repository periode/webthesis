<script lang="ts">
    import { findToCNodeByLabel } from "../../utils/find";
    import { slide } from "svelte/transition";
    import type { IToCNode } from "../../utils/types";
    import { fade } from "svelte/transition";

    export let heading: IToCNode;
    export let max_depth: number;
    export let depth: number;
    export let current_label: string = "";

    const type = heading.label.split(":")[0];
    const value = heading.label.split(":")[1];
    const styles: { [index: string]: string } = {
        chap: "mb-12 text-2xl font-semibold",
        sec: "my-4 text-xl font-normal ml-4",
        subsec: "my-1 text-base italic ml-4",
        subsubsec: "text-sm ml-4",
    };
    const style = styles[type];
    let isExpanded = false;
    let path: string = "";

    if (type == "chap") path = `/${value}`;
    else if (type === "sec") {
        const p = findToCNodeByLabel(heading.label);
        if (p) path = `/${p.parent?.label.split(":")[1]}/${value}`;
    }
    
    const toggleExpansion = () => {
        isExpanded = !isExpanded;
    };
</script>

<li class={`${style} flex justify-between`} id={heading.label}>
    <div>
        {#if type == "chap" || type == "sec"}
            <a href={path} class={`underline ${current_label == heading.label ? "font-bold" : ""}`}>{heading.value}</a>
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
                <div class={`w-6`}>
                    {#if heading.children && !isExpanded && depth >= max_depth}
                        <span transition:fade={{ duration: 50 }}>-</span>
                    {/if}
                </div>
                <div class={`${current_label == heading.label ? "font-bold" : ""}`}>
                    {heading.value}
                </div>
            </div>
        {/if}

        {#if heading.children && (depth < max_depth || isExpanded)}
            <ol transition:slide class="mt-2 mb-4">
                {#each heading.children as sec}
                    <svelte:self heading={sec} depth={depth + 1} {max_depth} {current_label}/>
                {/each}
            </ol>
        {/if}
    </div>
</li>
