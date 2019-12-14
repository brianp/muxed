#/usr/bin/env bash
_muxed_completions()
{
  if [ "$COMP_CWORD" -eq 1 ]; then
    local commands="$(compgen -W "new edit snapshot" "${COMP_WORDS[1]}")"
    local projects="$(compgen -W "$(echo $(ls ~/.muxed/))" "${COMP_WORDS[1]}")"
    COMPREPLY=( $commands $projects )
  elif [ "$COMP_CWORD" -eq 2 ]; then
    local projects="$(compgen -W "$(echo $(ls ~/.muxed/))" "${COMP_WORDS[2]}")"
    COMPREPLY=( $projects )
  fi
}

complete -F _muxed_completions muxed
