use clap::{Arg, ArgAction, ArgGroup, Command};

pub enum CliMode {
    Run { basename: String },
    Compinit,
    Complete { prefix: String },
}

pub fn parse_cli_args() -> Result<CliMode, clap::Error> {
    let matches = Command::new("prompt")
        .about("Send stdin with a prompt file to OpenAI chat completions")
        .arg(
            Arg::new("prompt_basename")
                .value_name("prompt_basename")
                .help("Basename in $HOME/prompts to load (.md, .txt, .prompt)"),
        )
        .arg(
            Arg::new("compinit")
                .long("compinit")
                .help("Print a zsh completion function compatible with compinit")
                .action(ArgAction::SetTrue),
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
                .args(["prompt_basename", "compinit", "complete"])
                .required(true)
                .multiple(false),
        )
        .try_get_matches()?;

    if matches.get_flag("compinit") {
        return Ok(CliMode::Compinit);
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
