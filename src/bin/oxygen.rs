use clap::{CommandFactory, Parser};
use clap_complete::aot;
use cmdline::OxygenCommands::Completions;
use cmdline::OxygenShells;

use std::{io, process::exit};

fn gen_comp<G: aot::Generator>(shell_one: G, shell_two: G) {
    let mut o2c_cmd = cmdline::O2CCli::command();
    let mut oxygen_cmd = cmdline::OxygenCli::command();

    aot::generate(shell_one, &mut o2c_cmd, "o2c", &mut io::stdout());
    aot::generate(shell_two, &mut oxygen_cmd, "oxygen", &mut io::stdout());

    exit(0);
}

fn main() {
    let cli = cmdline::OxygenCli::parse();

    match cli.command {
        Completions { shell } => match shell {
            OxygenShells::Fish => gen_comp(aot::Fish, aot::Fish),
            OxygenShells::Bash => gen_comp(aot::Bash, aot::Bash),
            OxygenShells::Zsh => gen_comp(aot::Zsh, aot::Zsh),
        },
    }
}
