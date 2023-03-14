<script>
  import "../app.css";
  import Header from "../components/interface/Header.svelte";
  import { onMount } from "svelte";
  import { pwaInfo } from "virtual:pwa-info";

  onMount(async () => {
    if (pwaInfo) {
      const { registerSW } = await import("virtual:pwa-register");
      registerSW({
        immediate: true,
        onRegistered(r) {
          console.log(`SW Registered: ${r}`);
        },
        onRegisterError(error) {
          console.log("SW registration error", error);
        },
      });
    }

    if (window)
      // @ts-ignore todo: have TS declarations for hypothesis
      window.hypothesisConfig = function () {
        return {
          theme: "clean",
        };
      };
  });

  $: webManifest = pwaInfo ? pwaInfo.webManifest.linkTag : "";
</script>

<svelte:head>
  <script src="https://hypothes.is/embed.js" async></script>
  {@html webManifest}
</svelte:head>

<Header />

<slot />
