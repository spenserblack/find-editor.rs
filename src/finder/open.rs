//! Utilities for opening an editor.
use super::Finder;
use crate::Error;
use std::ffi::OsString;
use std::path::Path;
use std::process::Command;

impl Finder {
    /// Opens an editor to edit `file`. Set `wait` to `true` to make this function wait
    /// until the editor is closed before returning.
    ///
    /// _When in doubt, you **should** set `wait` to `true`._
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use find_editor::Finder;
    ///
    /// let finder = Finder::new();
    /// finder.open_editor("config.toml", true).expect("Should be able to edit the file");
    /// ```
    pub fn open_editor<P>(&self, file: P, wait: bool) -> Result<(), Error>
    where
        P: AsRef<Path>,
    {
        let file = file.as_ref();
        let (editor, args) = self.which_editor()?;
        let mut args = args.into_iter().map(OsString::from).collect::<Vec<_>>();
        args.push(file.into());
        let mut child = Command::new(editor).args(args).spawn().map_err(Error::Io)?;
        if wait {
            child.wait().map_err(Error::Io)?;
        }
        Ok(())
    }
}
