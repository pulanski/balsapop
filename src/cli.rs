use clap::Parser;
use miette::Diagnostic;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Parser, Default, Debug)]
#[clap(name = "balsapopc")]
#[clap(
    // about = "Balsapop, an Ahead-of-time (AOT) Compiler for the Chimera Programming Language",
    // long_about = "Processes, compiles Chimera language source files."
)]
#[clap(bin_name = "balsapopc")]
#[clap(version)]
pub(crate) struct BalsapopCli {
    /// Optional path to the source file to compile.
    /// If no path is provided or the path doesn't exist,
    /// the compiler will return an error to the user.
    ///
    /// ## Example
    ///   Compile the Balsa source file `foo.balsa` in the current
    ///   directory:
    ///
    /// ```bash
    ///     $ balsapopc foo.balsa
    /// ```
    // #[clap(short, long)]
    pub(crate) source_path: Option<PathBuf>,

    /// Pause and wait for user input at the end of each phase in the
    /// compilation process. [default: false]
    ///
    /// This is useful for incrementally debugging the compiler as a whole
    /// or allowing a user to get a better understanding of the different
    /// phases within the compilation process.
    ///
    /// ### Default
    /// Defaults to `false`.
    #[clap(
        short = 'i',
        long,
        value_parser,
        required = false,
        default_value_t = false
    )]
    pub(crate) interactive: bool,
}

#[derive(Error, Debug, Diagnostic)]
#[error("File not found error")]
#[diagnostic(
    code(balsapop::cli::file_not_found),
    url(docsrs),
    help("Please provide a valid source file path.")
)]
pub(crate) struct FileNotFound {}
