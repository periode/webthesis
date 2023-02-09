<script>
    import parsed_data from "./data.json";
    import Node from './components/Node.svelte';
</script>

<h1>webthesis</h1>
{#each parsed_data as d}
   <Node tag={d.tag} value={d.value} children={d.children}/>
{/each}