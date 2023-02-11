export enum NodeType {
    Chapter = 'chapter',
    Listing = 'listing',
    Literal = 'literal',
    Paragraph = 'paragraph',
    Section = 'section',
    Subsection = 'subsection',
    Subsubsection = 'subsubsection',
    Emphasis = 'emph',
    Citation = 'citation',
}

export interface INode {
    tag: String;
    value: String;
    children: Array<INode> | null;
};