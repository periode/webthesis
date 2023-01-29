<script>
    import parsed_data from "./data.json";
</script>

<h1>webthesis</h1>
{#each parsed_data as d}
    <h2>{d._type}</h2>
    <p>{d.value}</p>
{/each}
