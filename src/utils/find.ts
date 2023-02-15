import type { INode } from "./types";

export const findNode = (nodes: Array<INode>, tag: string): INode | null => {
    var result = null;
    for (var i = 0; i < nodes.length; i++) {
        const n = nodes[i]

        if (n.tag === tag)
            return n;
        
        if (n.children) {
            result = findNode(n.children, tag);
            if(result)
                return result;
            
        }
    }
    return result;
}