<script lang="ts">
    import { onMount } from "svelte";
    import DarkMode from "./DarkMode.svelte";
    import { fade } from "svelte/transition";
    import { dark_mode } from "../../stores";

    const header_icon = "header";
    onMount(() => {
        //-- only respect system defaults if the user hasn't explicity chosen the light theme
        if (
            $dark_mode === "dark" ||
            (!("theme" in localStorage) &&
                window.matchMedia("(prefers-color-scheme: dark)").matches)
        ) {
            $dark_mode = "dark";
            document.documentElement.classList.add("dark");
        } else {
            $dark_mode = "light";
            document.documentElement.classList.remove("dark");
        }
    });

    let isExpanded = false;
    const toggleMenu = () => {
        isExpanded = !isExpanded;
    };
</script>

<header
    class="bg-zinc-50 dark:bg-zinc-900 md:bg-transparent w-full md:w-auto fixed top-0 p-1 pb-2 md:p-2 border-b border-b-zinc-900 dark:border-b-zinc-300 md:border-none flex z-10"
>
    <img
        width="28"
        height="28"
        on:click={toggleMenu}
        on:keydown={toggleMenu}
        class={`inline dark:hidden relative m-1 p-1 cursor-pointer`}
        src={`/images/interface/${
            isExpanded ? header_icon + "-open" : header_icon
        }.svg`}
        alt={`icon to toggle dark mode or light mode`}
    />
    <img
        width="28"
        height="28"
        on:click={toggleMenu}
        on:keydown={toggleMenu}
        class={`hidden dark:inline relative m-1 p-1 cursor-pointer`}
        src={`/images/interface/${
            isExpanded ? header_icon + "-open" : header_icon
        }-dark.svg`}
        alt={`icon to toggle dark mode or light mode`}
    />
    {#if isExpanded}
        <div transition:fade={{ duration: 100 }} class={`flex`}>
            <DarkMode />
            <!-- home -->
            <div>
                <a href="/">
                    <img
                        width="28"
                        height="28"
                        class={`hidden dark:inline relative m-1 p-1 cursor-pointer`}
                        src={`/images/interface/home-dark.svg`}
                        alt={`home icon`}
                    />
                    <img
                        width="28"
                        height="28"
                        class={`inline dark:hidden relative m-1 p-1 cursor-pointer`}
                        src={`/images/interface/home.svg`}
                        alt={`home icon`}
                    />
                </a>
            </div>
            <!-- download -->
            <div>
                <a href="/Depaz_AestheticsUnderstandingSourceCode.pdf" download>
                    <img
                        width="28"
                        height="28"
                        class={`hidden dark:inline relative m-1 p-1 cursor-pointer`}
                        src={`/images/interface/file-download-dark.svg`}
                        alt={`download icon`}
                    />
                    <img
                        width="28"
                        height="28"
                        class={`inline dark:hidden relative m-1 p-1 cursor-pointer`}
                        src={`/images/interface/file-download.svg`}
                        alt={`download icon`}
                    />
                </a>
            </div>
            <!-- about -->
            <div>
                <a href="/about">
                    <img
                        width="28"
                        height="28"
                        class={`hidden dark:inline relative m-1 p-1 cursor-pointer`}
                        src={`/images/interface/question-dark.svg`}
                        alt={`about icon`}
                    />
                    <img
                        width="28"
                        height="28"
                        class={`inline dark:hidden relative m-1 p-1 cursor-pointer`}
                        src={`/images/interface/question.svg`}
                        alt={`about icon`}
                    />
                </a>
            </div>
        </div>
    {/if}
</header>
