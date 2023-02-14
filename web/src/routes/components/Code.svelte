<script lang="ts">
    import hljs from "highlight.js";
    import type { INode } from "../../utils/types";
    import { getRawSourceCode } from "../../utils/snippet";
    export let nodes: Array<INode>;

    const raw = getRawSourceCode("testpy")
    
    let highlightedCode = hljs.highlightAuto(raw).value
</script>


<pre><code>{@html highlightedCode}</code></pre>