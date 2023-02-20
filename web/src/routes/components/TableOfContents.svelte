<script lang="ts">
    const chapters = [
        "introduction",
        "ideals",
        "understanding",
        "beauty",
        "programming",
        "conclusion",
    ];
</script>

<div class="w-full lg:w-6/12 md:w-8/12 m-auto my-4 py-2">
    <h3 class="my-4 text-2xl">Table of contents</h3>
    <ol class="list-decimal list-inside">
        {#each chapters as chap}
            <li><a href={`/${chap}`} class="hover:underline">{chap}</a></li>
        {/each}
    </ol>
</div>

