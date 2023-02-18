use super::Tag;

#[derive(Copy, Clone)]
pub enum Environment {
    Abstract,
    Aligned,
    Center,
    Document,
    Enumerate,
    Equation,
    Figure,
    Itemize,
    Listing,
    Minted,
    Paragraph,
    Quote,
    Root,
}

impl Tag for Environment {
    fn value(&self) -> &str {
        match *self {
            Environment::Abstract => "abstract",
            Environment::Aligned => "center",
            Environment::Center => "center",
            Environment::Document => "document",
            Environment::Enumerate => "list",
            Environment::Equation => "listing",
            Environment::Figure => "figure",
            Environment::Itemize => "list",
            Environment::Listing => "listing",
            Environment::Minted => "code",
            Environment::Paragraph => "paragraph",
            Environment::Quote => "quote",
            Environment::Root => "root",
        }
    }
}

pub fn parse_name(_name: &str) -> Option<Environment> {
    match _name {
        "abstract" => Some(Environment::Abstract),
        "aligned" => Some(Environment::Aligned),
        "center" => Some(Environment::Center),
        "document" => Some(Environment::Document),
        "enumerate" => Some(Environment::Enumerate),
        "equation" => Some(Environment::Equation),
        "figure" => Some(Environment::Figure),
        "itemize" => Some(Environment::Itemize),
        "listing" => Some(Environment::Listing),
        "minted" => Some(Environment::Minted),
        "quote" => Some(Environment::Quote),
        _ => None,
    }
}
