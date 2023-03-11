<script lang="ts">
    import {
        findFullReference,
        findHeadingValue,
        findLabel,
        findToCNodeByLabel,
    } from "../../utils/find";
    import type { INode } from "../../utils/types";
    export let node: INode;
    const value = node.children ? node.children[0].value : "Missing reference";

    const label = findLabel(value);
    const location = label ? label.value : "";
    let type = value.split(":")[0];
    let name = value.split(":")[1];
    let literal = name;
    let url = "";

    if (
        type == "chap" ||
        type == "sec" ||
        type == "subsec" ||
        type == "subsubsec"
    ) {
        if (type === "chap") {
            url = `/${location}`;
        } else if (type === "sec") {
            const current = findToCNodeByLabel(value);
            if (current)
                url = `/${current.parent?.label.split(":")[1]}/${name}`;
        } else if (type == "subsec" || type == "subsubsec") {
            //-- handle the case where the reference is to a subsection, meaning we need to get the chapter and section where that subsection is

            const current = findToCNodeByLabel(value);
            if (current && current.parent) {
                const sec = findToCNodeByLabel(current.parent.label);

                if (sec && sec.parent) {
                    if (sec.parent.tag === "Chapter") {
                        url = `/${sec.parent.label.split(":")[1]}/${
                            current.parent.label.split(":")[1]
                        }#${encodeURIComponent(name)}`;
                    } else {
                        //-- we are dealing with a subsubsection
                        const subsec = findToCNodeByLabel(sec.parent.label);

                        if (subsec && subsec.parent) {
                            url = `/${subsec.parent.label.split(":")[1]}/${
                                subsec.label.split(":")[1]
                            }#${encodeURIComponent(name)}`;
                        }
                    }
                }
            }
        }

        type = "heading";
        literal = findHeadingValue(value);
    } else {
        let reference;
        if (node.children)
            reference = findFullReference(node.children[0].value);

        if (reference)
            url = `/${
                reference.chapter_label !== ""
                    ? reference.chapter_label + "/"
                    : ""
            }${
                reference.section_label !== "" ? reference.section_label : ""
            }#${encodeURIComponent(name)}`;
        else url = `#${encodeURIComponent(name)}`;        
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
