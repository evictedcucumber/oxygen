use crate::error::{ResultT, cmdline::CmdlineError};

/// Stores the current config related to the program.
#[derive(Debug)]
pub struct ProgramConfig {
    /// Contains whether or not to display the help text of the program
    pub display_help: bool,
    /// Contains whether or not to display the tokens created during the
    /// compilation.
    pub display_tokens: bool,
}

impl ProgramConfig {
    /// Creates a new [`ProgramConfig`] with the default values.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = ProgramConfig::new();
    /// assert_eq!(config, ProgramConfig {
    ///     display_help: false,
    ///     display_tokens: false,
    /// });
    /// ```
    pub fn new() -> Self {
        Self {
            display_help: false,
            display_tokens: false,
        }
    }
}

/// A representation of a cmdline option.
enum Options {
    /// Represents displaying the help text.
    DisplayHelp,
    /// Represents displaying the tokens generated during compilation.
    DisplayTokens,
}

impl Options {
    pub const VALUES: [Self; 2] = [Self::DisplayHelp, Self::DisplayTokens];

    /// Gets the option's possible arguments.
    ///
    /// # Examples
    ///
    /// ```
    /// let args = Options::DisplayHelp::to_args();
    /// assert_eq!(args, &["-h", "--help"]);
    /// ```
    pub fn to_args(&self) -> &[&str] {
        use Options::*;

        match self {
            DisplayHelp => &["-h", "--help"],
            DisplayTokens => &["--display-tokens"],
        }
    }

    /// Gets the option's description.
    ///
    /// # Examples
    ///
    /// ```
    /// let desc = Options::DisplayHelp::to_desc();
    /// assert_eq!(desc, "Display this help text.");
    /// ```
    pub fn to_desc(&self) -> &str {
        use Options::*;

        match self {
            DisplayHelp => "Display this help text.",
            DisplayTokens => "Display the tokens representing the input file.",
        }
    }

    /// Modifies the given [`ProgramConfig`] matching the option.
    ///
    /// # Examples
    ///
    /// ```
    /// let config = ProgramConfig::new();
    /// assert!(!config.display_help);
    /// Options::DisplayHelp.configure(&mut config);
    /// assert!(config.display_help);
    /// ```
    pub fn configure(&self, config: &mut ProgramConfig) {
        use Options::*;

        match self {
            DisplayHelp => config.display_help = true,
            DisplayTokens => config.display_tokens = true,
        }
    }
}

pub fn help_text() -> String {
    let mut output = format!(
        "Usage: {} [options] <o2_file>\n\nOptions:\n",
        env!("CARGO_BIN_NAME")
    );
    let mut max_length: usize = 0;
    let mut options: Vec<(String, String)> = Vec::new();

    for opt in Options::VALUES {
        let mut option = String::new();
        let args = opt.to_args();
        if args.len() > 1 {
            for arg in args {
                option.push_str(format!("{arg}, ").as_str());
            }
            option.truncate(option.len() - 2);
        } else {
            option.push_str(args[0]);
        }

        if max_length < option.len() {
            max_length = option.len();
        }

        options.push((option, String::from(opt.to_desc())));
    }

    for option in options {
        let (opt, desc) = option;
        let left = format!("{opt: >max_length$}");
        output.push_str(format!("  {left}    {desc}\n").as_str());
    }

    output
}

/// Parses the cmdline arguments into a [`ResultT<ProgramConfig>`].
///
/// # Examples
///
/// ```
/// let args = vec!["-h"];
/// let parsed = parse_args(&args);
/// assert!(parsed.is_ok());
/// ```
///
/// ```
/// let args = vec!["-abc"];
/// let parsed = parse_args(&args);
/// assert!(parsed.is_err());
/// assert_eq!(
///     parsed.err(),
///     Error::CmdlineError(CmdlineError::UnknownArgument("--abc".to_string))
/// );
/// ```
pub fn parse_args(args: &[String]) -> ResultT<ProgramConfig> {
    let mut config = ProgramConfig::new();

    for arg in args {
        compute_args(&mut config, arg)?;
    }

    Ok(config)
}

/// Computes the given arg into its represented option in [`ProgramConfig`] or
/// throws an [`CmdlineError`].
///
/// # Examples
///
/// ```
/// let args = vec!["-h"];
/// let parsed = compute_args(&args);
/// assert!(parsed.is_ok());
/// ```
///
/// ```
/// let args = vec!["-abc"];
/// let parsed = parse_args(&args);
/// assert!(parsed.is_err());
/// assert_eq!(
///     parsed.err(),
///     CmdlineError::UnknownArgument("--abc".to_string)
/// );
/// ```
pub fn compute_args(config: &mut ProgramConfig, arg: &str) -> Result<(), CmdlineError> {
    let mut exists = false;
    for opt in Options::VALUES {
        let possible_args = opt.to_args();
        if possible_args.contains(&arg) {
            opt.configure(config);
            exists = true;
            break;
        } else {
            exists = false;
        }
    }

    if exists {
        Ok(())
    } else {
        Err(CmdlineError::UnknownArgument(arg.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_options_to_args() {
        assert_eq!(Options::DisplayTokens.to_args(), &["--display-tokens"]);
    }

    #[test]
    fn test_options_to_desc() {
        assert_eq!(
            Options::DisplayTokens.to_desc(),
            "Display the tokens representing the input file."
        );
    }

    #[test]
    fn test_options_config() {
        let mut config = ProgramConfig::new();
        Options::DisplayTokens.configure(&mut config);

        assert!(config.display_tokens);
    }

    #[test]
    fn test_help_text() {
        assert!(help_text().starts_with("Usage: o2c [options] <o2_file>\n\nOptions:\n"));
    }

    #[test]
    fn test_compute_args() {
        let mut config = ProgramConfig::new();

        assert!(compute_args(&mut config, "-h").is_ok());
        assert!(compute_args(&mut config, "--abc").is_err());
    }

    #[test]
    fn test_parse_args() {
        let args: &Vec<String> = &[String::from("-h"), String::from("--display-tokens")]
            .into_iter()
            .collect::<Vec<String>>();

        let res = parse_args(args);
        assert!(res.is_ok());

        let config = res.unwrap();

        assert!(config.display_help);
        assert!(config.display_tokens);
    }

    #[test]
    fn test_parse_args_unknown_arg() {
        let args: &Vec<String> = &[String::from("-abc")].into_iter().collect::<Vec<String>>();

        let res = parse_args(args);
        assert!(res.is_err());
    }
}
