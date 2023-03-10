use serde::Serialize;

use super::Tag;

#[derive(Serialize, Copy, Clone, PartialEq, Debug)]
pub enum Command {
    Affiliation,
    AtBeginEnvironment,
    Author,
    Baselineskip,
    Bibliography,
    Bibliographystyle,
    Caption,
    Captionsetup,
    Centerline,
    Chapter,
    Citation,
    Clearpage,
    Date,
    Defaultfontfeatures,
    Documentclass,
    Dots,
    Emph,
    Footnote,
    Footnotesize,
    Graphicspath,
    Hypersetup,
    Image,
    Include,
    InlineListing,
    Italic,
    Item,
    Itshape,
    Linespread,
    Linewidth,
    Label,
    Lstset,
    Maketitle,
    Newenvironment,
    Pagebreak,
    Reference,
    Rule,
    Section,
    Setmainfont,
    Setmonofont,
    Sloppy,
    Spacer,
    SpacerSmall,
    NewCommand,
    Subsection,
    Subsubsection,
    Superscript,
    Title,
    Today,
    Ttffamily,
    URL,
    Usemintedstyle,
    Usepackage,
    VSpace,
    //-- mathematics-specific
    Fraction,
    Partial,
    Mathcal,
    PartialSymbol,
    Quad,
    NablaSymbol,
    TimesSymbol,
    DotSymbol,
}

impl Tag for Command {
    fn value(&self) -> &str {
        match *self {
            Command::Affiliation => "affiliation",
            Command::AtBeginEnvironment => "AtBeginEnvironment",
            Command::Author => "author",
            Command::Baselineskip => "baselineskip",
            Command::Bibliography => "bibliography",
            Command::Bibliographystyle => "bibliographystyle",
            Command::Caption => "caption",
            Command::Captionsetup => "captionsetup",
            Command::Centerline => "centerline",
            Command::Citation => "citation",
            Command::Chapter => "chapter",
            Command::Clearpage => "clearpage",
            Command::Date => "date",
            Command::Defaultfontfeatures => "defaultfontfeatures",
            Command::Documentclass => "documentclass",
            Command::Dots => "dots",
            Command::Emph => "emph",
            Command::Footnote => "footnote",
            Command::Footnotesize => "footnotesize",
            Command::Graphicspath => "graphicspath",
            Command::Hypersetup => "hypersetup",
            Command::Image => "image",
            Command::Include => "include",
            Command::InlineListing => "inline_listing",
            Command::Italic => "italic",
            Command::Item => "item",
            Command::Itshape => "itshape",
            Command::Label => "label",
            Command::Linespread => "linespread",
            Command::Linewidth => "linewidth",
            Command::Lstset => "lstset",
            Command::Maketitle => "maketitle",
            Command::Newenvironment => "newenvironment",
            Command::Pagebreak => "pagebreak",
            Command::Reference => "reference",
            Command::Rule => "rule",
            Command::Section => "section",
            Command::Setmainfont => "setmainfont",
            Command::Setmonofont => "setmonofont",
            Command::Sloppy => "sloppy",
            Command::Spacer => "spacer",
            Command::SpacerSmall => "spacersmall",
            Command::NewCommand => "newcommand",
            Command::Subsection => "subsection",
            Command::Subsubsection => "subsubsection",
            Command::Superscript => "superscript",
            Command::Title => "title",
            Command::Today => "today",
            Command::Ttffamily => "ttfamily",
            Command::VSpace => "vspace",
            Command::URL => "url",
            Command::Usemintedstyle => "usemintedstyle",
            Command::Usepackage => "usepackage",
            //-- mathematics-specific
            Command::Fraction => "math_fraction",
            Command::Partial => "math_partial",
            Command::Mathcal => "math_mathcal",
            Command::PartialSymbol => "math_partial",
            Command::Quad => "math_quad",
            Command::NablaSymbol => "math_nabla",
            Command::TimesSymbol => "math_times",
            Command::DotSymbol => "math_dot",
        }
    }

    fn is_front(&self)  -> bool {
        match *self {
            Command::Title => true,
            Command::Author => true,
            Command::Affiliation => true,
            Command::Date => true,
            _ => false,
        }
    }
}

