<script lang="ts">
    type Node = {
        tag: String,
        value: String,
        children: Array<Node> | null,
    }
    
    export let tag : String;
    export let value : String;
    export let children : Array<Node>;
</script>

<div class="node">
    <div>tag: {tag}</div>
    <div>value: {value ? value : "none"}</div>
    {#if children}
    <div>children: {children ? children.length : "none"}</div>
        {#each children as child}
<svelte:self tag={child.tag} value={child.value} children={child.children}/>
        {/each}
    {/if}

</div>

<style>
    .node {
        border: 1px solid #222;
        padding: 0.5em;
    }
</style>