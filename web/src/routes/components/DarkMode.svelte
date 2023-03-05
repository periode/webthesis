<script lang="ts">
    import { onMount } from "svelte";

    let icon_name = "lightmode";
    const toggleDarkMode = () => {
        const theme = localStorage.theme;

        if (!theme) {
            document.documentElement.classList.add("dark");
            localStorage.setItem("theme", "dark");
        } else if (theme === "dark") {
            document.documentElement.classList.remove("dark");
            localStorage.setItem("theme", "light");
        } else if (theme === "light") {
            document.documentElement.classList.add("dark");
            localStorage.setItem("theme", "dark");
        }
        icon_name = `${localStorage.theme}mode`;
    };

    onMount(() => {
        let theme = localStorage.theme;

        //-- only respect system defaults if the user hasn't explicity chosen the light theme
        if (
            theme === "dark" ||
            (!("theme" in localStorage) &&
                window.matchMedia("(prefers-color-scheme: dark)").matches)
        ) {
            theme = "dark";
            document.documentElement.classList.add("dark");
        } else {
            theme = "light";
            document.documentElement.classList.remove("dark");
        }

        icon_name = theme ? `${theme}mode` : "lightmode";
    });
</script>

<div class="" on:click={toggleDarkMode} on:keydown={toggleDarkMode}>
    <img
        width="28"
        height="28"
        class="hidden dark:inline relative m-1 p-1 cursor-pointer"
        src={`/images/darkmode.svg`}
        alt={`icon to toggle dark mode or light mode`}
    />
    <img
        width="28"
        height="28"
        class="inline dark:hidden relative m-1 p-1 cursor-pointer"
        src={`/images/lightmode.svg`}
        alt={`icon to toggle dark mode or light mode`}
    />
</div>
