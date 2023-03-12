import type { PageLoad } from '../$types';
import { findNodesByTag, getBundle, getListings } from "../../../utils/find";
import type { IListingsNode } from '../../../utils/types';

export const load = (({ }) => {
    const both = getListings() as Array<IListingsNode>;
    const l = both.filter((el) => { if (el.label.split(":")[0] == "code") return el; })
    const lo: {[k: string]: IListingsNode[]} = {};
    //-- structure listings by chapters
    l.map((el) => {
        if(!lo.hasOwnProperty(el.chapter_label))
            lo[el.chapter_label] = [el]
        else
            lo[el.chapter_label] = [...lo[el.chapter_label], el]
    })
    
    const f = both.filter((el) => { if (el.label.split(":")[0] == "graphic") return el; })
    const fo: {[k: string]: IListingsNode[]} = {};
    //-- structure figures by chapter
    f.map((el) => {
        if(!fo.hasOwnProperty(el.chapter_label))
            fo[el.chapter_label] = [el]
        else
            fo[el.chapter_label] = [...fo[el.chapter_label], el]
    })

    const a = getBundle("all")
    
    const fl = findNodesByTag('listing', a)
    const ff = findNodesByTag('figure', a)

    return {
        full_listings: fl,
        listings: lo,
        full_figures: ff,
        figures: fo,
    };
}) satisfies PageLoad;