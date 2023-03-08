<script>
    import { page } from "$app/stores";
    import Spacer from "./components/Spacer.svelte";

    const { status } = $page;
</script>

<div
    class="container max-w-full bg-zinc-50 text-zinc-800 dark:bg-zinc-900 dark:text-zinc-50 p-2 lg:p-8 font-serif text-xl"
>
    <div class="w-full lg:w-5/12 md:w-6/12 m-auto text-center pt-12">
        <div class="text-2xl">Oops, we've encountered a {status} error.</div>
        <Spacer/>
        <div class="text-base">Return to <a href="/" class="underline md:no-underline hover:underline">the main page</a>.</div>
    </div>
</div>
