use clap::Args;
use std::io::Cursor;

use clap_complete::generate;
use color_eyre::eyre::Result;

use crate::cli::self_update::SelfUpdate;
use crate::config::Config;
use crate::output::Output;
use crate::shell::completions::zsh_complete;

/// Generate shell completions
#[derive(Debug, Args)]
#[clap(hide = true, verbatim_doc_comment)]
pub struct RenderCompletion {
    /// Shell type to generate completions for
    #[clap(required_unless_present = "shell_type")]
    shell: Option<clap_complete::Shell>,

    /// Shell type to generate completions for
    #[clap(long = "shell", short = 's', hide = true)]
    shell_type: Option<clap_complete::Shell>,
}

impl RenderCompletion {
    pub fn run(self, _config: Config, out: &mut Output) -> Result<()> {
        let shell = self.shell.or(self.shell_type).unwrap();

        let mut c = Cursor::new(Vec::new());
        let mut cmd = crate::cli::Cli::command().subcommand(SelfUpdate::command());

        if let clap_complete::Shell::Zsh = shell {
            rtxprintln!(out, "{}", zsh_complete(&cmd)?);
        } else {
            generate(shell, &mut cmd, "rtx", &mut c);
            rtxprintln!(out, "{}", String::from_utf8(c.into_inner()).unwrap());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_cli;

    #[test]
    fn test_completion() {
        assert_cli!("render-completion", "bash");
        assert_cli!("render-completion", "fish");
        assert_cli!("render-completion", "zsh");
    }
}