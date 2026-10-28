#!/usr/bin/env bash
set -euo pipefail

MODEL_DIR="$HOME/.config/eery"
BIN_DIR="$HOME/.local/bin"
ENGINE="$BIN_DIR/needle-engine"
HF="https://huggingface.co/Cactus-Compute"

download() {
  local url="$1" dest="$2"
  echo "downloading $(basename "$dest")..."
  curl -fL --retry 3 --progress-bar -o "$dest.part" "$url"
  mv "$dest.part" "$dest"
}

mkdir -p "$MODEL_DIR" "$BIN_DIR"

[ -s "$MODEL_DIR/needle3.cact" ] || download "$HF/needle3/resolve/main/needle3.cact" "$MODEL_DIR/needle3.cact"
[ -s "$MODEL_DIR/whistle.cact" ] || download "$HF/whistle/resolve/main/whistle.cact" "$MODEL_DIR/whistle.cact"

case "$(uname -m)" in
x86_64) PLAT=linux-x86_64 ;;
aarch64) PLAT=linux-arm64 ;;
armv7l) PLAT=linux-armv7 ;;
riscv64) PLAT=linux-riscv64 ;;
*)
  echo "unsupported architecture: $(uname -m)" >&2
  exit 1
  ;;
esac

if [ ! -x "$ENGINE" ]; then
  download "$HF/needle3/resolve/main/$PLAT/needle" "$ENGINE"
  chmod +x "$ENGINE"
fi

case ":$PATH:" in
*":$BIN_DIR:"*) ;;
*) echo "note: $BIN_DIR is not on your PATH; add it to your shell config." ;;
esac

echo "done."
echo "models: $MODEL_DIR"
echo "engine: $ENGINE"
