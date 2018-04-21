extern crate diecast;
extern crate typemap;
extern crate regex;

#[cfg(feature = "toml")]
extern crate toml as toml_;

#[cfg(feature = "yaml")]
extern crate yaml_rust as yaml_;

#[cfg(feature = "json")]
extern crate serde as serde;

use regex::Regex;

#[cfg(feature = "toml")]
pub mod toml;

#[cfg(feature = "yaml")]
pub mod yaml;

#[cfg(feature = "json")]
pub mod json;

/// Splits the body into a metadata and body pair
///
/// The body must begin with the opening metadata delimiter,
/// `---`. The metadata block is terminated by the same delimiter
/// on its own line. Everything after this block is considered
/// the actual content's body.
///
/// If there is no metadata or no match, then the metadata
/// slice will be empty and the body slice will consist of
/// the entire content.

pub fn split(content: &str) -> (&str, &str) {
    let re = Regex::new(
        r"(?mxs)
        \A                 # content must start with delimiter
        ---\s*             # opening
        ^(?P<metadata>.*?) # metadata
        ^---\s*            # ending")
        .unwrap();

    re.captures(&content)
    .and_then(|caps|
        caps.name("metadata").map(|meta| {
            // unwrap is safe because pos(0) is the whole capture,
            // and we're only here because there was indeed a capture
            let frontmatter_end = caps.get(0).unwrap().end();

            // indexing is safe because regex guarantees that the byte
            // indices it provides fall on character boundaries
            let body = &content[frontmatter_end ..];

            (meta.as_str(), body)
        })
    )
    .unwrap_or((&content[0 .. 0], content))
}

#[cfg(test)]
mod test {
    use super::split;

    #[test]
    fn test_split() {
        // meta and body
        let s1 = split("---\nname = 'test'\nlist = [1, 2, 3]\nage = 90\n---\
                       \nmultiline\nbody");

        assert!(!s1.0.is_empty());

        assert_eq!("name = 'test'\nlist = [1, 2, 3]\nage = 90\n", s1.0);
        assert_eq!("multiline\nbody", s1.1);

        // prejunk
        let s2 = split("junk ---\nname = 'test'\nlist = [1, 2, 3]\nage = 90\n---\
                       \nmultiline\nbody");

        assert!(s2.0.is_empty());

        // no body
        let s3 = split("---\nname = 'test'\nlist = [1, 2, 3]\nage = 90\n---");

        assert!(!s3.0.is_empty());

        assert_eq!("name = 'test'\nlist = [1, 2, 3]\nage = 90\n", s3.0);
        assert_eq!("", s3.1);

        // no meta
        let s4 = split("multiline\nbody");

        assert!(s4.0.is_empty());
    }
}
