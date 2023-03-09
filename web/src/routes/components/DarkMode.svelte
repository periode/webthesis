<script lang="ts">
    import { dark_mode } from "../../stores";

    const toggleDarkMode = () => {
        $dark_mode = $dark_mode === "dark" ? "light" : "dark";

        if ($dark_mode === "dark")
            document.documentElement.classList.add("dark");
        else document.documentElement.classList.remove("dark");
        
        localStorage.setItem("theme", $dark_mode);
    };
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
