#!/bin/bash

HIST="${HISTFILE:-$HOME/.bash_history}"

# Walk history oldest -> newest. Remember each cd; when an nvim line shows up,
# save it together with the closest cd above it. The last pair wins.
PAIR=$(awk '
  /^cd([ \t]|$)/   { cd = $0 }
  /^nvim([ \t]|$)/ { pcd = cd; pnv = $0 }
  END { if (pnv != "") { print pcd; print pnv } }
' "$HIST")

if [ -z "$PAIR" ]; then
  notify-send "No nvim command found in history."
  exit 1
fi

CD_CMD=$(printf '%s\n' "$PAIR" | sed -n 1p)
NVIM_CMD=$(printf '%s\n' "$PAIR" | sed -n 2p)

if [ -n "$CD_CMD" ]; then
  RUN="$CD_CMD && $NVIM_CMD"
else
  RUN="$NVIM_CMD"
fi

# Start in $HOME so a relative "cd project" resolves from home, as you wanted
kitty --detach --directory "$HOME" bash -c "$RUN; exec bash"
