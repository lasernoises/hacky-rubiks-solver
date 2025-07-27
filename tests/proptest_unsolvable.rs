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
                w b,
                r w,
            ],
            bottom: [
                y y,
                y o,
            ],
            left: [
                r y,
                r b,
            ],
            right: [
                g w,
                o o,
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
