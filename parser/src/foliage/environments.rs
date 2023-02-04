use super::Tag;

#[derive(Copy, Clone)]
pub enum Environment {
    Document,
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
            Environment::Document => "document",
            Environment::Itemize => "list",
            Environment::Listing => "figure",
            Environment::Minted => "code",
            Environment::Paragraph => "paragraph",
            Environment::Quote => "quote",
            Environment::Root => "root",
        }
    }
}

pub fn parse_name(_name: &str) -> Option<Environment> {
    match _name {
        "document" => Some(Environment::Document),
        "itemize" => Some(Environment::Itemize),
        "listing" => Some(Environment::Listing),
        "minted" => Some(Environment::Minted),
        "quote" => Some(Environment::Quote),
        _ => None
    }
}