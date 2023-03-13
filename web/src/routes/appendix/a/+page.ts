import type { PageLoad } from '../$types';
import { findNodesByTag, getBundle, getListings } from "../../../utils/find";
import type { IListingsNode } from '../../../utils/types';

export const load = (({ }) => {
    const both = getListings() as Array<IListingsNode>;
    
    //-- structure listings by chapters
    const l = both.filter((el) => { if (el.label.split(":")[0] == "code") return el; })
    const lo: {[k: string]: IListingsNode[]} = {};
    l.map((el) => {
        if(!lo.hasOwnProperty(el.chapter_label))
            lo[el.chapter_label] = [el]
        else
            lo[el.chapter_label] = [...lo[el.chapter_label], el]
    })
    
    //-- structure figures by chapter
    const f = both.filter((el) => { if (el.label.split(":")[0] == "graphic") return el; })
    const fo: {[k: string]: IListingsNode[]} = {};
    f.map((el) => {
        if(!fo.hasOwnProperty(el.chapter_label))
            fo[el.chapter_label] = [el]
        else
            fo[el.chapter_label] = [...fo[el.chapter_label], el]
    })

    return {
        listings: lo,
        figures: fo,
    };
}) satisfies PageLoad;