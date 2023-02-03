pub enum Environment {
    Document,
    Itemize,
    Listing,
    Minted,
    Quote,
}

impl Environment {
    pub fn value(&self) -> &str {
        match *self {
            Environment::Document => "document",
            Environment::Itemize => "list",
            Environment::Listing => "figure",
            Environment::Minted => "code",
            Environment::Quote => "quote",
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