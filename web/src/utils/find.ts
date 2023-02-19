import type { INode } from "./types";
import json_data from "./../routes/data.json";

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

export const findNodeByValue = (nodes: Array<INode>, value: string): INode | null => {
    var result = null;
    for (var i = 0; i < nodes.length; i++) {
        const n = nodes[i]

        if (n.value === value)
            return n;

        if (n.children) {
            result = findNodeByValue(n.children, value);
            if (result)
                return result;

        }
    }
    return result;
}

//-- findLabel finds a particular label node.
//-- this is mostly used when references need to locate on which page the label is
export const findLabel = (value: string): INode | null => {
    var result = null;
    const data = json_data as Array<INode>;
    const labels = findNodesByTag(data, "label")

    for(let label of labels){
        if(label.children && label.children[0].value === value)
            return label
    }
    return result;
}