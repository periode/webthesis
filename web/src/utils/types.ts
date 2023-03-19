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
    Superscript = 'superscript',
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
    Spacer = 'spacer',
}

export interface INode {
    tag: string;
    value: string;
    children: Array<INode> | null;
    index?: number,
};

export interface IToCNode {
    tag: string;
    value: string;
    label: string;
    children: Array<IToCNode> | null;
    parent?: IToCNode;
    index: Array<number>;
};

export interface IListingsNode {
    index: number,
    filename: string,
    label: string,
    value: string,
    section: string,
    section_label: string,
    chapter: string,
    chapter_label: string,
}

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
    'container-title': string,
    editor: Array<IAuthor>,
    issued: IDateParts,
    title: string,
    publisher: string,
    'publisher-place': string,
    URL: string,
    type: string,
    visible: boolean,
    highlighted: boolean,
    index: number,
}

export interface IFootnote {
    value: string,
    children: Array<INode>,
    visible: boolean,
    highlighted: boolean,
}