import type { PageLoad } from './$types';
import { findChapterIncipit, getBundle, getToC } from "../../utils/find";

export const load = (({params}) => {    
    const root = getBundle(params.chap);
    const nodes = findChapterIncipit(root)
    const toc = getToC(params.chap)
    
  return {
    nodes: nodes,
    toc: toc

  };
}) satisfies PageLoad;