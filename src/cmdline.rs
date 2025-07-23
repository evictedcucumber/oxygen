use std::path::PathBuf;

use clap::Parser;

/// Represents the posssible cmdline args using the [`clap`] crate.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Contains the optional path to place the output file.
    #[arg(
        short,
        value_name = "OUT_FILE",
        help = "The output path for the compiled binary"
    )]
    pub output_file: Option<PathBuf>,

    /// `true` if tokens should be displayed, `false` otherwise.
    #[arg(
        long,
        group = "display",
        help = "Display the tokens generated from the compilation"
    )]
    pub display_tokens: bool,

    /// Contains the file path to the validated oxygen file to compile.
    #[arg(
        value_name = "O2_FILE",
        help = "The path to the oxygen file to compile",
        value_parser = validate_oxygen_file
    )]
    pub oxygen_file: PathBuf,
}

/// Validates that a given string is a oxygen filename.
fn validate_oxygen_file(s: &str) -> Result<PathBuf, String> {
    if s.ends_with(".o2") {
        Ok(PathBuf::from(s))
    } else {
        Err("must end with '.o2'".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_validate_oxygen_file() {
        assert!(validate_oxygen_file("some.o2").is_ok())
    }

    #[test]
    fn should_err_validate_oxygen_file() {
        assert!(validate_oxygen_file("some").is_err())
    }

    #[test]
    fn validate_cli() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }
}
