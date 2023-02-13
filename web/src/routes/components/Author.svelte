<script lang="ts">
    import type { INode } from "../../utils/types";
    export let node: INode;
    const value = node.children ? node.children[0].value : "Missing author";

    const splitLatexLinebreaks = (input: string): string[] => {
        return input.split("\\\\");
    };
</script>

<div class="my-32">
    {#each splitLatexLinebreaks(value) as line}
        {`${line}`}<br />
    {/each}
</div>
