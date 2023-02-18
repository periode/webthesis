import type { INode } from "./types";

export const findNodeByTag = (nodes: Array<INode>, tag: string): INode | null => {
    var result = null;
    for (var i = 0; i < nodes.length; i++) {
        const n = nodes[i]

        if (n.tag === tag)
            return n;
        
        if (n.children) {
            result = findNodeByTag(n.children, tag);
            if(result)
                return result;
            
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
            if(result)
                return result;
            
        }
    }
    return result;
}