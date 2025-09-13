# Rust Raylib Game Template #

## Description ##

This is an example on how to structure a game in rust using raylib.
It displays moving squares and circles.

Main features used here are:

- The main module simply calls game::run() because all the logic is implemented in mod game.
- A GameObject trait is used to gather all elements that can be updated and printed.
- A Vec\<Box\<dyn GameObject\>\> is used to push all objects, that are updated and displayes in respective loops.
- Simple to add other objects by creating a new file in src/game/objects/ with a struct that implements the GameObject trait.

## Module Structure ##

## Screenshot ##

TODO: Add some screenshots of the game
