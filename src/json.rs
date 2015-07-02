use typemap;
use serde::json;

use diecast::{self, Item};

/// Metadata key type.
///
/// This is a simple key type for keying
/// into the item extensions and accessing
/// or modifying the parsed JSON metadata
/// for a given item.

pub struct Metadata;

impl typemap::Key for Metadata {
    type Value = json::Value;
}

/// Parse JSON metadata into Metadata field
///
/// This puts the parsed `Json` in the `Metadata`
/// extension field and then removes the metadata
/// from the item body

pub fn parse(item: &mut Item) -> diecast::Result<()> {
    let body = {
        let (meta, body) = super::split(&item.body);

        if !meta.is_empty() {
            let parsed: json::Value = try!(json::from_str(meta));

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
    use diecast::Item;
    use diecast::util::handle::item;

    use serde::json;

    use super::{Metadata, parse};

    #[test]
    fn test_parse() {
        let mut item = Item::reading("tests/json/input.md");

        item::read(&mut item).unwrap();

        parse(&mut item).unwrap();

        assert_eq!(item.body, include_str!("../tests/split-output.md"));

        let meta = item.extensions.get::<Metadata>();

        assert!(meta.is_some());

        let meta = meta.unwrap();

        let name = meta.lookup("name").and_then(json::Value::as_string);

        assert!(name.is_some());

        let name = name.unwrap();

        assert!(name == "testing");
    }
}
