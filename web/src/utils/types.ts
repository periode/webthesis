export enum NodeType {
    Chapter = 'chapter',
    Listing = 'listing',
    List = 'list',
    Literal = 'literal',
    Paragraph = 'paragraph',
    Section = 'section',
    Subsection = 'subsection',
    Subsubsection = 'subsubsection',
    Emphasis = 'emph',
    Italic = 'italic',
    Dots = 'dots',
    Citation = 'citation',
    Code = 'code',
    Figure = 'figure',
    Caption = 'caption',
    Label = 'label',
    Include = 'include',
    Root = 'root',
    Quote = 'quote',
    Footnote = 'footnote',
    Reference = 'reference',
    URL = 'url',
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

export interface IAuthor {
    family: string,
    given: string,
    literal: string,
}

export interface IDateParts {
    'date-parts': Array<Array<number>>,
}

export interface ICitation {
    id: string,
    author: Array<IAuthor>,
    contributor: Array<IAuthor>,
    editor: Array<IAuthor>,
    issued: IDateParts,
    title: string,
    publisher: string,
    visible: boolean,
}

export interface IFootnote {
    value: string,
    children: Array<INode>,
    visible: boolean,
}