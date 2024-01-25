#[allow(unused_imports)]
use crate::prelude::*;

/// The scope from/to which a configuration value is read/written.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    all(not(test), feature = "serde"),
    derive(serde::Serialize, serde::Deserialize)
)]
pub enum ConfigScope {
    // NOTE(qix-): We disable all but `Local` when testing.
    // NOTE(qix-): This is not a standard practice, and you shouldn't
    // NOTE(qix-): do this in almost any other case. However, we do
    // NOTE(qix-): this here because most backends for Git do not have
    // NOTE(qix-): a way to override global/system/etc config locations,
    // NOTE(qix-): and we don't want to accidentally modify the user's
    // NOTE(qix-): global config when running tests or have them influence
    // NOTE(qix-): the tests in any way. Thus, we force test writers to use
    // NOTE(qix-): `Local` scope when testing. This is not ideal, but it's
    // NOTE(qix-): the best we can do for now. Sorry for the mess.
    /// Pull from the most appropriate scope.
    /// This is the default, and will fall back to a higher
    /// scope if the value is not initially found.
    #[cfg(not(test))]
    #[cfg_attr(not(test), default)]
    Auto = 0,
    /// Pull from the local scope (`.git/config`) _only_.
    #[cfg_attr(test, default)]
    Local = 1,
    /// Pull from the system-wide scope (`${prefix}/etc/gitconfig`) _only_.
    #[cfg(not(test))]
    System = 2,
    /// Pull from the global (user) scope (typically `~/.gitconfig`) _only_.
    #[cfg(not(test))]
    Global = 3,
}

/// A handle to an open Git repository.
pub trait Repository {
    /// The type of error returned by this repository.
    type Error: core::error::Error + core::fmt::Debug + Send + Sync + 'static;

    /// Reads a configuration value.
    ///
    /// Errors if the value is not valid UTF-8.
    async fn config_get(
        &self,
        key: &str,
        scope: ConfigScope,
    ) -> Result<Option<String>, Self::Error>;

    /// Writes a configuration value.
    ///
    /// Errors if the new value is not valid UTF-8.
    async fn config_set(
        &self,
        key: &str,
        value: &str,
        scope: ConfigScope,
    ) -> Result<(), Self::Error>;
}

/// Provides authentication credentials when performing
/// an operation that interacts with a remote.
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Authorization {
    /// Performs no attempt to authorize; uses the system's
    /// default authorization mechanism, if any.
    #[default]
    Auto,
    /// Performs HTTP(S) Basic authentication with a username
    /// and password.
    ///
    /// Note that certain remotes may use this mechanism
    /// for passing tokens as well; consult the respective
    /// remote's documentation for what information to supply.
    Basic {
        /// The username to use for authentication.
        username: String,
        /// The password to use for authentication.
        password: String,
    },
    /// Specifies a set of credentials for logging in with SSH.
    Ssh {
        /// The path to the SSH private key to use for authentication.
        /// If `None`, the default SSH key will be used (i.e. `-i` will not
        /// be passed to `ssh`).
        private_key: Option<String>,
        /// The passphrase to use for the SSH private key.
        /// If `None`, the key is assumed to be unencrypted.
        /// A prompt for a passphrase will result in an error.
        passphrase: Option<String>,
    },
}
