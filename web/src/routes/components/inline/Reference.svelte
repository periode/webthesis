<script lang="ts">
    import type { INode } from "../../../utils/types";
    export let node: INode;
    const value = node.children ? node.children[0].value : "Missing reference";

    const type = value.split(":")[0];
    const name = value.split(":")[1];
    // console.log("reference with type:", type)
</script>

<span class="font-bold">
    &nbsp;<a href={`/#${encodeURIComponent(value)}`}
        >{name}{#if type == "code" || type == "graphic"}<img
                class="inline dark:hidden relative bottom-1 left-1"
                src={`/images/${type}.svg`}
                alt={`icon to reference the ${value} item`}
            />
            <img
                class="hidden dark:inline relative bottom-1 left-1"
                src={`/images/${type}-dark.svg`}
                alt={`icon to reference the ${value} item`}
            />{/if}</a
    >&nbsp;&nbsp;
</span>
