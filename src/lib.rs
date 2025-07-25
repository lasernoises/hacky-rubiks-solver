#[derive(Copy, Clone, PartialEq, Eq)]
enum Color {
    White,
    Blue,
    Red,
    Orange,
    Green,
    Yellow,
    Inside,
}

struct Piece {
    top: Color,
    bottom: Color,
    left: Color,
    right: Color,
    front: Color,
    back: Color,
}

/// Rotation is always clockwise.
#[derive(Copy, Clone, Debug, Arbitrary)]
enum Move {
    Top,
    Bottom,
    Left,
    Right,
}

struct Cube {
    top_left_front: Piece,
    top_left_back: Piece,
    top_right_front: Piece,
    top_right_back: Piece,
    bottom_left_front: Piece,
    bottom_left_back: Piece,
    bottom_right_front: Piece,
    bottom_right_back: Piece,
}

fn rotate<T>(a: &mut T, b: &mut T, c: &mut T, d: &mut T) {
    use std::mem::swap;

    // 1 2 3 4
    // 2 1 3 4 <- swap(a, b)
    // 2 1 4 3 <- swap(c, d)
    // 4 1 2 3 <- swap(a, c)
    swap(a, b);
    swap(c, d);
    swap(a, c);
}

impl Piece {
    fn apply(&mut self, move_: Move) -> &mut Self {
        match move_ {
            Move::Top => rotate(
                &mut self.front,
                &mut self.left,
                &mut self.back,
                &mut self.right,
            ),
            Move::Bottom => rotate(
                &mut self.back,
                &mut self.right,
                &mut self.front,
                &mut self.left,
            ),
            Move::Left => rotate(
                &mut self.bottom,
                &mut self.back,
                &mut self.top,
                &mut self.front,
            ),
            Move::Right => rotate(
                &mut self.top,
                &mut self.back,
                &mut self.bottom,
                &mut self.front,
            ),
        }

        self
    }
}

impl Cube {
    fn apply(&mut self, move_: Move) {
        match move_ {
            Move::Top => rotate(
                self.top_left_front.apply(move_),
                self.top_left_back.apply(move_),
                self.top_right_back.apply(move_),
                self.top_right_front.apply(move_),
            ),
            Move::Bottom => rotate(
                self.bottom_right_front.apply(move_),
                self.bottom_left_front.apply(move_),
                self.bottom_left_back.apply(move_),
                self.bottom_right_back.apply(move_),
            ),
            Move::Left => rotate(
                self.bottom_left_front.apply(move_),
                self.bottom_left_back.apply(move_),
                self.top_left_back.apply(move_),
                self.top_left_front.apply(move_),
            ),
            Move::Right => rotate(
                self.bottom_right_back.apply(move_),
                self.bottom_right_front.apply(move_),
                self.top_right_front.apply(move_),
                self.top_right_back.apply(move_),
            ),
        }
    }

    fn solved(&self) -> bool {
        fn all_eq(side: [Color; 4]) -> bool {
            side.windows(2).all(|w| w[0] == w[1])
        }

        all_eq([
            self.top_left_front.top,
            self.top_left_back.top,
            self.top_right_front.top,
            self.top_right_back.top,
        ]) && all_eq([
            self.bottom_left_front.bottom,
            self.bottom_left_back.bottom,
            self.bottom_right_front.bottom,
            self.bottom_right_back.bottom,
        ]) && all_eq([
            self.top_left_front.left,
            self.top_left_back.left,
            self.bottom_left_front.left,
            self.bottom_left_back.left,
        ]) && all_eq([
            self.top_right_front.right,
            self.top_right_back.right,
            self.bottom_right_front.right,
            self.bottom_right_back.right,
        ]) && all_eq([
            self.top_left_front.front,
            self.top_right_front.front,
            self.bottom_left_front.front,
            self.bottom_right_front.front,
        ]) && all_eq([
            self.top_left_back.back,
            self.top_right_back.back,
            self.bottom_left_back.back,
            self.bottom_right_back.back,
        ])
    }
}

use Color::*;

const SOLVED: Cube = Cube {
    top_left_front: Piece {
        top: White,
        bottom: Inside,
        left: Blue,
        right: Inside,
        front: Orange,
        back: Inside,
    },
    top_left_back: Piece {
        top: White,
        bottom: Inside,
        left: Blue,
        right: Inside,
        front: Inside,
        back: Red,
    },
    top_right_front: Piece {
        top: White,
        bottom: Inside,
        left: Inside,
        right: Green,
        front: Orange,
        back: Inside,
    },
    top_right_back: Piece {
        top: White,
        bottom: Inside,
        left: Inside,
        right: Green,
        front: Inside,
        back: Red,
    },
    bottom_left_front: Piece {
        top: Inside,
        bottom: Yellow,
        left: Blue,
        right: Inside,
        front: Orange,
        back: Inside,
    },
    bottom_left_back: Piece {
        top: Inside,
        bottom: Yellow,
        left: Blue,
        right: Inside,
        front: Inside,
        back: Red,
    },
    bottom_right_front: Piece {
        top: Inside,
        bottom: Yellow,
        left: Inside,
        right: Green,
        front: Orange,
        back: Inside,
    },
    bottom_right_back: Piece {
        top: Inside,
        bottom: Yellow,
        left: Inside,
        right: Green,
        front: Inside,
        back: Red,
    },
};

use proptest::prelude::*;
use proptest_derive::Arbitrary;
proptest! {
    // #[test]
    // fn doesnt_crash(ref vec in any::<Vec<Move>>()) {
    //     let mut cube = SOLVED;

    //     for move_ in vec {
    //         cube.apply(*move_);
    //     }

    //     prop_assert!(!cube.solved());
    // }
}

#[test]
fn solved_is_solved() {
    assert!(SOLVED.solved());
}
