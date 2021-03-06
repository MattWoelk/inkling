# Inkling

Limited Rust implementation of the `Ink` markup/scripting language for game dialog.

Ink is a creation of [Inkle](https://www.inklestudios.com/). For more information about the language, [see their website](https://www.inklestudios.com/ink/).

```
Using Ink you can easily write a story or create a dialog tree.

*   Branching is very simple[]: <>
    just start your line with an asterix or plus marker.
    Want nested choices? Add more markers!
    * *     A branching choice contains all information below it
            of the same level or higher.
            * * *       [I see.]
    * *     Pretty cool, huh?
    - -     Use gather points like this to return all nested choices <>
            to a single path.
    * *     [Cool!] -> fin

*   You can organize the story using knots <>
    and divert (move) to them, like this:
    -> next_knot

=== next_knot ===
Simple and fun.
-> fin

=== fin ===
Ink is very powerful and has a lot more features than shown here. <>
Do note that `inkling` only implements a subset of all its features. <>
Hopefully more in the future!
-> END
```


## Features

Currently and likely for the foreseeable future the feature set is very limited compared to Inkle's own implementation. Available features are:

*   Knots, stitches, glue and diverts, ie. basic story structure
*   Choices, of sticky and non-sticky kinds, plus fallback choices
*   Nesting choices and gather points
*   Line text alternative sequences (sequences, cycle, once-only) and conditions
*   Conditionals for displaying text and choices to user
*   Tagging of lines and choices
*   Variables in choices, conditions and text
*   Optional: De/serialization of finished stories through `serde`

Likely candidates for further development:

*   Variable modification in scripts
*   Includes of other files

Difficult features for which I doubt my skill level to implement:

*   Advanced flow control: tunnels and threads
*   Verifying that all story branches are complete


## Usage

See the [documentation](https://docs.rs/inkling/) or provided example for a minimum viable story processor. Enable `serde` de/serialization by activating the `serde_support` feature. This feature derives `Deserialize` and `Serialize` for all required structs.


## Contributions

Writing this has mostly been for fun and to create a simple game, hence the lack of features. Contributions are welcome!
