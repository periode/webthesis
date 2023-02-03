pub enum Command {
    Baselineskip,
    Caption,
    Centerline,
    Chapter,
    Citation,
    Dots,
    Emph,
    Footnote,
    InlineListing,
    Italic,
    Linespread,
    Linewidth,
    Label,
    Pagebreak,
    Reference,
    Rule,
    Section,
    Subsection,
    Subsubsection,
    URL,
    VSpace,
}

impl Command {
    pub fn value(&self) -> &str {
        match *self {
            Command::Baselineskip => "baselineskip",
            Command::Caption => "caption",
            Command::Centerline => "centerline",
            Command::Citation => "citation",
            Command::Chapter => "chapter",
            Command::Dots => "dots",
            Command::Emph => "emph",
            Command::Footnote => "footnote",
            Command::InlineListing => "inline_listing",
            Command::Italic => "italic",
            Command::Label => "label",
            Command::Linespread => "linespread",
            Command::Linewidth => "linewidth",
            Command::Pagebreak => "pagebreak",
            Command::Reference => "reference",
            Command::Rule => "rule",
            Command::Section => "section",
            Command::Subsection => "subsection",
            Command::Subsubsection => "subsubsection",
            Command::VSpace => "vspace",
            Command::URL => "url",
        }
    }

    pub fn is_print_layout(&self) -> bool {
        match *self {
            Command::Linespread => true,
            Command::VSpace => true,
            Command::Centerline => true,
            Command::Pagebreak => true,
            Command::Rule => true,
            Command::Linewidth => true,
            Command::Baselineskip => true,
            _ => false,
        }
    }
}

pub fn parse_cmd_name(_name: &str) -> Option<Command> {
    match _name {
        "baselineskip" => Some(Command::Baselineskip),
        "caption" => Some(Command::Caption),
        "centerline" => Some(Command::Centerline),
        "chapter" => Some(Command::Chapter),
        "citep" => Some(Command::Citation),
        "dots" => Some(Command::Dots),
        "emph" => Some(Command::Emph),
        "footnote" => Some(Command::Footnote),
        "label" => Some(Command::Label),
        "lstinline" => Some(Command::InlineListing),
        "linespread" => Some(Command::Linespread),
        "linewidth" => Some(Command::Linewidth),
        "pagebreak" => Some(Command::Pagebreak),
        "ref" => Some(Command::Reference),
        "rule" => Some(Command::Rule),
        "section" => Some(Command::Section),
        "subsection" => Some(Command::Subsection),
        "subsubsection" => Some(Command::Subsubsection),
        "textit" => Some(Command::Italic),
        "vspace" => Some(Command::VSpace),
        "url" => Some(Command::URL),
        _ => None,
    }
}