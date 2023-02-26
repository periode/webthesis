<script lang="ts">
    import { children } from "svelte/internal";
import type { IToCNode } from "../../utils/types";
    import Spacer from "./Spacer.svelte";

    export let heading: IToCNode;

    const path = heading.value.split('.')[0];
    const chapter = heading.children ? heading.children[0] as IToCNode : null;
    // todo get the url
    const sections = chapter? chapter.children as Array<IToCNode> : [];
</script>

<li class="text-2xl my-12">
    <a href={`./${path}.html`} class="font-bold hover:underline">{chapter?.value}</a>
    <ol class="mt-4">
        {#each sections as sec}
            <li class="text-xl list-decimal ml-12 mb-2">{sec.value}</li>
            <ul class="mb-4">
                {#if sec.children}
                    {#each sec.children as subsec}
                        <li class="text-base italic ml-16 md:ml-24">{subsec.value}</li>
                    {/each}
                {/if}
            </ul>
        {/each}
    </ol>
</li>
