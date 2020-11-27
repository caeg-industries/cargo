//! Helpers for validating and checking names like package and crate names.

use crate::core::subcrate::{MAX_SUBCRATE_DEPTH, SUBCRATE_DELIMETER};
use crate::util::CargoResult;
use anyhow::bail;
use std::path::Path;

/// Returns `true` if the name contains non-ASCII characters.
pub fn is_non_ascii_name(name: &str) -> bool {
    name.chars().any(|ch| ch > '\x7f')
}

/// A Rust keyword.
pub fn is_keyword(name: &str) -> bool {
    // See https://doc.rust-lang.org/reference/keywords.html
    [
        "Self", "abstract", "as", "async", "await", "become", "box", "break", "const", "continue",
        "crate", "do", "dyn", "else", "enum", "extern", "false", "final", "fn", "for", "if",
        "impl", "in", "let", "loop", "macro", "match", "mod", "move", "mut", "override", "priv",
        "pub", "ref", "return", "self", "static", "struct", "super", "trait", "true", "try",
        "type", "typeof", "unsafe", "unsized", "use", "virtual", "where", "while", "yield",
    ]
    .contains(&name)
}

/// These names cannot be used on Windows, even with an extension.
pub fn is_windows_reserved(name: &str) -> bool {
    [
        "con", "prn", "aux", "nul", "com1", "com2", "com3", "com4", "com5", "com6", "com7", "com8",
        "com9", "lpt1", "lpt2", "lpt3", "lpt4", "lpt5", "lpt6", "lpt7", "lpt8", "lpt9",
    ]
    .contains(&name.to_ascii_lowercase().as_str())
}

/// An artifact with this name will conflict with one of Cargo's build directories.
pub fn is_conflicting_artifact_name(name: &str) -> bool {
    ["deps", "examples", "build", "incremental"].contains(&name)
}

/// Check the base requirements for a package name.
///
/// This can be used for other things than package names, to enforce some
/// level of sanity. Note that package names have other restrictions
/// elsewhere. `cargo new` has a few restrictions, such as checking for
/// reserved names. crates.io has even more restrictions.
pub fn validate_package_name(name: &str, what: &str, help: &str) -> CargoResult<()> {
    let mut chars = name.chars();
    if let Some(ch) = chars.next() {
        if ch.is_digit(10) {
            // A specific error for a potentially common case.
            bail!(
                "the name `{}` cannot be used as a {}, \
                the name cannot start with a digit{}",
                name,
                what,
                help
            );
        }
        if !(unicode_xid::UnicodeXID::is_xid_start(ch) || ch == '_') {
            bail!(
                "invalid character `{}` in {}: `{}`, \
                the first character must be a Unicode XID start character \
                (most letters or `_`){}",
                ch,
                what,
                name,
                help
            );
        }
    }

    // It's hard to check for subcrate delimeter characters because it
    // might actually be a sequence of characters! However, we can
    // strip out the delimeter and see if the characters we're left
    // with are all our other valid characters.
    let name_without_subcrate_delim = if let Some(max_depth) = MAX_SUBCRATE_DEPTH {
        let name_without_subcrate_delim = name.replacen(SUBCRATE_DELIMETER, "", max_depth);
        if name_without_subcrate_delim.contains(SUBCRATE_DELIMETER) {
            bail!(
                "the name `{}` cannot be used as a {}, \
                 crates can be namespaced at most {} levels deep{}",
                name,
                what,
                max_depth,
                help
            );
        }
        name_without_subcrate_delim
    } else {
        name.replace(SUBCRATE_DELIMETER, "")
    };

    for ch in name_without_subcrate_delim.chars() {
        if !(unicode_xid::UnicodeXID::is_xid_continue(ch) || ch == '-') {
            bail!(
                "invalid character `{}` in {}: `{}`, \
                characters must be Unicode XID characters \
                (numbers, `-`, `_`, '{}', or most letters){}",
                ch,
                what,
                name,
                SUBCRATE_DELIMETER,
                help
            );
        }
    }
    Ok(())
}

/// Check the entire path for names reserved in Windows.
pub fn is_windows_reserved_path(path: &Path) -> bool {
    path.iter()
        .filter_map(|component| component.to_str())
        .any(|component| {
            let stem = component.split('.').next().unwrap();
            is_windows_reserved(stem)
        })
}

/// Returns `true` if the name contains any glob pattern wildcards.
pub fn is_glob_pattern<T: AsRef<str>>(name: T) -> bool {
    name.as_ref().contains(&['*', '?', '[', ']'][..])
}
