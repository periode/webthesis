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
    Code = 'code',
    Caption = 'caption',
    Label = 'label',
    Include = 'include',
    Root = 'root',
    Quote = 'quote',
    Footnote = 'footnote',
    Reference = 'reference',
    InlineListing = 'inline_listing',
}

export interface INode {
    tag: string;
    value: string;
    children: Array<INode> | null;
};