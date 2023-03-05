<script lang="ts">
    import { findLabel } from "../../utils/find";
    import type { IToCNode } from "../../utils/types";
    export let heading: IToCNode;
    export let max_depth: number;
    export let depth: number;

    const type = heading.label.split(":")[0];
    const value = heading.label.split(":")[1];
    console.log(type, value)
    const styles: { [index: string]: string } = {
        chap: "mb-12 text-2xl font-bold",
        sec: "my-4 text-xl font-normal ml-4",
        subsec: "my-1 text-base italic ml-4",
        subsubsec: "text-sm ml-4",
    };
    const style = styles[type];

    let path: string = "";
    const p = findLabel(heading.label)
    if(p)
        path = type == "chap" ? `/${p.value}`: `/${p.value}/${value}` 
    
</script>


<li class={`${style} flex justify-between`}>
    <div>
        {#if type == "chap" || type == "sec"}
            <a href={path} class="underline">{heading.value}</a>
        {:else}
            <div>{heading.value}</div>
        {/if}

        {#if heading.children  && depth < max_depth}
            <ol class="mt-2 mb-4">
                {#each heading.children as sec}
                    <svelte:self heading={sec} depth={depth+1} {max_depth}/>
                {/each}
            </ol>
        {/if}
    </div>
</li>

