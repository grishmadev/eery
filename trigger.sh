#!/bin/bash
PID_FILE="/tmp/eery.pid"
WAV_PATH="/tmp/eery.wav"

if [ -f "$PID_FILE" ]; then
  kill $(cat "$PID_FILE") 2>/dev/null
  rm -f "$PID_FILE"
  echo "Recording stopped. Processing command..."
  notify-send "Recorded. Parsing."
  eery --audio /tmp/eery.wav
else
  rm -f "$PID_FILE" "$WAV_PATH"

  ffmpeg -y -f alsa -i default -t 30 "$WAV_PATH" &
  echo $! >"$PID_FILE"
  notify-send "Recording."
fi
