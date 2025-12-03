//! Find and open an editor.
//!
//! If no editor is found, this library will always fall back to `notepad.exe` on
//! Windows and `vi` on every other platform.
//!
//! Use [`Finder`] for more advanced usage.
//!
//! # Features
//!
//! - `open` - Provides [`open_editor`].
//! - `split` - Provides [`split_editor_name`], which can help with multi-word editors
//!   like `code --wait`.
//! - `which` - Provides [`which_editor`], which finds the editor on `$PATH`. Calling an
//!   executable on Windows can find and run an executable in the current directory.
//!   [`which_editor`] helps *prevent* calling an executable in the current directory.
#[cfg(any(feature = "open", feature = "split", feature = "which"))]
pub use error::Error;
pub use finder::Finder;
#[cfg(feature = "split")]
pub use shell_words::ParseError;
use std::ffi::OsString;
#[cfg(feature = "open")]
use std::path::Path;
#[cfg(feature = "which")]
use std::path::PathBuf;
#[cfg(feature = "which")]
pub use which::Error as WhichError;

#[cfg(any(feature = "open", feature = "split", feature = "which"))]
mod error;
mod finder;

/// Gets the name of an editor as a [`String`].
///
/// See [`Finder::editor_name`] for more information.
pub fn editor_name() -> String {
    Finder::new().editor_name()
}

/// Gets the name of an editor as an [`OsString`].
///
/// See [`Finder::editor_name_os`] for more information.
#[inline]
pub fn editor_name_os() -> OsString {
    Finder::new().editor_name_os()
}

/// Splits the editor into its command and any arguments.
///
/// See [`Finder::split_editor_name`] for more information.
#[cfg(feature = "split")]
pub fn split_editor_name() -> Result<(String, Vec<String>), Error> {
    Finder::new().split_editor_name()
}

/// Gets the editor's command path and any arguments that should be passed to it.
///
/// See [`Finder::which_editor`] for more information.
///
/// # Example
///
/// ```rust,no_run
/// use find_editor::which_editor;
///
/// let (editor_cmd, args) = which_editor().expect("Should find an editor");
/// ```
#[cfg(feature = "which")]
pub fn which_editor() -> Result<(PathBuf, Vec<String>), Error> {
    Finder::new().which_editor()
}

/// Opens an editor to edit `file`. Set `wait` to `true` to make this function wait
/// until the editor is closed before returning.
///
/// _When in doubt, you **should** set `wait` to `true`._
///
/// See [`Finder::open_editor`] for more information.
///
/// # Example
///
/// ```rust,no_run
/// use find_editor::open_editor;
///
/// open_editor("config.toml", true).expect("Should be able to edit the file");
/// ```
#[cfg(feature = "open")]
pub fn open_editor<P>(file: P, wait: bool) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    Finder::new().open_editor(file, wait)
}
