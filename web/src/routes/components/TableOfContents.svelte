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

<div class="my-4 py-2">
    <h3 class="my-4 text-2xl">Table of contents</h3>
    <ol class="list-decimal list-inside">
        {#each chapters as chap}
            <li><a href={`/${chap}`} class="hover:underline">{chap}</a></li>
        {/each}
    </ol>
</div>

