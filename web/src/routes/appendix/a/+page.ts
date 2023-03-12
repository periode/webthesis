import type { PageLoad } from '../$types';
import { findNodesByTag, getBundle, getListings } from "../../../utils/find";

export const load = (({ }) => {
    const both = getListings()
    const l = both.filter((el) => { if (el.label.split(":")[0] == "code") return el; })
    const f = both.filter((el) => { if (el.label.split(":")[0] == "graphic") return el; })
    const a = getBundle("all")
    
    const fl = findNodesByTag('listing', a)
    const ff = findNodesByTag('figure', a)

    return {
        full_listings: fl,
        listings: l,
        full_figures: ff,
        figures: f,
    };
}) satisfies PageLoad;