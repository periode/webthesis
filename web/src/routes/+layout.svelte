<script>
  import "../app.css";
  import Header from "./components/interface/Header.svelte";
</script>

<svelte:head>
  <meta
    name="viewport"
    content="width=device-width, initial-scale=1, maximum-scale=1, user-scalable=0"
  />
</svelte:head>

<Header />

<slot />

