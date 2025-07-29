#![no_main]

use hacky_rubiks_solver::{Cube, Move, cube};
use libfuzzer_sys::arbitrary::{Arbitrary, Result, Unstructured};
use libfuzzer_sys::fuzz_target;

#[derive(Debug)]
enum ArbitraryMove<const LAYERS: usize> {
    X(usize),
    Y(usize),
    Z(usize),
}

impl<'a, const LAYERS: usize> Arbitrary<'a> for ArbitraryMove<LAYERS> {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
        let layer = u.int_in_range(0..=LAYERS - 1)?;

        Ok(match u.int_in_range(0..=2)? {
            0 => Self::X(layer),
            1 => Self::Y(layer),
            2 => Self::Z(layer),
            _ => unreachable!(),
        })
    }
}

impl<const LAYERS: usize> ArbitraryMove<LAYERS> {
    fn into_move(self) -> hacky_rubiks_solver::Move {
        match self {
            Self::X(layer) => Move::X(layer),
            Self::Y(layer) => Move::Y(layer),
            Self::Z(layer) => Move::Z(layer),
        }
    }
}

fuzz_target!(|data: Vec<ArbitraryMove<3>>| {
    let mut cube: Cube<3> = cube! {
        top: [
            o o w,
            w o w,
            g r b,
        ],
        bottom: [
            g r r,
            b r g,
            o y y,
        ],
        left: [
            w y b,
            w b y,
            r b w,
        ],
        right: [
            o o r,
            o g b,
            g g w,
        ],
        front: [
            b r y,
            b y g,
            y y b,
        ],
        back: [
            r g g,
            r w o,
            y w o,
        ],
    };

    for move_ in data {
        cube.apply(move_.into_move());
    }

    assert!(!cube.solved());

    // fuzzed code goes here
});
