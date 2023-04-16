use std::path::PathBuf;

use clap::Args;
use clap_complete::Shell;

/// Generate completions for shell
#[derive(Debug, Clone, PartialEq, Args)]
pub struct Completion {
    /// The shell (bash, zsh, fish, elvish, powershell)
    pub shell: Shell,

    /// The output directory
    #[clap(short, long)]
    pub out_dir: Option<PathBuf>,
}

#[cfg(test)]
mod tests {
    use assert2::check;
    use clap::Parser;

    use super::*;

    #[derive(Debug, Parser)]
    struct JustCompletion {
        #[clap(flatten)]
        command: Completion,
    }

    #[rstest::rstest]
    #[case(&["bin", "zsh"], Completion { 
        shell: Shell::Zsh,
        out_dir: None,
    })]
    #[case(&["bin", "zsh", "--out-dir", "~/.zfunc"], Completion { 
        shell: Shell::Zsh,
        out_dir: Some(PathBuf::from("~/.zfunc")),
    })]
    #[case(&["bin", "zsh", "-o", "~/.zfunc"], Completion { 
        shell: Shell::Zsh,
        out_dir: Some(PathBuf::from("~/.zfunc")),
    })]
    fn should_parse_schema(#[case] args: &[&str], #[case] expected: Completion) {
        let result = JustCompletion::parse_from(args);
        check!(result.command == expected);
    }
}
