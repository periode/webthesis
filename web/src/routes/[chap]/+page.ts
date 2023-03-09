import type { PageLoad } from './$types';
import { findChapterIncipit, getBundle, getToC } from "../../utils/find";

export const load = (({ params, depends }) => {
  const chap = params.chap;
  const root = getBundle(chap);
  const nodes = findChapterIncipit(root)
  const toc = getToC(chap)

  return {
    nodes: nodes,
    toc: toc
  };
}) satisfies PageLoad;