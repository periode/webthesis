<script lang="ts">
    import { children } from "svelte/internal";
    import type { IToCNode } from "../../utils/types";
    import Spacer from "./Spacer.svelte";

    export let heading: IToCNode;

    const path = heading.value.split(".")[0];
    const chapter = heading.children ? (heading.children[0] as IToCNode) : null;

    const sections = chapter ? (chapter.children as Array<IToCNode>) : [];
    let showPreview = false;
</script>

<li class="text-2xl my-12 flex justify-between">
    <div>
        <a
            on:mouseenter={() => {
                showPreview = true;
            }}
            href={`./${path}`}
            class="font-bold hover:underline">{chapter?.value}</a
        >
        <ol class="mt-4">
            {#each sections as sec}
                <li class="text-xl list-decimal ml-12 mb-2">
                    <a href={`./${path}/${sec.label.split(":")[1]}`}>{sec.value}</a>
                </li>
                <ul class="mb-4">
                    {#if sec.children}
                        {#each sec.children as subsec}
                            <li class="text-base italic ml-16 md:ml-24">
                                {subsec.value}
                            </li>
                        {/each}
                    {/if}
                </ul>
            {/each}
        </ol>
    </div>
    <div
        class={`${
            showPreview ? "" : "hidden"
        } w-5/12 relative border border-zinc-100`}
    >
    <div on:click={() => {showPreview = false;}} on:keypress={() => {showPreview = false;}} class="absolute font-mono cursor-pointer top-1 right-1 m-2 rounded-full bg-zinc-50"><img src="/images/close-circle-fill.svg" alt="" srcset=""></div>
        <iframe
            class="w-full h-full overflow-y-scroll"
            src={`./${path}`}
            frameborder="0"
            title={chapter?.value}
        />
    </div>
</li>
