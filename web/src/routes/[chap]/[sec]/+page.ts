import type { PageLoad } from './$types';
import { findChapterInInclude, findSection, getBundle } from "../../../utils/find";

export const load = (({params}) => {    
    const root = getBundle(params.chap);
    const nodes = findSection(params.chap, params.sec, root);

    const chap_node = findChapterInInclude(params.chap, root);
    const chap_value = chap_node.children
        ? chap_node.children[0].value
        : "MISSING CHAP";
    
  return {
    nodes: nodes,
    chapter: chap_value,
  };
}) satisfies PageLoad;