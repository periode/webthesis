export enum NodeType {
    Chapter = 'chapter',
    Listing = 'listing',
    Literal = 'literal',
    Paragraph = 'paragraph',
}

export interface INode {
    tag: String;
    value: String;
    children: Array<INode> | null;
};