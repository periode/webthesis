use super::Tag;

#[derive(Copy, Clone)]
pub enum Environment {
    Center,
    Document,
    Enumerate,
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
            Environment::Center => "center",
            Environment::Document => "document",
            Environment::Enumerate => "list",
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
        "center" => Some(Environment::Center),
        "document" => Some(Environment::Document),
        "enumerate" => Some(Environment::Enumerate),
        "figure" => Some(Environment::Figure),
        "itemize" => Some(Environment::Itemize),
        "listing" => Some(Environment::Listing),
        "minted" => Some(Environment::Minted),
        "quote" => Some(Environment::Quote),
        _ => None
    }
}