<script lang="ts">
    import { onMount } from "svelte";
    import DarkMode from "../DarkMode.svelte";

    const header_icon = "header";
    let theme: string | null = "";
    onMount(() => {
        theme = window.localStorage.getItem("theme");
    });

    let isExpanded = false;
    const toggleMenu = () => {
        isExpanded = !isExpanded;
    };
</script>

<div
    class="bg-zinc-50 dark:bg-zinc-900 md:bg-transparent w-full md:w-auto fixed top-0 p-1 pb-2 md:p-2 border-b border-b-zinc-900 dark:border-b-zinc-300 md:border-none flex z-10"
>
    <img
        width="28"
        height="28"
        on:click={toggleMenu}
        on:keydown={toggleMenu}
        class={`inline dark:hidden relative m-1 p-1 cursor-pointer`}
        src={`/images/${isExpanded ? header_icon + "-open" : header_icon}.svg`}
        alt={`icon to toggle dark mode or light mode`}
    />
    <img
        width="28"
        height="28"
        on:click={toggleMenu}
        on:keydown={toggleMenu}
        class={`hidden dark:inline relative m-1 p-1 cursor-pointer`}
        src={`/images/${
            isExpanded ? header_icon + "-open" : header_icon
        }-dark.svg`}
        alt={`icon to toggle dark mode or light mode`}
    />
    <div class={`${isExpanded ? "inline" : "hidden"} flex`}>
        <DarkMode />
        <!-- home -->
        <div>
            <a href="/">
                <img
                    width="28"
                    height="28"
                    class={`hidden dark:inline relative m-1 p-1 cursor-pointer`}
                    src={`/images/home-dark.svg`}
                    alt={`icon to get back to the index page`}
                />
                <img
                    width="28"
                    height="28"
                    class={`inline dark:hidden relative m-1 p-1 cursor-pointer`}
                    src={`/images/home.svg`}
                    alt={`icon to get back to the index page`}
                />
            </a>
        </div>
        <!-- download -->
        <div>
            <a href="/thesis.pdf" download>
                <img
                    width="28"
                    height="28"
                    class={`hidden dark:inline relative m-1 p-1 cursor-pointer`}
                    src={`/images/file-download-dark.svg`}
                    alt={`icon to download the pdf of the thesis`}
                />
                <img
                    width="28"
                    height="28"
                    class={`inline dark:hidden relative m-1 p-1 cursor-pointer`}
                    src={`/images/file-download.svg`}
                    alt={`icon to download the pdf of the thesis`}
                />
            </a>
        </div>
    </div>
</div>
