<script lang="ts">
    import type { INode } from "../../utils/types";
    import Paragraph from "./Paragraph.svelte";
    export let children: Array<INode> | null;
</script>

<div class="m-5 italic border-l-2">
    <Paragraph children={children}/>
</div>
