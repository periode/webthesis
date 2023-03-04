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

    //-- handle the cases where the reference is to a chapter, if the reference is to another chapter, or if its a current anchor.
    const url = location === name ? `/${location}` : `${location !== "" ? "/" + location : ""}#${encodeURIComponent(
            name
        )}`

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

<span>
    &nbsp;<a class="font-mono hover:underline"
        href={`${location !== "" ? "/" + location : ""}#${encodeURIComponent(
            name
        )}`}
    >
        {literal}
        </a
    >&nbsp;
</span>
