use hacky_rubiks_solver::{Cube, Move, cube};
use proptest::prelude::*;

fn strat(layers: usize, min_size: usize, max_size: usize) -> BoxedStrategy<Vec<Move>> {
    let moves = layers * 3;

    (
        0..moves,
        1..=3u8,
        proptest::collection::vec((0..moves - 1, 1..=3u8), min_size..=max_size),
    )
        .prop_map(move |(inital_move, initial_repeat, remaining)| {
            let push_move = |moves: &mut Vec<Move>, move_idx: usize, repeat: u8| {
                for _ in 0..repeat {
                    moves.push(match move_idx / layers {
                        0 => Move::X(move_idx % layers),
                        1 => Move::Y(move_idx % layers),
                        2 => Move::Z(move_idx % layers),
                        _ => unreachable!(),
                    });
                }
            };

            // This capacity is an upper bound. Maybe a somewhat lower number would be better so
            // that most of the time it's sufficent, but I'm not sure there. Might also depend on
            // the allocator, I suppose.
            let mut moves: Vec<Move> = Vec::with_capacity(3 + remaining.len() * 3);

            push_move(&mut moves, inital_move, initial_repeat);

            let mut last_move = inital_move;

            for (move_idx, repeat) in remaining {
                let move_idx = if move_idx >= last_move {
                    move_idx + 1
                } else {
                    move_idx
                };

                push_move(&mut moves, move_idx, repeat);

                last_move = move_idx;
            }

            moves
        })
        .boxed()
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 1_000_000,
        ..ProptestConfig::default()
    })]
    #[test]
    fn unsolvable(ref vec in strat(3, 16, 32)) {
        // let mut cube: Cube<2> = cube! {
        //     top: [
        //         r r,
        //         o y,
        //     ],
        //     bottom: [
        //         o y,
        //         w w,
        //     ],
        //     left: [
        //         b r,
        //         y w,
        //     ],
        //     right: [
        //         o g,
        //         r o,
        //     ],
        //     front: [
        //         w y,
        //         b g,
        //     ],
        //     back: [
        //         b g,
        //         g b,
        //     ],
        // };
        // let mut cube: Cube<3> = cube! {
        //     top: [
        //         o o w,
        //         w o w,
        //         g r b,
        //     ],
        //     bottom: [
        //         g r r,
        //         b r g,
        //         o y y,
        //     ],
        //     left: [
        //         w y b,
        //         w b y,
        //         r b w,
        //     ],
        //     right: [
        //         o o r,
        //         o g b,
        //         g g w,
        //     ],
        //     front: [
        //         b r y,
        //         b y g,
        //         y y b,
        //     ],
        //     back: [
        //         r g g,
        //         r w o,
        //         y w o,
        //     ],
        // };
        let mut cube: Cube<3> = cube! {
            top: [
                y g y,
                y g y,
                o r o,
            ],
            bottom: [
                r b w,
                r b w,
                r o w,
            ],
            left: [
                b b o,
                o y y,
                b b w,
            ],
            right: [
                r g g,
                w w w,
                y g g,
            ],
            front: [
                r r y,
                r r y,
                b b b,
            ],
            back: [
                w o o,
                w o o,
                g g g,
            ],
        };

        for move_ in vec {
            cube.apply(*move_);
            prop_assert!(!cube.solved());
        }
    }
}
