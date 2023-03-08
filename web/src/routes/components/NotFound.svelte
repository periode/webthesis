<script lang="ts">
    import Spacer from "./Spacer.svelte";

    export const status: string = "";
</script>

<div class="w-full lg:w-5/12 md:w-6/12 m-auto text-center pt-12">
    <div class="text-2xl">
        Oops, we've encountered an error{status !== ""
            ? `(specifically, a ${status} error)`
            : ""}.
    </div>
    <Spacer />
    <div class="text-base">
        Return to <a href="/" class="underline md:no-underline hover:underline"
            >the main page</a
        >.
    </div>
</div>
