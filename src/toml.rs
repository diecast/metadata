use std::error::Error as StdError;
use std::fmt;

use typemap;
use toml_;

use diecast::{self, Item};

/// Metadata key type.
///
/// This is a simple key type for keying
/// into the item extensions and accessing
/// or modifying the parsed TOML metadata
/// for a given item.

pub struct Metadata;

impl typemap::Key for Metadata {
    type Value = toml_::Value;
}

/// TOML parsing error.
///
/// The toml crate defines the associated `Err` type as `Vec<ParserError>`,
/// which itself doesn't implement `Error`, preventing the use of `try!` or
/// other propagation means when parsing the toml.
///
/// This type simply wraps this value and implements `Error`, delegating
/// to the actual errors as needed.

#[derive(Debug)]
pub struct Error {
    errors: Vec<toml_::ParserError>,
}

impl StdError for Error {
    fn description(&self) -> &str {
        "TOML parsing error"
    }
}

impl From<Vec<toml_::ParserError>> for Error {
    fn from(errors: Vec<toml_::ParserError>) -> Error {
        Error { errors: errors }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for error in &self.errors {
            try!(writeln!(f, "{}", error))
        }

        Ok(())
    }
}

/// Parse TOML metadata into Metadata field
///
/// This puts the parsed `Toml` in the `Metadata`
/// extension field and then removes the metadata
/// from the item body

pub fn parse(item: &mut Item) -> diecast::Result<()> {
    let body = {
        let (meta, body) = super::split(&item.body);

        if !meta.is_empty() {
            let parsed = try!(meta.parse().map_err(Error::from));

            item.extensions.insert::<Metadata>(parsed);

            Some(String::from(body))
        } else {
            None
        }
    };

    body.map(|b| item.body = b);

    Ok(())
}

#[cfg(test)]
mod test {
    use toml_;

    use diecast::Item;
    use diecast::util::handle::item;

    use super::{Metadata, parse};

    #[test]
    fn test_parse() {
        let mut item = Item::reading("tests/toml/input.md");

        item::read(&mut item).unwrap();

        parse(&mut item).unwrap();

        assert_eq!(item.body, include_str!("../tests/split-output.md"));

        let meta = item.extensions.get::<Metadata>();

        assert!(meta.is_some());

        let meta = meta.unwrap();

        let name = meta.lookup("name").and_then(toml_::Value::as_str);

        assert!(name.is_some());

        let name = name.unwrap();

        assert!(name == "testing");
    }
}
