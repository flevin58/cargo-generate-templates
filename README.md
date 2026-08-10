# cargo-generate-templates
A series of rust templates to use with cargo-generate.

## Configuration ##

Add the following to your **~/.cargo/cargo-generate.toml** file:

```toml
[favorites.flevin58]
description = "A series of templates to get started!"
git = "https://github.com/flevin58/cargo-generate-templates"
```

## Usage ##

1. open the terminal
2. cd to the folder where you keep your projects
3. ender the following command: `cargo generate flevin58`
4. follow the prompts

Note: if you did not configure the favorites as suggested above, then you can use this command:

`cargo generate --git flevin58/cargo-generate-templates`

# raylib-game #

### Description ###

This is an example on how to structure a game in rust using raylib.
It displays moving squares and circles.

Main features used here are:

- The main module simply calls`game::run()`because all the logic is implemented in mod game.
- A`GameObject`trait is used to gather all elements that can be updated and drawed.
- A`Vec<Box<dyn GameObject>>`is used to push all objects, that are updated and displayes in respective loops.
- Simple to add other objects by creating a new file in src/game/objects/ with a struct that implements the`GameObject`trait.

### Module Structure ###

<img width="444" height="391" alt="Capto_Capture 2025-08-18_12-41-57_AM" src="https://github.com/user-attachments/assets/5cd23c00-d846-4cee-a9f7-a2a371faad28" />

### Screenshot ###

<img width="800" height="628" alt="Screenshot" src="https://github.com/user-attachments/assets/b450ff7a-ed0d-4577-9f92-12f2ccc637dc" />
