#!/bin/bash

# Read history in reverse to find the most recent nvim command
LAST_CMD=$(tac ~/.bash_history | grep -m 1 '^nvim\s\+')

if [ -z "$LAST_CMD" ]; then
  notify-send "No recent nvim command found in history."
  exit 1
fi

kitty --detach sh -c "$LAST_CMD; exec bash"
