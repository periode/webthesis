<script lang="ts">
    export let value: string;

    let isVisible = false;
    const showFootnote = () => {
        isVisible = !isVisible;
    };
</script>

<span>
    <sup
        on:mouseenter={showFootnote}
        on:click={showFootnote}
        on:keydown={showFootnote}
        class="font-bold cursor-pointer"
    >
        ?
    </sup>
    <div
        class={`absolute left-12 z-10 w-full lg:w-5/12 md:w-8/12 p-4 font-normal text-md text-left min-h-max min-w-max border border-slate-700 bg-slate-50 ${
            isVisible ? "" : "hidden"
        }`}
    >
        {value}

        <div
            on:click={() => (isVisible = false)}
            on:keydown={() => (isVisible = false)}
            class="absolute top-0 right-1 cursor-pointer"
        >
            x
        </div>
    </div>
</span>
