import type { INode, IToCNode } from "./types";
import text_data from "../data/text.json";
import toc_data from "../data/toc.json";
import front_data from "../data/front.json";
import introduction_data from "../data/introduction.json";
import ideals_data from "../data/ideals.json";
import understanding_data from "../data/understanding.json";
import beauty_data from "../data/beauty.json";
import programming_data from "../data/programming.json";
import conclusion_data from "../data/conclusion.json";
const full_text = text_data as Array<INode>;
const toc = toc_data as Array<IToCNode>;

export const findNodeByTag = (tag: string, nodes: Array<INode>): INode => {
    var result: INode = { tag: "error", value: "error", children: null };
    for (var i = 0; i < nodes.length; i++) {
        const n = nodes[i]

        if (n.tag === tag)
            return n;

        if (n.children) {
            result = findNodeByTag(tag, n.children);
            if (result.tag !== "error")
                return result;

        }
    }
    return result;
}

//-- returns the full toc if chap is empty, or the ToC of a specific chapter
export const getToC = (chap?: string): Array<IToCNode> => {
    const includes = toc[0].children as Array<IToCNode>;

    if(chap === undefined || chap === null || chap === ""){
        let chapters: IToCNode[] = [];
        for(let inc of includes)
            if(inc.children)
                chapters.push(inc.children[0])
        return chapters
    }

    
    for(let inc of includes){
        if(inc.children && inc.children[0].label === `chap:${chap}`)
            return inc.children[0].children as Array<IToCNode>
    }
    return [];
}

export const getBundle = (name: string): Array<INode> => {
    let bundle;
    switch (name) {
        case "front":
            bundle = front_data as Array<INode>;
            break;
        case "introduction":
            bundle = [introduction_data as INode];
            break;
        case "ideals":
            bundle = [ideals_data as INode];
            break;
        case "understanding":
            bundle = [understanding_data as INode];
            break;
        case "beauty":
            bundle = [beauty_data as INode];
            break;
        case "programming":
            bundle = [programming_data as INode];
            break;
        case "conclusion":
            bundle = [conclusion_data as INode];
            break;
        default:
            return []
    }

    return bundle;
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

export const findNodeByValue = (value: string, nodes: Array<INode>): INode | null => {

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

//-- findChapterInInclude returns the chapter node of a particular include
//-- it's used when rendering a section in order to get the chapter to which the section belongs
export const findChapterInInclude = (include: string, nodes: Array<INode>): INode => {
    const include_node = findNodeByValue(`${include}.tex`, nodes);
    if (include_node) {
        const children = include_node.children as Array<INode>;
        const chap_node = findNodeByTag("chapter", children)
        return chap_node;
    } else {
        const err = {
            tag: "error",
            value: "could not find include",
            children: null,
        }
        return err;
    }
}

export const findToCNodeByLabel = (label: string, nodes?: Array<IToCNode>): IToCNode | null => {
    if (nodes === undefined)
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
    if (nodes === undefined)
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

//-- findHeadingValue takes a label and returns the literal for the heading associated with the label
export const findHeadingValue = (label: string): string => {
    const node = findToCNodeByLabel(label)
    return node ? node.value : "ERROR";
}

//-- findLabel finds a particular label node.
//-- this is mostly used when references need to locate on which page the label is
export const findLabel = (value: string): INode | null => {
    var result = null;
    const labels = findNodesByTag(full_text, "label")

    for (let label of labels) {
        if (label.children && label.children[0].value === value)
            return label
    }
    return result;
}

export const findPreviousToC = (chapter: string, section: string): IToCNode | null => {
    let start_nodes: Array<IToCNode> = [];
    let label = ""

    if (section === "") { //-- find chapter siblings
        start_nodes = toc[0] ? toc[0].children ? toc[0].children : [] : [];
        label = chapter;
    } else { //-- find section siblings
        const chapter_nodes = toc[0] ? toc[0].children ? toc[0].children : [] : [];
        for (let chap of chapter_nodes)
            for (let c of chap.children as Array<IToCNode>)
                if (c.label == chapter)
                    start_nodes = c.children as Array<IToCNode>;
        label = section;
    }


    for (let i = 0; i < start_nodes.length; i++) {
        const n = start_nodes[i];

        if (n.label === label) { //-- checking at the section level
            return (i == 0) ? null : start_nodes[i - 1];
        }

        if (n.children && n.children[0].label === label) { //-- checking at the chapter level
            // found! now check if it's the first
            if (i == 0)
                return null
            else
                if (start_nodes[i - 1].children) {
                    const c = start_nodes[i - 1].children as Array<IToCNode>;
                    return c[0];
                }

        }
    }

    return null;
}

export const findNextToC = (chapter: string, section: string): IToCNode | null => {
    let start_nodes: Array<IToCNode> = [];
    let label = ""

    if (section === "") { //-- chapter
        start_nodes = toc[0] ? toc[0].children ? toc[0].children : [] : [];
        label = chapter;
    } else { //-- section
        const chapter_nodes = toc[0] ? toc[0].children ? toc[0].children : [] : [];
        for (let chap of chapter_nodes)
            for (let c of chap.children as Array<IToCNode>)
                if (c.label == chapter)
                    start_nodes = c.children as Array<IToCNode>;
        label = section;
    }


    for (let i = 0; i < start_nodes.length; i++) {
        const n = start_nodes[i];

        if (n.label === label) { //-- checking at the section level
            return (i == start_nodes.length - 1) ? null : start_nodes[i + 1];
        }

        if (n.children && n.children[0].label == label) {
            // found! now check if it's the first
            if (i == start_nodes.length - 1)
                return null
            else
                if (start_nodes[i + 1].children) {
                    const c = start_nodes[i + 1].children as Array<IToCNode>;
                    return c[0];
                }
        }
    }

    return null;
}

//-- findSection returns an interval of nodes from a starting section (value), until the next section or the end of the include (chapter)
export const findSection = (include: string, section: string, nodes: Array<INode>): Array<INode> => {
    var result: INode[] = [];

    const root = findNodeByValue(`${include}.tex`, nodes)

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

                            if (label_value === section) {
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

//-- findChapterIncipit returns all the nodes of a given input (root) until reaching the first section
export const findChapterIncipit = (root: Array<INode>): Array<INode> => {
    let result: Array<INode> = [];
    const r = root[0]
    if (r.children && r.children[0].children) {
        const all_paragraphs = r.children[0].children.map((el) => {
            if (el.children) {
                return el
            }
        }) as Array<INode>

        for (let i = 0; i < all_paragraphs.length; i++) {
            const par = all_paragraphs[i];

            //-- we break if we find the beginning of a new section
            if (par.children && par.children[0].tag === "section")
                return result;

            result.push(par);

        }
    }
    return result
}