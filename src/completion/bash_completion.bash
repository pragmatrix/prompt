_prompt_completion() {
  local cur
  cur="${COMP_WORDS[COMP_CWORD]}"

  local suggestions
  suggestions=$(prompt --complete "$cur")
  COMPREPLY=($(compgen -W "$suggestions" -- "$cur"))
}

complete -F _prompt_completion prompt
