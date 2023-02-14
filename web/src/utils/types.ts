export enum NodeType {
    Chapter = 'chapter',
    Listing = 'listing',
    Literal = 'literal',
    Paragraph = 'paragraph',
    Section = 'section',
    Subsection = 'subsection',
    Subsubsection = 'subsubsection',
    Emphasis = 'emph',
    Italic = 'italic',
    Citation = 'citation',
    Code = 'code',
    Figure = 'listing',
    Caption = 'caption',
    Label = 'label',
    Include = 'include',
    Root = 'root',
    Quote = 'quote',
    Footnote = 'footnote',
    Reference = 'reference',
    InlineListing = 'inline_listing',
    Title = 'title',
    Author = 'author',
    Affiliation = 'affiliation',
    Date = 'date',
}

export interface INode {
    tag: string;
    value: string;
    children: Array<INode> | null;
};