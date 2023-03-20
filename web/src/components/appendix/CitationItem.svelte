<script lang="ts">
    import { slide } from "svelte/transition";
    import { getBibtexEntry, getParagraphReferences } from "../../utils/find";
    import type { ICitation } from "../../utils/types";

    export let citation: ICitation;

    const isValidHttpUrl = (_url: string): boolean => {
        let url;
        try {
            url = new URL(_url);
        } catch (_) {
            return false;
        }
        return url.protocol === "http:" || url.protocol === "https:";
    };

    let date = citation.issued["date-parts"][0][0];
    let authors = "";
    let editors = "";
    if (citation.author)
        authors = citation.author
            .map((a) => {
                return a.literal ? a.literal : `${a.family}, ${a.given[0]}.`;
            })
            .join(", ");
    else if (citation.editor)
        editors = citation.editor
            .map((a) => {
                return a.literal ? a.literal : `${a.family} ${a.given}`;
            })
            .join(", ");

    let url: string = "";
    if (citation.publisher && citation.publisher.startsWith("http"))
        url = citation.publisher;
    else if (citation.URL) url = citation.URL;

    let inline = `${authors} (${date}). ${
        citation.type === "book" ? "<i>" : ""
    }${citation.title}${citation.type === "book" ? "</i>." : "."} ${
        citation["container-title"]
            ? " <i>" + citation["container-title"] + "</i>."
            : ""
    }${
        citation.publisher && !isValidHttpUrl(citation.publisher)
            ? " " + citation.publisher + "."
            : ""
    }`;

    let isExpanded = false;
    const toggleExpansion = () => {
        isExpanded = !isExpanded;
    };

    let isCitationHidden = true;
    const toggleCitationVisibility = () => {
        isCitationHidden = !isCitationHidden;
    };

    const bibtex = getBibtexEntry(citation.id);

    let hasCopiedToClipboard = false;
    const copyToClipboard = () => {
        navigator.clipboard.writeText(bibtex);
        hasCopiedToClipboard = true;
        setTimeout(() => {
            hasCopiedToClipboard = false;
        }, 2000);
    };

    const paragraphs = getParagraphReferences(citation.id);
</script>

<div
    id={citation.id}
    class="m-auto my-4  md:py-0 text-zinc-800 dark:text-zinc-50 bg-zinc-50 dark:bg-zinc-900 md:bg-transparent pointer-events-auto flex flex-col"
>
    <div class="flex justify-between">
        <div class="flex w-9/12">
            <div>{isExpanded ? "-" : "+"}&nbsp;</div>
            <div class="w-full">
                <div
                    class="cursor-pointer"
                    on:click={toggleExpansion}
                    on:keydown={toggleExpansion}
                >
                    {@html inline}
                </div>
                {#if isExpanded}
                    <div transition:slide class="flex my-3">
                        <div
                            class="flex items-center mr-6 cursor-pointer"
                            on:click={toggleCitationVisibility}
                            on:keydown={toggleCitationVisibility}
                        >
                            <div>
                                {#if isCitationHidden}
                                    <img
                                        width="22"
                                        height="22"
                                        class={`hidden dark:inline relative m-2 cursor-pointer`}
                                        src={`/images/interface/show-dark.svg`}
                                        alt={`icon to show the citation`}
                                    />
                                    <img
                                        width="22"
                                        height="22"
                                        class={`inline dark:hidden relative m-2 cursor-pointer`}
                                        src={`/images/interface/show.svg`}
                                        alt={`icon to show the citation`}
                                    />
                                {:else}
                                    <img
                                        width="22"
                                        height="22"
                                        class={`hidden dark:inline relative m-2 cursor-pointer`}
                                        src={`/images/interface/hide-dark.svg`}
                                        alt={`icon to hide the citation`}
                                    />
                                    <img
                                        width="22"
                                        height="22"
                                        class={`inline dark:hidden relative m-2 cursor-pointer`}
                                        src={`/images/interface/hide.svg`}
                                        alt={`icon to hide the citation`}
                                    />
                                {/if}
                            </div>
                            <div>{isCitationHidden ? "view" : "hide"}</div>
                        </div>
                        <div class="mr-6 flex items-center">
                            <div>
                                <img
                                    width="22"
                                    height="22"
                                    class={`hidden dark:inline relative m-2 cursor-pointer`}
                                    src={`/images/interface/copy-dark.svg`}
                                    alt={`icon to copy information to the clipboard`}
                                />
                                <img
                                    width="22"
                                    height="22"
                                    class={`inline dark:hidden relative m-2 cursor-pointer`}
                                    src={`/images/interface/copy.svg`}
                                    alt={`icon to copy information to the clipboard`}
                                />
                            </div>
                            <div
                                class="cursor-pointer"
                                on:click={copyToClipboard}
                                on:keydown={copyToClipboard}
                            >
                                {hasCopiedToClipboard ? "copied!" : "copy"}
                            </div>
                        </div>

                        {#if url !== ""}
                            <div class="mr-6 flex items-center">
                                <div>
                                    <img
                                        width="22"
                                        height="22"
                                        class={`hidden dark:inline relative m-2 cursor-pointer`}
                                        src={`/images/interface/link-dark.svg`}
                                        alt={`icon to open an external link`}
                                    />
                                    <img
                                        width="22"
                                        height="22"
                                        class={`inline dark:hidden relative m-2 cursor-pointer`}
                                        src={`/images/interface/link.svg`}
                                        alt={`icon to open an external link`}
                                    />
                                </div>
                                <div>
                                    <a
                                        href={url}
                                        target="_blank"
                                        class="underline">open</a
                                    >
                                </div>
                            </div>
                        {/if}
                    </div>
                    {#if !isCitationHidden}
                        <div transition:slide class="text-sm leading-0">
                            <div
                                class="whitespace-pre-wrap white bg-zinc-100 dark:bg-zinc-800 dark:text-zinc-300 p-2"
                            >
                                <code>
                                    {bibtex}
                                </code>
                            </div>
                        </div>
                    {/if}
                {/if}
            </div>
        </div>

        <div class="text-right">
            {#each paragraphs as par}
                <a href={`/${par.location}#par-${par.index}`} class="par"
                    >¶<span class="hover:underline">{par.index}</span></a
                >
            {/each}
        </div>
    </div>
</div>

<style>
    .par ~ .par::before {
        content: ", ";
    }
</style>
