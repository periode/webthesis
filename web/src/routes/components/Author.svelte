<script lang="ts">
    export let value: string;

    const escapeCharacter = (input: string): string[] => {        
        return input.split("\\\\");
    };
</script>

<div class="my-32">
    {#each escapeCharacter(value) as line}
    {`${line}`}<br/>
    {/each}
</div>
