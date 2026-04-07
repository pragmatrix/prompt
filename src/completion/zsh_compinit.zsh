#compdef prompt
_prompt() {
  local -a basenames
  basenames=("${(@f)$(prompt --complete "$PREFIX")}")
  compadd -- $basenames
}

compdef _prompt prompt
