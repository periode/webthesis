import type { PageLoad } from './$types';
import { getBundle } from "../../utils/find";
import type { INode } from '../../utils/types';

export const load = (({params}) => {    
    const root = getBundle(params.chap);
    const nodes = root ? root : [] as Array<INode>;
    
  return {
    nodes: nodes
  };
}) satisfies PageLoad;