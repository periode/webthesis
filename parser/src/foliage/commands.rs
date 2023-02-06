use super::Tag;

#[derive(Copy, Clone, PartialEq)]
pub enum Command {
    AtBeginEnvironment,
    Baselineskip,
    Caption,
    Captionsetup,
    Centerline,
    Chapter,
    Citation,
    Clearpage,
    Defaultfontfeatures,
    Dots,
    Emph,
    Footnote,
    Footnotesize,
    Include,
    InlineListing,
    Italic,
    Item,
    Itshape,
    Linespread,
    Linewidth,
    Label,
    Lstset,
    Newenvironment,
    Pagebreak,
    Reference,
    Rule,
    Section,
    Setmainfont,
    Setmonofont,
    Sloppy,
    Subsection,
    Subsubsection,
    Ttffamily,
    URL,
    Usemintedstyle,
    Usepackage,
    VSpace,
}

impl Tag for Command {
    fn value(&self) -> &str {
        match *self {
            Command::AtBeginEnvironment => "AtBeginEnvironment",
            Command::Baselineskip => "baselineskip",
            Command::Caption => "caption",
            Command::Captionsetup => "captionsetup",
            Command::Centerline => "centerline",
            Command::Citation => "citation",
            Command::Chapter => "chapter",
            Command::Clearpage => "clearpage",
            Command::Defaultfontfeatures => "defaultfontfeatures",
            Command::Dots => "dots",
            Command::Emph => "emph",
            Command::Footnote => "footnote",
            Command::Footnotesize => "footnotesize",
            Command::Include => "include",
            Command::InlineListing => "inline_listing",
            Command::Italic => "italic",
            Command::Item => "item",
            Command::Itshape => "itshape",
            Command::Label => "label",
            Command::Linespread => "linespread",
            Command::Linewidth => "linewidth",
            Command::Lstset => "lstset",
            Command::Newenvironment => "newenvironment",
            Command::Pagebreak => "pagebreak",
            Command::Reference => "reference",
            Command::Rule => "rule",
            Command::Section => "section",
            Command::Setmainfont => "setmainfont",
            Command::Setmonofont => "setmonofont",
            Command::Sloppy => "sloppy",
            Command::Subsection => "subsection",
            Command::Subsubsection => "subsubsection",
            Command::Ttffamily => "ttffamily",
            Command::VSpace => "vspace",
            Command::URL => "url",
            Command::Usemintedstyle => "usemintedstyle",
            Command::Usepackage => "usepackage",
        }
    }
}

impl Command {
    pub fn is_semantic(&self) -> bool {
        return !self.is_print_specific() && !self.is_latex_specific()
    }

    fn is_print_specific(&self) -> bool {
        match *self {
            Command::Baselineskip => true,
            Command::Centerline => true,
            Command::Clearpage => true,
            Command::Linespread => true,
            Command::Linewidth => true,
            Command::VSpace => true,
            Command::Pagebreak => true,
            Command::Rule => true,
            _ => false,
        }
    }

    fn is_latex_specific(&self) -> bool {
        match *self {
            Command::AtBeginEnvironment => true,
            Command::Captionsetup => true,
            Command::Defaultfontfeatures => true,
            Command::Footnotesize => true,
            Command::Itshape => true,
            Command::Lstset => true,
            Command::Newenvironment => true,
            Command::Setmainfont => true,
            Command::Setmonofont => true,
            Command::Sloppy => true,
            Command::Ttffamily => true,
            Command::Usemintedstyle => true,
            Command::Usepackage => true,
            _ => false,
        }
    }
}

pub fn parse_name(_name: &str) -> Option<Command> {
    match _name {
        "AtBeginEnvironment" => Some(Command::AtBeginEnvironment),
        "baselineskip" => Some(Command::Baselineskip),
        "caption" => Some(Command::Caption),
        "captionsetup" => Some(Command::Captionsetup),
        "centerline" => Some(Command::Centerline),
        "clearpage" => Some(Command::Clearpage),
        "chapter" => Some(Command::Chapter),
        "citep" => Some(Command::Citation),
        "defaultfontfeatures" => Some(Command::Defaultfontfeatures),
        "dots" => Some(Command::Dots),
        "emph" => Some(Command::Emph),
        "footnote" => Some(Command::Footnote),
        "footnotesize" => Some(Command::Footnotesize),
        "item" => Some(Command::Item),
        "itshape" => Some(Command::Itshape),
        "include" => Some(Command::Include),
        "label" => Some(Command::Label),
        "lstinline" => Some(Command::InlineListing),
        "lstset" => Some(Command::Lstset),
        "linespread" => Some(Command::Linespread),
        "linewidth" => Some(Command::Linewidth),
        "newenvironment" => Some(Command::Newenvironment),
        "pagebreak" => Some(Command::Pagebreak),
        "ref" => Some(Command::Reference),
        "rule" => Some(Command::Rule),
        "section" => Some(Command::Section),
        "setmainfont" => Some(Command::Setmainfont),
        "setmonofont" => Some(Command::Setmonofont),
        "subsection" => Some(Command::Subsection),
        "subsubsection" => Some(Command::Subsubsection),
        "sloppy" => Some(Command::Sloppy),
        "textit" => Some(Command::Italic),
        "ttffamily" => Some(Command::Ttffamily),
        "vspace" => Some(Command::VSpace),
        "url" => Some(Command::URL),
        "usemintedstyle" => Some(Command::Usemintedstyle),
        "usepackage" => Some(Command::Usepackage),
        _ => None,
    }
}