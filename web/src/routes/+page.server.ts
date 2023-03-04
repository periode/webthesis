import type { PageServerLoad } from './$types';
import { findNodeByTag, getBundle } from "../utils/find";

export const load = (async () => {
    const fp = getBundle("front");
    
    const t = findNodeByTag("title", fp)
    const title = t.children ? t.children[0].value: "MISSING TITLE"

    const au = findNodeByTag("author", fp)
    const author = au.children ? au.children[0].value: "MISSING AUTHOR"

    const af = findNodeByTag("affiliation", fp)
    const affiliation = af.children ? af.children[0].value: "MISSING AFFILIATION"

    const date = findNodeByTag("date", fp).value

    const ab = findNodeByTag("abstract", fp)
    const abstract = ab.children ? ab.children[0].children ? ab.children[0].children[0].value: "MISSING ABSTRACT" : "MISSING ABSTRACT"
    
  return {
    title: title,
    author: author,
    affiliation: affiliation,
    date: date,
    abstract: abstract,
  };
}) satisfies PageServerLoad;