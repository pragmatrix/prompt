use clap::{Arg, ArgAction, ArgGroup, Command};

use crate::completion::CompletionShell;
use crate::{PROMPTS_DIR_NAME, prompt_extensions_display};

pub enum CliMode {
    Run { basename: String },
    CompletionScript { shell: CompletionShell },
    Complete { prefix: String },
}

pub fn parse_cli_args() -> Result<CliMode, clap::Error> {
    let prompt_help = format!(
        "Basename in $HOME/{PROMPTS_DIR_NAME} to load ({})",
        prompt_extensions_display()
    );

    let matches = Command::new("prompt")
        .about("Send stdin with a prompt file to OpenAI chat completions")
        .arg(
            Arg::new("prompt_basename")
                .value_name("prompt_basename")
                .help(prompt_help),
        )
        .arg(
            Arg::new("compinit")
                .long("compinit")
                .help("Print a zsh completion function compatible with compinit")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("completion")
                .long("completion")
                .value_name("shell")
                .help("Print a shell completion script (zsh or bash)")
                .value_parser(["zsh", "bash"]),
        )
        .arg(
            Arg::new("complete")
                .long("complete")
                .value_name("prefix")
                .help("List matching prompt basenames for shell completion")
                .num_args(0..=1)
                .default_missing_value(""),
        )
        .group(
            ArgGroup::new("mode")
                .args(["prompt_basename", "compinit", "completion", "complete"])
                .required(true)
                .multiple(false),
        )
        .try_get_matches()?;

    if matches.get_flag("compinit") {
        return Ok(CliMode::CompletionScript {
            shell: CompletionShell::Zsh,
        });
    }

    if let Some(shell) = matches.get_one::<String>("completion") {
        let shell = match shell.as_str() {
            "zsh" => CompletionShell::Zsh,
            "bash" => CompletionShell::Bash,
            _ => unreachable!("clap value_parser enforces valid shell"),
        };
        return Ok(CliMode::CompletionScript { shell });
    }

    if let Some(prefix) = matches.get_one::<String>("complete") {
        return Ok(CliMode::Complete {
            prefix: prefix.to_string(),
        });
    }

    let basename = matches
        .get_one::<String>("prompt_basename")
        .expect("clap requires one mode")
        .to_string();
    Ok(CliMode::Run { basename })
}
