use hacky_rubiks_solver::{Cube, Move, cube};
use proptest::{prelude::*, sample::SizeRange};

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 1_000_000,
        ..ProptestConfig::default()
    })]
    #[test]
    fn unsolvable(ref vec in any_with::<Vec<Move>>((SizeRange::new(0..=100), 2))) {
        let mut cube: Cube<2> = cube! {
            top: [
                r w,
                w b,
            ],
            bottom: [
                o y,
                y y,
            ],
            left: [
                y b,
                r r,
            ],
            right: [
                o g,
                o w,
            ],
            front: [
                b o,
                w b,
            ],
            back: [
                r g,
                g g,
            ],
        };

        for move_ in vec {
            cube.apply(*move_);
        }

        prop_assert!(!cube.solved());
    }
}
