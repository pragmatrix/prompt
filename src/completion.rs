use std::fs;
use std::io;

pub enum CompletionShell {
    Zsh,
    Bash,
}

impl CompletionShell {
    pub fn completion_script(self) -> &'static str {
        match self {
            CompletionShell::Zsh => include_str!("completion/zsh_compinit.zsh"),
            CompletionShell::Bash => include_str!("completion/bash_completion.bash"),
        }
    }
}

pub fn list_prompt_basenames(prefix: &str) -> Result<Vec<String>, io::Error> {
    let home =
        home::home_dir().ok_or_else(|| io::Error::other("Could not resolve home directory"))?;
    let prompt_dir_path = home.join("prompts");

    let mut basenames = std::collections::BTreeSet::new();
    let entries = match fs::read_dir(&prompt_dir_path) {
        Ok(entries) => entries,
        Err(err) if err.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(err) => return Err(err),
    };

    for entry_result in entries {
        let entry = entry_result?;
        let file_name = entry.file_name();
        let file_name = file_name.to_string_lossy();

        if file_name.starts_with('.') {
            continue;
        }

        let path = std::path::Path::new(file_name.as_ref());
        let stem = match path.file_stem().and_then(|s| s.to_str()) {
            Some(stem) if !stem.is_empty() => stem,
            _ => continue,
        };
        let ext = match path.extension().and_then(|s| s.to_str()) {
            Some(ext) => ext,
            None => continue,
        };

        if !matches!(ext, "md" | "txt" | "prompt") {
            continue;
        }
        if !stem.starts_with(prefix) {
            continue;
        }
        basenames.insert(stem.to_string());
    }

    Ok(basenames.into_iter().collect())
}
