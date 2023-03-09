<script lang="ts">
    import hljs from "highlight.js";

    import { getRawSourceCode } from "../../../utils/snippet";
    export let path: string;
    export let lang: string

    const fname = path.replace("./corpus/", "")
    const raw = getRawSourceCode(fname);

    let highlightedCode = hljs.highlightAuto(raw).value;
</script>

<pre><code lang={lang}>{@html highlightedCode}</code></pre>
