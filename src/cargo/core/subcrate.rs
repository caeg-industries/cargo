pub(crate) const SUBCRATE_DELIMETER: &str = "/";
pub(crate) const MAX_SUBCRATE_DEPTH: Option<usize> = Some(1);
/// This is used in contexts where the full subcrate name needs to be a valid filename, like the crate tarball.
pub(crate) const SUBCRATE_DELIMETER_FILENAME_REPLACEMENT: &str = "~";
/// This is used in contexts where the full subcrate name needs to be referenced from Rust code.
pub(crate) const SUBCRATE_DELIMETER_RUST_CODE_REPLACEMENT: &str = "_";
