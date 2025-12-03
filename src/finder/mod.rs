//! Contains the [`Finder`] struct which helps find editors.

#[cfg(any(feature = "open", feature = "split", feature = "which"))]
use crate::Error;
use std::env;
use std::ffi::{OsStr, OsString};
#[cfg(feature = "which")]
use std::path::PathBuf;
#[cfg(feature = "open")]
mod open;

/// Helper to find and open an editor.
///
/// Can take extra environment variable keys to define environment variables specific
/// to your tool that should be looked up first.
#[derive(Default)]
pub struct Finder {
    /// Extra environment variables to search for.
    extra_env_vars: Vec<OsString>,
}

impl Finder {
    #[cfg(windows)]
    const COMMON_EDITOR: &'static str = "notepad.exe";
    #[cfg(not(windows))]
    const COMMON_EDITOR: &'static str = "vi";
    /// Basic environment variables to look up.
    const STANDARD_ENV_VARS: [&'static str; 2] = ["VISUAL", "EDITOR"];

    /// Creates a new [`Finder`].
    #[inline]
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new [`Finder`] with a set of extra environment variables to look up.
    ///
    /// This can be useful if you're writing an executable and you would also like
    /// to use environment variables like `$MY_TOOL_EDITOR`. The extra environment
    /// variables always take priority over the defaults.
    pub fn with_extra_environment_variables<S, I>(extras: I) -> Self
    where
        S: AsRef<OsStr>,
        I: IntoIterator<Item = S>,
    {
        let extra_env_vars = extras
            .into_iter()
            .map(|s| OsString::from(s.as_ref()))
            .collect();
        Self { extra_env_vars }
    }

    /// Gets the name of an editor as a [`String`].
    ///
    /// Sometimes an editor can be multiple words (e.g. `code --wait`). Consider using
    /// [`Finder::split_editor_name`] to handle this case. Also consider using
    /// [`Finder::which_editor`] to assert that the editor exists in `$PATH`.
    #[inline]
    pub fn editor_name(&self) -> String {
        self.editor_name_inner(|key| env::var(key), Self::COMMON_EDITOR)
    }

    /// Gets the name of an editor as a [`String`].
    fn editor_name_inner<Env, E>(&self, f: Env, fallback: &'static str) -> String
    where
        Env: Copy + FnMut(&OsStr) -> Result<String, E>,
    {
        let editor = self
            .find_extra_editor_name(f)
            .or_else(|| Self::find_editor_name(f))
            .unwrap_or_else(|| String::from(fallback));
        debug_assert!(!editor.is_empty(), "An editor should always be found");
        editor
    }

    /// Finds the editor [`String`]
    fn find_editor_name<Env, E>(f: Env) -> Option<String>
    where
        Env: FnMut(&OsStr) -> Result<String, E>,
    {
        Self::STANDARD_ENV_VARS
            .into_iter()
            .map(OsStr::new)
            .map(f)
            .filter_map(Result::ok)
            .next()
    }

    /// Finds the editor [`String`] from any extra environment variable keys that were
    /// configured.
    fn find_extra_editor_name<Env, E>(&self, f: Env) -> Option<String>
    where
        Env: FnMut(&OsStr) -> Result<String, E>,
    {
        // NOTE We'll just skip if we can't read.
        self.extra_env_vars
            .iter()
            .map(|key| key.as_ref())
            .map(f)
            .filter_map(Result::ok)
            .next()
    }

    /// Gets the name of an editor as an [`OsString`].
    ///
    /// This is a lower-level utility in case you expect the editor's name to not be valid
    /// unicode.
    #[inline]
    pub fn editor_name_os(&self) -> OsString {
        self.editor_name_os_inner(|key| env::var_os(key), Self::COMMON_EDITOR)
    }

    /// Gets the name of an editor as an [`OsString`].
    ///
    /// This is a lower-level utility in case you expect the editor's name to not be valid
    /// unicode.
    fn editor_name_os_inner<Env>(&self, f: Env, fallback: &'static str) -> OsString
    where
        Env: Copy + FnMut(&OsStr) -> Option<OsString>,
    {
        let editor = self
            .find_extra_editor_name_os(f)
            .or_else(|| Self::find_editor_name_os(f))
            .unwrap_or_else(|| OsString::from(fallback));
        debug_assert!(!editor.is_empty(), "An editor should always be found");
        editor
    }

    /// Finds the editor [`OsString`] from any extra environment variable keys that were
    /// configured.
    fn find_extra_editor_name_os<Env>(&self, f: Env) -> Option<OsString>
    where
        Env: FnMut(&OsStr) -> Option<OsString>,
    {
        // NOTE We'll just skip if we can't read.
        self.extra_env_vars
            .iter()
            .map(|key| key.as_ref())
            .filter_map(f)
            .next()
    }

    /// Finds the editor [`OsString`].
    fn find_editor_name_os<Env>(f: Env) -> Option<OsString>
    where
        Env: FnMut(&OsStr) -> Option<OsString>,
    {
        Self::STANDARD_ENV_VARS
            .into_iter()
            .map(OsStr::new)
            .filter_map(f)
            .next()
    }

    /// Finds the editor with [`Finder::editor_name`], then splits the editor into its
    /// command and any arguments.
    ///
    /// This can be useful when the editor includes arguments, like `code --wait`.
    #[cfg(feature = "split")]
    pub fn split_editor_name(&self) -> Result<(String, Vec<String>), Error> {
        let editor = self.editor_name();
        let words = shell_words::split(&editor).map_err(Error::ShellWords)?;
        debug_assert!(!words.is_empty(), "There should always be at least 1 word");
        let mut words = words.into_iter();
        let editor = words.next().expect("A command name should be present");
        let args = words.collect::<Vec<_>>();
        Ok((editor, args))
    }

    /// Finds the editor's command with [`Finder::split_editor_name`], then finds editor
    /// command's path. Also returns any arguments that should be passed to the command.
    ///
    /// It does *not* search in `cwd`, even on Windows, as it is extremely unlikely that one
    /// would intentionally want to run a binary in the current directory, and can be a
    /// security issue.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use find_editor::Finder;
    ///
    /// let finder = Finder::new();
    /// let (editor_cmd, args) = finder.which_editor().expect("Should find an editor");
    /// ```
    #[cfg(feature = "which")]
    pub fn which_editor(&self) -> Result<(PathBuf, Vec<String>), Error> {
        use which::which;

        let (editor, args) = self.split_editor_name()?;
        let editor = which(editor).map_err(Error::Which)?;
        Ok((editor, args))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const FALLBACK: &str = "fallback";

    #[rstest]
    #[case::visual_defined("VISUAL", [], "foo", "foo")]
    #[case::editor_defined("EDITOR", [], "bar", "bar")]
    #[case::unknown_key_defined("--UNKNOWN--", [], "foo", FALLBACK)]
    #[case::custom_editor_defined("MY_EXTRA", ["MY_EXTRA"], "baz", "baz")]
    fn test_editor_name<Extras>(
        #[case] defined_key: &str,
        #[case] extra_keys: Extras,
        #[case] editor_name: &str,
        #[case] expected: &str,
    ) where
        Extras: IntoIterator<Item = &'static str>,
    {
        let f = |key: &OsStr| {
            (key == defined_key)
                .then_some(String::from(editor_name))
                .ok_or(())
        };
        let finder = Finder::with_extra_environment_variables(extra_keys);
        let actual = finder.editor_name_inner(f, FALLBACK);
        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case::visual_defined("VISUAL", [], "foo", "foo")]
    #[case::editor_defined("EDITOR", [], "bar", "bar")]
    #[case::unknown_key_defined("--UNKNOWN--", [], "foo", FALLBACK)]
    #[case::custom_editor_defined("MY_EXTRA", ["MY_EXTRA"], "baz", "baz")]
    fn test_editor_name_os<Extras>(
        #[case] defined_key: &str,
        #[case] extra_keys: Extras,
        #[case] editor_name: &str,
        #[case] expected: &str,
    ) where
        Extras: IntoIterator<Item = &'static str>,
    {
        let f = |key: &OsStr| (key == defined_key).then_some(OsString::from(editor_name));
        let finder = Finder::with_extra_environment_variables(extra_keys);
        let actual = finder.editor_name_os_inner(f, FALLBACK);
        assert_eq!(expected, actual);
    }
}
