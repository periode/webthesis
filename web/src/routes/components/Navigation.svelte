<script lang="ts">
    import { findNextToC, findPreviousToC } from "../../utils/find";

    export let chapter: string;
    export let section: string = "";

    const prev = findPreviousToC(chapter, section);
    const prev_path = prev ? prev.label.split(":")[1] : "MISSING";
    const next = findNextToC(chapter, section);
    const next_path = next ? next.label.split(":")[1] : "MISSING";
</script>

<div class="w-11/12 lg:w-4/12 md:w-6/12 m-auto my-6 md:my-12 flex justify-between">
    <div class="w-1/3 flex justify-start">
        {#if prev}
            <a
                data-sveltekit-reload
                href={section === ""
                    ? `/${prev_path}`
                    : `/${chapter.split(":")[1]}/${prev_path}`}
                class="flex items-center"
            >
                <div>
                    <img
                        width="24"
                        height="24"
                        class="inline dark:hidden relative bottom-1 left-1 pointer-events-none"
                        src={`/images/arrow-left.svg`}
                        alt={`icon to reference a footnote`}
                    />
                    <img
                        width="24"
                        height="24"
                        class="relative bottom-1 left-1 pointer-events-none hidden dark:inline"
                        src={`/images/arrow-left-dark.svg`}
                        alt={`icon to reference a footnote`}
                    />
                </div>
                <div class="hidden md:visible text-sm italic ml-2 h-6">
                    {prev.value}
                </div>
            </a>
        {/if}
    </div>

    <div class="w-1/3 flex justify-center">
        <a
            data-sveltekit-reload
            href={section === "" ? "/" : `/${chapter.split(":")[1]}`}
        >
            <img
                width="24"
                height="24"
                class="inline dark:hidden relative bottom-1 left-1 pointer-events-none"
                src={`/images/arrow-up.svg`}
                alt={`icon to reference a footnote`}
            />
            <img
                width="24"
                height="24"
                class="relative bottom-1 left-1 pointer-events-none hidden dark:inline"
                src={`/images/arrow-up-dark.svg`}
                alt={`icon to reference a footnote`}
            />
        </a>
    </div>

    <div class="w-1/3 flex justify-end">
        {#if next}
            <a
                data-sveltekit-reload
                href={section === ""
                    ? `/${next_path}`
                    : `/${chapter.split(":")[1]}/${next_path}`}
                    class="flex items-center"
            >
                <div class="hidden md:visible text-sm italic ml-2 h-6">
                    {next.value}
                </div>
                <div>
                    <img
                        width="24"
                        height="24"
                        class="inline dark:hidden relative bottom-1 left-1 pointer-events-none"
                        src={`/images/arrow-right.svg`}
                        alt={`icon to reference a footnote`}
                    />
                    <img
                        width="24"
                        height="24"
                        class="relative bottom-1 left-1 pointer-events-none hidden dark:inline"
                        src={`/images/arrow-right-dark.svg`}
                        alt={`icon to reference a footnote`}
                    />
                </div>
            </a>
        {/if}
    </div>
</div>
