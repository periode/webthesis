<script lang="ts">
    import { onMount } from "svelte";

    let icon_name = "lightmode";
    const toggleDarkMode = () => {
        const theme = window.localStorage.getItem("theme");

        if (!theme) {
            document.documentElement.classList.add("dark");
            window.localStorage.setItem("theme", "dark");
            icon_name = "darkmode"
        } else if (theme === "dark") {
            document.documentElement.classList.remove("dark");
            window.localStorage.setItem("theme", "light");
            icon_name = "lightmode"
        } else if (theme === "light") {
            document.documentElement.classList.add("dark");
            window.localStorage.setItem("theme", "dark");
            icon_name = "darkmode"
        }
    };

    
    onMount(() => {
        const theme = window.localStorage.getItem("theme");
        if (!theme) {
            //-- handle user defaults here
            document.documentElement.classList.add("dark");
            window.localStorage.setItem("theme", "dark");
        } else if (theme === "dark") {
            document.documentElement.classList.add("dark");
        } else if (theme === "light") {
            document.documentElement.classList.remove("dark");
        }

        icon_name = theme ? `${theme}mode` : "lightmode"
    });
</script>

<div
    class="fixed top-2 right-2 text-zinc-800 dark:text-zinc-50"
    on:click={toggleDarkMode}
    on:keydown={toggleDarkMode}
>
    <img
        class="inline relative m-1 p-1 cursor-pointer"
        src={`/images/${icon_name}.svg`}
        alt={`icon to toggle dark mode or light mode`}
    />
</div>
