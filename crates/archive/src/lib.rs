/// Handle `.tar`, `.tar.gz`, and `.tar.xz` files.
#[cfg(feature = "tar")]
pub mod tar;

/// Handle `.zip` files.
#[cfg(feature = "zip")]
pub mod zip;

mod archive;
mod archive_error;
mod tree_differ;

pub use archive::*;
pub use archive_error::*;
pub use tree_differ::*;

// Use native path utils to join the paths, so we can ensure
// the parts are joined correctly within the archive!
pub fn join_file_name<I, V>(parts: I) -> String
where
    I: IntoIterator<Item = V>,
    V: AsRef<str>,
{
    parts
        .into_iter()
        .filter_map(|p| {
            let p = p.as_ref();

            if p.is_empty() {
                None
            } else {
                Some(p.to_owned())
            }
        })
        .collect::<Vec<_>>()
        .join("/")
}