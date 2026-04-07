# prompt

A small CLI that sends stdin plus a selected prompt file to OpenAI chat completions and prints the model output.

## Install

```bash
cargo install --path .
```

## Prompt files

Prompt files are read from `$HOME/prompts` by basename.

Given:

- `$HOME/prompts/review.md`
- `$HOME/prompts/review.txt`
- `$HOME/prompts/review.prompt`

you can run:

```bash
echo "some input" | prompt review
```

The tool checks extensions in this order: `.md`, `.txt`, `.prompt`.

## Environment

The CLI tries to load `$HOME/prompts/.env` and expects:

- `OPENAI_API_KEY` (required)
- `OPENAI_MODEL` (optional, default: `gpt-5.4`)

## Usage

```bash
prompt <prompt_basename>
prompt --compinit
prompt --completion <zsh|bash>
prompt --complete [prefix]
```

- `--compinit` prints a zsh completion function compatible with `compinit`.
- `--completion <zsh|bash>` prints a completion script for the selected shell.
- `--complete [prefix]` prints matching prompt basenames (used by completion scripts).

## zsh compinit completion

Add this to your shell startup file (for example `~/.zshrc`) after `compinit` is initialized:

```bash
eval "$(prompt --compinit)"
```

Then reload your shell.

You can also use:

```bash
eval "$(prompt --completion zsh)"
```

## bash completion

Add this to your shell startup file (for example `~/.bashrc`):

```bash
source <(prompt --completion bash)
```

Then reload your shell.

Completion suggestions are basenames of files in `$HOME/prompts` with `.md`, `.txt`, or `.prompt` extensions.
