<script lang="ts">
    export let author: string;    
    const splitLatexLinebreaks = (input: string): string[] => {
        return input.split("\\\\");
    };
</script>

<div class="my-16">
    {#each splitLatexLinebreaks(author) as line}
        {`${line}`}<br />
    {/each}
</div>
