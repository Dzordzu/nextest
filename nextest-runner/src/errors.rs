// Copyright (c) The nextest Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Errors produced by nextest.

use crate::{
    reporter::{StatusLevel, TestOutputDisplay},
    test_filter::RunIgnored,
};
use camino::Utf8PathBuf;
use config::ConfigError;
use std::{borrow::Cow, error, fmt};

/// An error that occurred while parsing the config.
#[derive(Debug)]
#[non_exhaustive]
pub struct ConfigParseError {
    config_file: Utf8PathBuf,
    err: ConfigError,
}

impl ConfigParseError {
    pub(crate) fn new(config_file: impl Into<Utf8PathBuf>, err: ConfigError) -> Self {
        Self {
            config_file: config_file.into(),
            err,
        }
    }
}

impl fmt::Display for ConfigParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "failed to parse nextest config at `{}`",
            self.config_file
        )?;
        Ok(())
    }
}

impl error::Error for ConfigParseError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(&self.err)
    }
}

/// An error which indicates that a profile was requested but not known to nextest.
#[derive(Clone, Debug)]
pub struct ProfileNotFound {
    profile: String,
    all_profiles: Vec<String>,
}

impl ProfileNotFound {
    pub(crate) fn new(
        profile: impl Into<String>,
        all_profiles: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        let mut all_profiles: Vec<_> = all_profiles.into_iter().map(|s| s.into()).collect();
        all_profiles.sort_unstable();
        Self {
            profile: profile.into(),
            all_profiles,
        }
    }
}

impl fmt::Display for ProfileNotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "profile '{}' not found (known profiles: {})",
            self.profile,
            self.all_profiles.join(", ")
        )
    }
}

impl error::Error for ProfileNotFound {}

/// Error returned while parsing a [`TestOutputDisplay`] value from a string.
#[derive(Clone, Debug)]
pub struct TestOutputDisplayParseError {
    input: String,
}

impl TestOutputDisplayParseError {
    pub(crate) fn new(input: impl Into<String>) -> Self {
        Self {
            input: input.into(),
        }
    }
}

impl fmt::Display for TestOutputDisplayParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "unrecognized value for test output display: {}\n(known values: {})",
            self.input,
            TestOutputDisplay::variants().join(", ")
        )
    }
}

impl error::Error for TestOutputDisplayParseError {}

/// Error returned while parsing a [`StatusLevel`] value from a string.
#[derive(Clone, Debug)]
pub struct StatusLevelParseError {
    input: String,
}

impl StatusLevelParseError {
    pub(crate) fn new(input: impl Into<String>) -> Self {
        Self {
            input: input.into(),
        }
    }
}

impl fmt::Display for StatusLevelParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "unrecognized value for status-level: {}\n(known values: {})",
            self.input,
            StatusLevel::variants().join(", ")
        )
    }
}

impl error::Error for StatusLevelParseError {}

/// An error that occurs while parsing a [`RunIgnored`] value from a string.
#[derive(Clone, Debug)]
pub struct RunIgnoredParseError {
    input: String,
}

impl RunIgnoredParseError {
    pub(crate) fn new(input: impl Into<String>) -> Self {
        Self {
            input: input.into(),
        }
    }
}

impl fmt::Display for RunIgnoredParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "unrecognized value for run-ignored: {}\n(known values: {})",
            self.input,
            RunIgnored::variants().join(", ")
        )
    }
}

impl error::Error for RunIgnoredParseError {}

/// An error that occurs while parsing a
/// [`PartitionerBuilder`](crate::partition::PartitionerBuilder) input.
#[derive(Clone, Debug)]
pub struct PartitionerBuilderParseError {
    expected_format: Option<&'static str>,
    message: Cow<'static, str>,
}

impl PartitionerBuilderParseError {
    pub(crate) fn new(
        expected_format: Option<&'static str>,
        message: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self {
            expected_format,
            message: message.into(),
        }
    }
}

