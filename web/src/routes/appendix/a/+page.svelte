<script lang="ts">
    import ListingItem from "../../../components/appendix/ListingItem.svelte";
    import type { IListingsNode, INode } from "../../../utils/types";
    import type { PageData } from "../$types";
    import FigureItem from "../../../components/appendix/FigureItem.svelte";
    import Spacer from "../../../components/block/Spacer.svelte";
    import { findHeadingValue } from "../../../utils/find";

    export let data: PageData;

    const listings = data.listings as { [k: string]: Array<IListingsNode> };
    const full_listings = data.full_listings as Array<INode>;
    const figures = data.figures as { [k: string]: Array<IListingsNode> };
    const full_figures = data.full_figures as Array<INode>;
</script>

<div
    class="container max-w-full bg-zinc-50 text-zinc-800 dark:bg-zinc-900 dark:text-zinc-50 p-2 lg:p-3 font-serif md:text-lg"
>
    <h1 class="w-full lg:w-5/12 md:w-6/12 m-auto text-4xl mt-24 mb-8">
        Appendix A: Figures and Listings
    </h1>
    <hr class="w-8/12 lg:w-5/12 md:w-6/12 m-auto my-10" />
    <main class="w-full lg:w-5/12 md:w-6/12 m-auto mb-10">
        <section>
            <h2 class="text-2xl mt-24 mb-8">List of listings</h2>
            <ol class="w-full list-inside list-decimal">
                {#each Object.entries(listings) as [chapter, children]}
                    <div class="text-xl my-8 font-semibold">{findHeadingValue(`chap:${chapter}`)}</div>
                    {#each children as listing}
                        <ListingItem {listing} node={full_listings[listing.index]} />
                    {/each}
                {/each}
            </ol>
        </section>

        <Spacer />

        <section>
            <h2 class="text-2xl mt-24 mb-8">List of figures</h2>
            <ol class="w-full list-inside list-decimal">
                {#each Object.entries(figures) as [chapter, children]}
                <div class="text-xl my-8 font-semibold">{findHeadingValue(`chap:${chapter}`)}</div>
                {#each children as figure}
                    <FigureItem {figure} node={full_figures[figure.index]} />
                {/each}
            {/each}
            </ol>
        </section>
    </main>
</div>
