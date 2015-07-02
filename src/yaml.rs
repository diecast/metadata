use typemap;
use yaml_::{Yaml, YamlLoader};

use diecast::{self, Item};

/// Metadata key type.
///
/// This is a simple key type for keying
/// into the item extensions and accessing
/// or modifying the parsed YAML metadata
/// for a given item.

pub struct Metadata;

impl typemap::Key for Metadata {
    type Value = Yaml;
}

/// Parse YAML metadata into Metadata field
///
/// This puts the parsed `Yaml` in the `Metadata`
/// extension field and then removes the metadata
/// from the item body

pub fn parse(item: &mut Item) -> diecast::Result<()> {
    let body = {
        let (meta, body) = super::split(&item.body);

        if !meta.is_empty() {
            let mut parsed =
                try!(YamlLoader::load_from_str(meta));

            let document = parsed.swap_remove(0);

            item.extensions.insert::<Metadata>(document);

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

    use super::{Metadata, parse};

    #[test]
    fn test_parse() {
        let mut item = Item::reading("tests/yaml/input.md");

        item::read(&mut item).unwrap();

        parse(&mut item).unwrap();

        assert_eq!(item.body, include_str!("../tests/split-output.md"));

        let meta = item.extensions.get::<Metadata>();

        assert!(meta.is_some());

        let meta = meta.unwrap();

        let name = meta["name"].as_str();

        assert!(name.is_some());

        let name = name.unwrap();

        assert!(name == "testing");
    }
}