impl Command {
    pub fn is_semantic(&self) -> bool {
        return !self.is_print_specific()
            && !self.is_latex_specific()
            && !self.is_mathematics_specific();
    }

    pub fn is_header(&self) -> bool {
        match *self {
            Command::Chapter => true,
            Command::Section => true,
            Command::Subsection => true,
            Command::Subsubsection => true,
            _ => false,
        }
    }

    pub fn get_indent(&self) -> i8 {
        match *self {
            Command::Include => 4,
            Command::Chapter => 3,
            Command::Section => 2,
            Command::Subsection => 1,
            Command::Subsubsection => 0,
            _ => -1,
        }
    }

    fn is_mathematics_specific(&self) -> bool {
        match *self {
            Command::Fraction => true,
            Command::Partial => true,
            Command::Mathcal => true,
            Command::PartialSymbol => true,
            Command::Quad => true,
            Command::NablaSymbol => true,
            Command::TimesSymbol => true,
            Command::DotSymbol => true,
            _ => false,
        }
    }

    fn is_print_specific(&self) -> bool {
        match *self {
            Command::Baselineskip => true,
            Command::Centerline => true,
            Command::Clearpage => true,
            Command::Documentclass => true,
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
            Command::NewCommand => true,
            Command::AtBeginEnvironment => true,
            Command::Bibliographystyle => true,
            Command::Captionsetup => true,
            Command::Defaultfontfeatures => true,
            Command::Footnotesize => true,
            Command::Graphicspath => true,
            Command::Hypersetup => true,
            Command::Itshape => true,
            Command::Lstset => true,
            Command::Maketitle => true,
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
        "affil" => Some(Command::Affiliation),
        "AtBeginEnvironment" => Some(Command::AtBeginEnvironment),
        "author" => Some(Command::Author),
        "baselineskip" => Some(Command::Baselineskip),
        "bibliography" => Some(Command::Bibliography),
        "bibliographystyle" => Some(Command::Bibliographystyle),
        "caption" => Some(Command::Caption),
        "captionsetup" => Some(Command::Captionsetup),
        "centerline" => Some(Command::Centerline),
        "clearpage" => Some(Command::Clearpage),
        "chapter" => Some(Command::Chapter),
        "citep" => Some(Command::Citation),
        "date" => Some(Command::Date),
        "defaultfontfeatures" => Some(Command::Defaultfontfeatures),
        "documentclass" => Some(Command::Documentclass),
        "dots" => Some(Command::Dots),
        "emph" => Some(Command::Emph),
        "footnote" => Some(Command::Footnote),
        "footnotesize" => Some(Command::Footnotesize),
        "graphicspath" => Some(Command::Graphicspath),
        "hypersetup" => Some(Command::Hypersetup),
        "item" => Some(Command::Item),
        "itshape" => Some(Command::Itshape),
        "include" => Some(Command::Include),
        "includegraphics" => Some(Command::Image),
        "label" => Some(Command::Label),
        "lstinline" => Some(Command::InlineListing),
        "lstset" => Some(Command::Lstset),
        "linespread" => Some(Command::Linespread),
        "linewidth" => Some(Command::Linewidth),
        "maketitle" => Some(Command::Maketitle),
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
        "spacer" => Some(Command::Spacer),
        "spacersmall" => Some(Command::SpacerSmall),
        "newcommand" => Some(Command::NewCommand),
        "textit" => Some(Command::Italic),
        "title" => Some(Command::Title),
        "today" => Some(Command::Today),
        "ttfamily" => Some(Command::Ttffamily),
        "url" => Some(Command::URL),
        "usemintedstyle" => Some(Command::Usemintedstyle),
        "usepackage" => Some(Command::Usepackage),
        "vspace" => Some(Command::VSpace),
        "^" => Some(Command::Superscript),
        //-- mathematics-specific
        "frac" => Some(Command::Fraction),
        "partial" => Some(Command::Partial),
        "mathcal" => Some(Command::Mathcal),
        "quad" => Some(Command::Quad),
        "nabla" => Some(Command::NablaSymbol),
        "times" => Some(Command::TimesSymbol),
        "cdot" => Some(Command::DotSymbol),
        _ => None,
    }
}
