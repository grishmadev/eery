# Eery

Wassup! This is a new project I thought of tonight.

There are 2 tiny ahh models working one after another.

- [Whistle](https://cactuscompute.com/blog/whistle) for voice recognition ~ 16MB
- Needle3 (Also from cactus) for tool calling ~ 34MB

## Setup

```sh
git clone https://github.com/grishmadev/eery.git
cd eery
make install
```

And Done!

### Configuration

All the configuration (including the models) are in the `$HOME/.config/eery/` directory.
Follow the format in [example](/example) to edit you configs

- `tools.json` for declaring tools for the model
- `keywords.txt` for the model to bias it's words on.

Else remaining are scripts that you can use in your window manager to create a shortcut using the [trigger](/trigger.sh)

That's.. it!

Thank you for using my weekend project.
