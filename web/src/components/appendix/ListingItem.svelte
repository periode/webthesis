<script lang="ts">
    import type { IListingsNode } from "../../utils/types";


    export let item: IListingsNode;
</script>

<li class="flex w-full my-2 ">
    <div class="flex flex-row w-full justify-between">
        <div class="w-1/2 flex flex-row">
            <div class="mr-3">{item.index}.</div>
            <div class="flex flex-col">
                <span>{item.label.split(":")[1]}</span>
                <span class="text-sm">{item.value}</span>
            </div>
        </div>

        <div class="flex flex-row">
            <div class="flex flex-col justify-start text-right">
                <span>{item.chapter}</span>
                <span class="text-sm">{item.section}</span>
            </div>
            <div class="ml-3">
                <a href={`/${item.chapter_label}/${item.section_label}#${item.label.split(':')[1]}`} class="hover:font-bold">→</a>
            </div>
        </div>

    </div>
</li>