impl fmt::Display for PartitionerBuilderParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.expected_format {
            Some(format) => {
                write!(
                    f,
                    "partition must be in the format \"{}\":\n{}",
                    format, self.message
                )
            }
            None => write!(f, "{}", self.message),
        }
    }
}

impl error::Error for PartitionerBuilderParseError {}

/// An error that occurs in [`RustTestArtifact::from_messages`](crate::test_list::RustTestArtifact::from_messages).
#[derive(Debug)]
#[non_exhaustive]
pub enum FromMessagesError {
    /// An error occurred while reading Cargo's JSON messages.
    ReadMessages(std::io::Error),

    /// An error occurred while querying the package graph.
    PackageGraph(guppy::Error),
}

impl fmt::Display for FromMessagesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FromMessagesError::ReadMessages(_) => {
                write!(f, "error reading Cargo JSON messages")
            }
            FromMessagesError::PackageGraph(_) => {
                write!(f, "error querying package graph")
            }
        }
    }
}

impl error::Error for FromMessagesError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            FromMessagesError::ReadMessages(error) => Some(error),
            FromMessagesError::PackageGraph(error) => Some(error),
        }
    }
}

/// An error that occurs while parsing test list output.
#[derive(Debug)]
#[non_exhaustive]
pub enum ParseTestListError {
    /// Running a command to gather the list of tests failed.
    Command {
        /// The command that was run.
        command: Cow<'static, str>,

        /// The underlying error.
        error: std::io::Error,
    },

    /// An error occurred while parsing a line in the test output.
    ParseLine {
        /// A descriptive message.
        message: Cow<'static, str>,

        /// The full output.
        full_output: String,
    },
}

impl ParseTestListError {
    pub(crate) fn command(command: impl Into<Cow<'static, str>>, error: std::io::Error) -> Self {
        ParseTestListError::Command {
            command: command.into(),
            error,
        }
    }

    pub(crate) fn parse_line(
        message: impl Into<Cow<'static, str>>,
        full_output: impl Into<String>,
    ) -> Self {
        ParseTestListError::ParseLine {
            message: message.into(),
            full_output: full_output.into(),
        }
    }
}

impl fmt::Display for ParseTestListError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseTestListError::Command { command, .. } => {
                write!(f, "running '{}' failed", command)
            }
            ParseTestListError::ParseLine {
                message,
                full_output,
            } => {
                write!(f, "{}\nfull output:\n{}", message, full_output)
            }
        }
    }
}

impl error::Error for ParseTestListError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            ParseTestListError::Command { error, .. } => Some(error),
            ParseTestListError::ParseLine { .. } => None,
        }
    }
}

/// An error that occurs while writing list output.
#[derive(Debug)]
#[non_exhaustive]
pub enum WriteTestListError {
    /// An error occurred while writing the list to the provided output.
    Io(std::io::Error),

    /// An error occurred while serializing JSON, or while writing it to the provided output.
    Json(serde_json::Error),
}

impl fmt::Display for WriteTestListError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WriteTestListError::Io(_) => {
                write!(f, "error writing to output")
            }
            WriteTestListError::Json(_) => {
                write!(f, "error serializing to JSON")
            }
        }
    }
}

impl error::Error for WriteTestListError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            WriteTestListError::Io(error) => Some(error),
            WriteTestListError::Json(error) => Some(error),
        }
    }
}

/// An error that occurs while writing an event.
#[derive(Debug)]
#[non_exhaustive]
pub enum WriteEventError {
    /// An error occurred while writing the event to the provided output.
    Io(std::io::Error),

    /// An error occurred while operating on the file system.
    Fs {
        /// The file being operated on.
        file: Utf8PathBuf,

        /// The underlying IO error.
        error: std::io::Error,
    },

    /// An error occurred while producing JUnit XML.
    Junit {
        /// The output file.
        file: Utf8PathBuf,

        /// The underlying error.
        error: JunitError,
    },
}

impl fmt::Display for WriteEventError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WriteEventError::Io(_) => {
                write!(f, "error writing to output")
            }
            WriteEventError::Fs { file, .. } => {
                write!(f, "error operating on path {}", file)
            }
            WriteEventError::Junit { file, .. } => {
                write!(f, "error writing JUnit output to {}", file)
            }
        }
    }
}

