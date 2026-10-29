# Eery

Wassup! This is a new project I thought of tonight.

There are 2 tiny ahh models working one after another.

- [Whistle](https://cactuscompute.com/whistle) for voice recognition ~ 16MB
- [Needle3](https://cactuscompute.com/needle) for tool calling ~ 34MB

## Setup

```sh
git clone https://github.com/grishmadev/eery.git
cd eery
make install
```

And Done!

## Configuration

All the configuration (including the models) are to be in `$HOME/.config/eery/` directory.
Follow the format in [example](/example) to edit you configs.


- `tools.json` for declaring tools for the model
- `keywords.txt` for the model to bias it's words on.

Else remaining are scripts that you can use in your window manager to create a shortcut using the [trigger](/trigger.sh)

### Arguments

`--tools <path>`: Point to the tools path

`--keywords <path>`: Point to the keywords file

`--voice-model <path>`: Point to speech recognizing model

`--parse-model <path>`: Point to parsing model

`--audio <path>`: Audio to parse

`--manual`: Whether to parse using model (automatic) or pattern matching (manual) [default: false]

Note: Parsing with model is **more dynamic** but **slower** and uses **more resource** (although not much)
while Manual mode is **faster** since its only pattern matching but is **less dynamic**.

That's.. it!

Thank you for using my weekend project.
