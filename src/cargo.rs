//! A module dealing with the cargo interaction.
use crate::error::Error;
use std::env;
use std::fs;
use std::iter;
use std::path::{Path, PathBuf};

/// Returns the path to the cargo project root.
///
/// # Errors
/// This function returns [`NotACrate`][not_a_crate], if no file `Cargo.toml`
/// can be found in the current directory or any of its parent directories.
///
/// [not_a_crate]: ../error/enum.Error.html#variant.NotACrate
pub fn root() -> Result<PathBuf, Error> {
    let cwd = env::current_dir()?;
    cwd.ancestors()
        .chain(iter::once(cwd.as_path()))
        .find(|directory| contains_manifest(directory))
        .map(|directory| directory.to_path_buf())
        .ok_or(Error::NotACrate)
}

/// Query, if `directory` contains a `Cargo.toml` file.
///
/// This function returns `false` on an I/O error.
fn contains_manifest(directory: &Path) -> bool {
    fs::read_dir(directory)
        .map(|entries| {
            entries
                .filter_map(|entry| entry.ok())
                .any(|entry| entry.file_name() == "Cargo.toml")
        })
        .unwrap_or(false)
}

/// Query the name of the crate.
///
/// # Errors
/// Returns an `IoError` if the `Cargo.toml` could not be found or a
/// `InvalidManifest` error, if the file does not contain a name property.
pub fn crate_name() -> Result<String, Error> {
    let name_line = fs::read_to_string("Cargo.toml")?
        .lines()
        .find(|line| line.contains("name"))
        .map(|line| line.to_string())
        .ok_or(Error::InvalidManifest)?;
    Ok(name_line
        .splitn(2, "=")
        .last()
        .ok_or(Error::InvalidManifest)?
        .trim()
        .trim_matches('"')
        .trim()
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::{contains_manifest, crate_name, env, root, Error};

    #[test]
    fn crate_root_contains_manifest() {
        assert!(contains_manifest(&env::current_dir().unwrap()));
    }

    #[test]
    fn crate_subdirectory_contains_no_manifest() {
        assert!(!contains_manifest(&env::current_dir().unwrap().join("src")));
    }

    /* ignored, since tests are un in parallel, and due to global state (the
     * environment variable) this test interferes with the other ones. */
    #[test]
    #[ignore]
    fn root_returns_error_if_not_a_cargo_project() {
        env::set_current_dir("/").unwrap();
        assert_eq!(root().unwrap_err(), Error::NotACrate);
    }

    #[test]
    fn crate_name_matches() {
        assert_eq!(crate_name().unwrap(), "cargo-size");
    }
}
