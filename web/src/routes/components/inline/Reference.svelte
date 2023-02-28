<script lang="ts">
    import { findHeadingValue, findLabel } from "../../../utils/find";
    import type { INode } from "../../../utils/types";
    export let node: INode;
    const value = node.children ? node.children[0].value : "Missing reference";

    const label = findLabel(value);
    const location = label ? label.value : "";
    let type = value.split(":")[0];
    let name = value.split(":")[1];
    let literal = name;

    if (
        type == "chap" ||
        type == "sec" ||
        type == "subsec" ||
        type == "subsubsec"
    ) {
        literal = findHeadingValue(value);
        type = "heading";
    }
</script>

<span class="font-bold">
    &nbsp;<a
        href={`${location !== "" ? "/" + location : ""}#${encodeURIComponent(
            name
        )}`}
    >
        {literal}
        {#if type == "code" || type == "graphic"}
            <img
                class="inline dark:hidden relative bottom-1 left-1"
                src={`/images/${type}.svg`}
                alt={`icon to reference the ${value} item`}
            />
            <img
                class="hidden dark:inline relative bottom-1 left-1"
                src={`/images/${type}-dark.svg`}
                alt={`icon to reference the ${value} item`}
            />
        {/if}</a
    >&nbsp;&nbsp;
</span>
