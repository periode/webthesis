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
    let url = "";

    console.log("location:", location, " value:", value, "type:", type);

    if (
        type == "chap" ||
        type == "sec" ||
        type == "subsec" ||
        type == "subsubsec"
    ) {
        if (type === "chap") {
            url = `/${location}`;
        } else if (type === "sec") {
            url = `/${location}/${name}`;
        } else {
            //-- handle the cases where the reference is to a chapter, if the reference is to another chapter, or if its a current anchor.
            url =
                location === name
                    ? `/${location}`
                    : `${
                          location !== "" ? "/" + location : ""
                      }#${encodeURIComponent(name)}`;
        }

        type = "heading";
        literal = findHeadingValue(value);
    } else {
        url = `#${encodeURIComponent(name)}`;
    }

    const target = type === "heading" ? "_self" : "";
</script>

<span>
    &nbsp;<a
        {target}
        class={`${type === "code" ? "font-mono" : "underline"} hover:underline`}
        href={url}
    >
        {literal}
    </a>&nbsp;
</span>
