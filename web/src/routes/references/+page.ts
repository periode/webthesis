import type { PageLoad } from '../$types';
import { getBibliography } from '../../utils/find';


export const load = (() => {
    const bib = getBibliography();

    //-- sort alphabeltically
    bib.sort((a, b) => {
        const first = a.author ? a.author[0].family ? a.author[0].family.toUpperCase() : a.author[0].literal.toUpperCase() : a.editor[0].family.toUpperCase()

        const second = b.author ? b.author[0].family ? b.author[0].family.toUpperCase() : b.author[0].literal.toUpperCase() : b.editor[0].family.toUpperCase()

        if (first < second) return -1
        if (second < first) return 1
        return 0
    })

    return {
        citations: bib,
    };
}) satisfies PageLoad;