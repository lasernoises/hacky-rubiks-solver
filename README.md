# Hacky Rubiks Solver

I wanted to see if I can solve a Rubik's Cube by abusing property based tests. Basically the idea is
write a test that asserts that for any list of moves the cube is unsolvable or will not lead to a
solved state in the cube. If the configuration is indeed solvable, proptest will reduce the moves
required and print the solution.

Currently the implementation is very naive. So for example proptest will happily try the same move
more than three times in a row. Also, I think there's probably a bias for rotation direction,
because we don't encode direction in the moves. This is because we can encode a counterclockwise
move as the same clockwise move three times in a row. This probably leads to it taking longer to
find a solution, but I haven't confirmed this.

The representation of the cube is also not the most efficient. There's a `Color` enum that encodes
the six colors plus an `Inside` variant. The cube is just a multidimensional array of pieces, each
of which has six sides with colors. So we even encode the center piece that has the `Inside` color
on each side. All this is simply because it makes it easier to reason about (for me, at least). Of
course this also means that we can encode tons of invalid and nonsensical states.

## Got an unsolved cube?

Enter the colors on your cube into the `cube!` macro invocation in `tests/proptest_unsolvable.rs`.

Then run:

```sh
cargo test --release
```

Without `--release` also works, but is much slower. On my machine it was about 20x slower.

