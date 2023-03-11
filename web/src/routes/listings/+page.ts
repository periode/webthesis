import type { PageLoad } from './$types';
import { getListings } from "../../utils/find";

export const load = (({ }) => {
    const both = getListings()
    const l = both.filter((el) => { if (el.label.split(":")[0] == "code") return el; })
    const f = both.filter((el) => { if (el.label.split(":")[0] == "graphic") return el; })

    return {
        listings: l,
        figures: f,
    };
}) satisfies PageLoad;