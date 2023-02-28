import type { INode, IToCNode } from "./types";
import text_data from "./../routes/text.json";
import toc_data from "./../routes/toc.json";
const data = text_data as Array<INode>;
const toc = toc_data as Array<IToCNode>;

export const findNodeByTag = (nodes: Array<INode>, tag: string): INode | null => {
    var result = null;
    for (var i = 0; i < nodes.length; i++) {
        const n = nodes[i]

        if (n.tag === tag)
            return n;

        if (n.children) {
            result = findNodeByTag(n.children, tag);
            if (result)
                return result;

        }
    }
    return result;
}

export const findNodesByTag = (nodes: Array<INode>, tag: string): Array<INode> => {
    var result: Array<INode> = [];
    for (var i = 0; i < nodes.length; i++) {
        const n = nodes[i]

        if (n.tag === tag)
            result.push(n)

        if (n.children) {
            const r = findNodesByTag(n.children, tag);
            if (r)
                result.push(...r)

        }
    }
    return result;
}

export const findNodeByValue = (value: string, nodes?: Array<INode>): INode | null => {
    if(nodes === undefined)
        nodes = data;

    var result = null;
    for (var i = 0; i < nodes.length; i++) {
        const n = nodes[i]

        if (n.value === value)
            return n;

        if (n.children) {
            result = findNodeByValue(value, n.children);
            if (result)
                return result;

        }
    }
    return result;
}

export const findChapterInInclude = (include: string): INode => {
    const include_node = findNodeByValue(`${include}.tex`);
    if (include_node){
        const children = include_node.children as Array<INode>;
        const chap_node = findNodeByTag(children, "chapter")
        return chap_node as INode;
    }else{
        const err = {
            tag: "error",
            value: "could not find include",
            children: null,
        }
        return err;
    }
}

export const findToCNodeByLabel = (label: string, nodes?: Array<IToCNode>): IToCNode | null => {
    if(nodes === undefined)
        nodes = toc;

    var result = null;
    for (var i = 0; i < nodes.length; i++) {
        const n = nodes[i]

        if (n.label === label)
            return n;

        if (n.children) {
            result = findToCNodeByLabel(label, n.children);
            if (result)
                return result;

        }
    }
    return result;
}

export const findToCNodeByValue = (value: string, nodes?: Array<IToCNode>): IToCNode | null => {
    if(nodes === undefined)
        nodes = toc;

    var result = null;
    for (var i = 0; i < nodes.length; i++) {
        const n = nodes[i]

        if (n.value === value)
            return n;

        if (n.children) {
            result = findToCNodeByValue(value, n.children);
            if (result)
                return result;

        }
    }
    return result;
}

export const findHeadingValue = (label: string): string => {
    const node = findToCNodeByLabel(label)
    return node ? node.value : "ERROR";
}

//-- findLabel finds a particular label node.
//-- this is mostly used when references need to locate on which page the label is
export const findLabel = (value: string): INode | null => {
    var result = null;
    const labels = findNodesByTag(data, "label")

    for (let label of labels) {
        if (label.children && label.children[0].value === value)
            return label
    }
    return result;
}

export const findSection = (include: string, value: string): Array<INode> => {
    var result: INode[] = [];
    const root = findNodeByValue(`${include}.tex`, data)

    if (root && root.children) {
        if (root.children[0].children) {
            const all_paragraphs = root.children[0].children.map((el) => {
                if (el.children) {
                    return el
                }
            }) as Array<INode>

            let sectionFound = false;
            for (let i = 0; i < all_paragraphs.length; i++) {
                const par = all_paragraphs[i];

                if (sectionFound) {
                    //-- we break if we find the beginning of a new section
                    if (par.children && par.children[0].tag === "section")
                        return result;

                    result.push(par);

                } else {
                    //-- we look for the starting section, whose label should match the given value
                    if (par.children && par.children[0].tag === 'label') {
                        if (par.children[0].children) {
                            const label = par.children[0].children[0];
                            const label_value = label.value.split(":")[1]

                            if (label_value === value) {
                                sectionFound = true;
                                result.push(all_paragraphs[i - 1])
                                result.push(all_paragraphs[i])
                            }
                        }
                    }
                }
            }
        }
    }

    return result;
}