<script lang="ts">
    import { onMount } from "svelte";

    let icon_name = "lightmode";
    const toggleDarkMode = () => {
        const theme = window.localStorage.getItem("theme");

        if (!theme) {
            document.documentElement.classList.add("dark");
            window.localStorage.setItem("theme", "dark");
            icon_name = "darkmode";
        } else if (theme === "dark") {
            document.documentElement.classList.remove("dark");
            window.localStorage.setItem("theme", "light");
            icon_name = "lightmode";
        } else if (theme === "light") {
            document.documentElement.classList.add("dark");
            window.localStorage.setItem("theme", "dark");
            icon_name = "darkmode";
        }
    };

    onMount(() => {
        const theme = window.localStorage.getItem("theme");

        //-- only respect system defaults if the user hasn't explicity chosen the light theme
        if (
            window.matchMedia("(prefers-color-scheme: dark)").matches &&
            theme !== "light"
        ) {
            console.log(
                "mounting for dark mode:",
                window.matchMedia("(prefers-color-scheme: dark)").matches
            );

            document.documentElement.classList.add("dark");
            window.localStorage.setItem("theme", "dark");
            return;
        }

        if (!theme) {
            //-- handle user defaults here
            document.documentElement.classList.add("dark");
            window.localStorage.setItem("theme", "dark");
        } else if (theme === "dark") {
            document.documentElement.classList.add("dark");
        } else if (theme === "light") {
            document.documentElement.classList.remove("dark");
        }

        icon_name = theme ? `${theme}mode` : "lightmode";
    });
</script>

<div class="" on:click={toggleDarkMode} on:keydown={toggleDarkMode}>
    <img
        class="hidden dark:inline relative m-1 p-1 cursor-pointer"
        src={`/images/darkmode.svg`}
        alt={`icon to toggle dark mode or light mode`}
    />
    <img
        class="inline dark:hidden relative m-1 p-1 cursor-pointer"
        src={`/images/lightmode.svg`}
        alt={`icon to toggle dark mode or light mode`}
    />
</div>
