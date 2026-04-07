use std::{
    env,
    io::{self, Read},
    process,
};

use async_openai::{
    Client,
    config::OpenAIConfig,
    types::chat::{
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs,
    },
};
use cap_std::{ambient_authority, fs::Dir};
use dotenv::from_path;

mod cli;
mod completion;

use cli::{CliMode, parse_cli_args};
use completion::list_prompt_basenames;

pub(crate) const PROMPTS_DIR_NAME: &str = ".prompts";
pub(crate) const PROMPT_FILE_EXTENSIONS: [&str; 3] = ["md", "txt", "prompt"];

pub(crate) fn prompt_extensions_display() -> String {
    PROMPT_FILE_EXTENSIONS
        .iter()
        .map(|ext| format!(".{ext}"))
        .collect::<Vec<_>>()
        .join(", ")
}

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        eprintln!("{err}");
        process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    match parse_cli_args()? {
        CliMode::CompletionScript { shell } => {
            print!("{}", shell.completion_script());
            Ok(())
        }
        CliMode::Complete { prefix } => {
            for candidate in list_prompt_basenames(&prefix)? {
                println!("{candidate}");
            }
            Ok(())
        }
        CliMode::Run { basename } => run_prompt(&basename).await,
    }
}

async fn run_prompt(basename: &str) -> Result<(), Box<dyn std::error::Error>> {
    if basename.starts_with('.') {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Prompt basename cannot start with '.'",
        )
        .into());
    }

    let prompt_text = read_prompt_text(basename)?;

    let mut stdin_text = String::new();
    io::stdin().read_to_string(&mut stdin_text)?;

    let home =
        home::home_dir().ok_or_else(|| io::Error::other("Could not resolve home directory"))?;
    let env_path = home.join(PROMPTS_DIR_NAME).join(".env");
    if let Err(err) = from_path(&env_path) {
        eprintln!("Warning: could not load {}: {err}", env_path.display());
    }

    let api_key = env::var("OPENAI_API_KEY")
        .ok()
        .map(|k| k.trim().to_string())
        .filter(|k| !k.is_empty())
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::PermissionDenied,
                "OPENAI_API_KEY is required (set it in local .env or environment)",
            )
        })?;
    let model = env::var("OPENAI_MODEL").unwrap_or_else(|_| "gpt-5.4".to_string());

    let request = CreateChatCompletionRequestArgs::default()
        .model(model)
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content(prompt_text)
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content(stdin_text)
                .build()?
                .into(),
        ])
        .build()?;

    let client = Client::with_config(OpenAIConfig::new().with_api_key(api_key));
    let response = client.chat().create(request).await?;

    let output = response
        .choices
        .first()
        .and_then(|choice| choice.message.content.as_deref())
        .map(str::trim)
        .filter(|text| !text.is_empty())
        .ok_or_else(|| io::Error::other("OpenAI returned no text output"))?;

    println!("{output}");
    Ok(())
}

fn read_prompt_text(basename: &str) -> Result<String, io::Error> {
    let home =
        home::home_dir().ok_or_else(|| io::Error::other("Could not resolve home directory"))?;
    let prompt_dir_path = home.join(PROMPTS_DIR_NAME);
    let prompt_dir = Dir::open_ambient_dir(&prompt_dir_path, ambient_authority())?;

    for ext in PROMPT_FILE_EXTENSIONS {
        let file_name = format!("{basename}.{ext}");
        match prompt_dir.open(&file_name) {
            Ok(mut file) => {
                let mut prompt_text = String::new();
                file.read_to_string(&mut prompt_text)?;
                return Ok(prompt_text);
            }
            Err(err) if err.kind() == io::ErrorKind::NotFound => continue,
            Err(err) => return Err(err),
        }
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        format!(
            "No prompt file found for '{basename}' in {} with extensions {}",
            prompt_dir_path.display(),
            prompt_extensions_display()
        ),
    ))
}