impl error::Error for WriteEventError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            WriteEventError::Io(error) => Some(error),
            WriteEventError::Fs { error, .. } => Some(error),
            WriteEventError::Junit { error, .. } => Some(error),
        }
    }
}

/// An error that occurred while producing JUnit XML.
#[derive(Debug)]
pub struct JunitError {
    err: quick_junit::Error,
}

impl JunitError {
    pub(crate) fn new(err: quick_junit::Error) -> Self {
        Self { err }
    }
}

impl fmt::Display for JunitError {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

impl error::Error for JunitError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(&self.err)
    }
}

/// An error occurred determining the target runner
#[derive(Debug)]
pub enum TargetRunnerError {
    /// Failed to determine the host triple, which is needed to determine the
    /// default target triple when a target is not explicitly specified
    UnknownHostPlatform(target_spec::Error),
    /// An environment variable contained non-utf8 content
    InvalidEnvironmentVar(String),
    /// An environment variable or config key was found that matches the target
    /// triple, but it didn't actually contain a binary
    BinaryNotSpecified {
        /// The environment variable or config key path
        key: String,
        /// The value that was read from the key
        value: String,
    },
    /// Failed to retrieve a directory
    UnableToReadDir(std::io::Error),
    /// Failed to canonicalize a path
    FailedPathCanonicalization {
        /// The path that failed to canonicalize
        path: Utf8PathBuf,
        /// The error the occurred during canonicalization
        error: std::io::Error,
    },
    /// A path was non-utf8
    NonUtf8Path(std::path::PathBuf),
    /// Failed to read config file
    FailedToReadConfig {
        /// The path of the config file
        path: Utf8PathBuf,
        /// The error that occurred trying to read the config file
        error: std::io::Error,
    },
    /// Failed to deserialize config file
    FailedToParseConfig {
        /// The path of the config file
        path: Utf8PathBuf,
        /// The error that occurred trying to deserialize the config file
        error: toml::de::Error,
    },
    /// Failed to parse the specified target triple
    FailedToParseTargetTriple {
        /// The triple that failed to parse
        triple: String,
        /// The error that occurred parsing the triple
        error: target_spec::errors::TripleParseError,
    },
}

impl fmt::Display for TargetRunnerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownHostPlatform(error) => {
                write!(f, "unable to determine host triple: {}", error)
            }
            Self::InvalidEnvironmentVar(key) => {
                write!(f, "environment variable '{}' contained non-utf8 data", key)
            }
            Self::BinaryNotSpecified { key, value } => {
                write!(
                    f,
                    "runner '{}' = '{}' did not contain a runner binary",
                    key, value
                )
            }
            Self::UnableToReadDir(io) => {
                write!(f, "unable to read directory: {}", io)
            }
            Self::FailedPathCanonicalization { path, error } => {
                write!(f, "failed to canonicalize path '{}': {}", path, error)
            }
            Self::NonUtf8Path(path) => {
                write!(f, "path '{}' is non-utf8", path.display())
            }
            Self::FailedToReadConfig { path, error } => {
                write!(f, "failed to read '{}': {}", path, error)
            }
            Self::FailedToParseConfig { path, error } => {
                write!(f, "failed to parse config '{}': {}", path, error)
            }
            Self::FailedToParseTargetTriple { triple, error } => {
                write!(f, "failed to parse triple '{}': {}", triple, error)
            }
        }
    }
}

impl error::Error for TargetRunnerError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::UnknownHostPlatform(error) => Some(error),
            Self::UnableToReadDir(io) => Some(io),
            Self::FailedPathCanonicalization { error, .. } => Some(error),
            Self::FailedToReadConfig { error, .. } => Some(error),
            Self::FailedToParseConfig { error, .. } => Some(error),
            Self::FailedToParseTargetTriple { error, .. } => Some(error),
            _ => None,
        }
    }
}
