# Eery

Yo! This is a new project I thought of tonight.

There are 2 tiny ahh models working one after another.

- [Whistle](https://cactuscompute.com/blog/whistle) for voice recognition ~ 16MB
- Needle3 (Same Provider) for tool calling ~ 34MB

## Setup

- Install Whistle via Python

```sh
pip install cactus-needle
needle install linux-x86_64 # or arm64 if your chip says so
needle install whisper
```

- Install eery

```sh
cargo install --git https://github.com/grishmadev/eery.git
```
