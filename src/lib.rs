#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Color {
    White,
    Blue,
    Red,
    Orange,
    Green,
    Yellow,
    Inside,
}

#[derive(Copy, Clone, Debug)]
pub struct Piece {
    pub top: Color,
    pub bottom: Color,
    pub left: Color,
    pub right: Color,
    pub front: Color,
    pub back: Color,
}

/// A move is clockwise rotation of the layer with the given index on the given axis. Clockwise
/// means clockwise when looking in the direction of the axis, so when rotating the bottom face of
/// the cube it's counterclockwise when you look at it from the bottom.
#[derive(Copy, Clone, Debug, proptest_derive::Arbitrary)]
#[proptest(params(usize))]
pub enum Move {
    X(#[proptest(strategy = "0..params")] usize),
    Y(#[proptest(strategy = "0..params")] usize),
    Z(#[proptest(strategy = "0..params")] usize),
}

#[derive(Debug)]
pub struct Cube<const LAYERS: usize> {
    /// In order from outer to inner arrays these encode the x, y and z axes. This encoding is
    /// simply for the convenience of indexing them as `[x][y][z]`.
    pub content: [[[Piece; LAYERS]; LAYERS]; LAYERS],
}

#[derive(Debug)]
struct CubeLayer<'a, T, const LAYERS: usize>([[&'a mut T; LAYERS]; LAYERS]);

impl<'a, T, const LAYERS: usize> CubeLayer<'a, T, LAYERS> {
    fn four_disjoint_mut(&mut self, indices: [(usize, usize); 4]) -> [&mut T; 4] {
        let flattened = self.0.as_flattened_mut();

        flattened
            .get_disjoint_mut(indices.map(|(x, y)| x * LAYERS + y))
            .unwrap()
            .map(|piece| &mut **piece)
    }

    fn rotate(&mut self) {
        for i in 0..LAYERS / 2 {
            let size = LAYERS - i * 2;

            // 0 1 2 3      C 1 2 0      C 8 2 0      C 8 4 0
            // 4 5 6 7  =>  4 5 6 7  =>  4 5 6 1  =>  D 5 6 1
            // 8 9 A B      8 9 A B      E 9 A B      E 9 A 2
            // C D E F      F D E 3      F D 7 3      F B 7 3
            for j in 0..size - 1 {
                let [a, b, c, d] = self.four_disjoint_mut([
                    (i + j, i),
                    (i + size - 1, i + j),
                    (i + size - 1 - j, i + size - 1),
                    (i, i + size - 1 - j),
                ]);
                rotate(a, b, c, d);
            }
        }
    }
}

impl<'a, const LAYERS: usize> CubeLayer<'a, Piece, LAYERS> {
    fn rotate_x(&mut self) {
        self.rotate();

        for piece in self.0.as_flattened_mut() {
            piece.rotate_x();
        }
    }

    fn rotate_y(&mut self) {
        self.rotate();

        for piece in self.0.as_flattened_mut() {
            piece.rotate_y();
        }
    }

    fn rotate_z(&mut self) {
        self.rotate();

        for piece in self.0.as_flattened_mut() {
            piece.rotate_z();
        }
    }
}

fn rotate<T>(a: &mut T, b: &mut T, c: &mut T, d: &mut T) {
    // 1 2 3 4
    // 2 1 3 4 <- swap(a, b)
    // 2 1 4 3 <- swap(c, d)
    // 4 1 2 3 <- swap(a, c)
    std::mem::swap(a, b);
    std::mem::swap(c, d);
    std::mem::swap(a, c);
}

fn swap_axes<T, const N: usize>(grid: [[T; N]; N]) -> [[T; N]; N] {
    let mut iters = grid.map(|x| x.into_iter());

    [0; N].map(|_| iters.each_mut().map(|x| x.next().unwrap()))
}

impl Piece {
    fn rotate_x(&mut self) {
        rotate(
            &mut self.top,
            &mut self.front,
            &mut self.bottom,
            &mut self.back,
        );
    }

    fn rotate_y(&mut self) {
        rotate(
            &mut self.front,
            &mut self.left,
            &mut self.back,
            &mut self.right,
        );
    }

    fn rotate_z(&mut self) {
        rotate(
            &mut self.left,
            &mut self.top,
            &mut self.right,
            &mut self.bottom,
        );
    }
}

impl<const LAYERS: usize> Cube<LAYERS> {
    fn x_layer(&mut self, layer: usize) -> CubeLayer<Piece, LAYERS> {
        CubeLayer(self.content[layer].each_mut().map(|x| x.each_mut()))
    }

    fn y_layer(&mut self, layer: usize) -> CubeLayer<Piece, LAYERS> {
        CubeLayer(swap_axes(
            self.content.each_mut().map(|x| x[layer].each_mut()),
        ))
    }

    fn z_layer(&mut self, layer: usize) -> CubeLayer<Piece, LAYERS> {
        CubeLayer(
            self.content
                .each_mut()
                .map(|x| x.each_mut().map(|x| &mut x[layer])),
        )
    }

    pub fn apply(&mut self, move_: Move) {
        match move_ {
            Move::X(layer) => self.x_layer(layer).rotate_x(),
            Move::Y(layer) => self.y_layer(layer).rotate_y(),
            Move::Z(layer) => self.z_layer(layer).rotate_z(),
        }
    }

    pub fn solved(&mut self) -> bool {
        fn all_eq<const LAYERS: usize>(
            layer: &mut CubeLayer<Piece, LAYERS>,
            field: impl Fn(&Piece) -> Color,
        ) -> bool {
            layer.0.as_flattened().windows(2).all(|w| {
                assert!(field(w[0]) != Color::Inside);
                assert!(field(w[1]) != Color::Inside);
                field(w[0]) == field(w[1])
            })
        }

        all_eq(&mut self.x_layer(0), |p| p.left)
            && all_eq(&mut self.x_layer(LAYERS - 1), |p| p.right)
            && all_eq(&mut self.y_layer(0), |p| p.top)
            && all_eq(&mut self.y_layer(LAYERS - 1), |p| p.bottom)
            && all_eq(&mut self.z_layer(0), |p| p.front)
            && all_eq(&mut self.z_layer(LAYERS - 1), |p| p.back)
    }
}

pub fn cube<const LAYERS: usize>(
    top: [[Color; LAYERS]; LAYERS],
    bottom: [[Color; LAYERS]; LAYERS],
    left: [[Color; LAYERS]; LAYERS],
    right: [[Color; LAYERS]; LAYERS],
    front: [[Color; LAYERS]; LAYERS],
    back: [[Color; LAYERS]; LAYERS],
) -> Cube<LAYERS> {
    fn color_face<const LAYERS: usize>(
        layer: CubeLayer<Piece, LAYERS>,
        face: [[Color; LAYERS]; LAYERS],
        side: impl Fn(&mut Piece) -> &mut Color,
    ) {
        layer
            .0
            .into_iter()
            .zip(face)
            .for_each(|(layer_col, face_col)| {
                layer_col
                    .into_iter()
                    .zip(face_col)
                    .for_each(|(piece, color)| {
                        *side(piece) = color;
                    })
            });
    }

    fn flip<const LAYERS: usize>(mut layer: CubeLayer<Piece, LAYERS>) -> CubeLayer<Piece, LAYERS> {
        layer.0.reverse();
        layer
    }

    let top = swap_axes(top);
    let bottom = swap_axes(bottom);
    let left = swap_axes(left);
    let right = swap_axes(right);
    let front = swap_axes(front);
    let back = swap_axes(back);

    let mut cube = Cube {
        content: [[[Piece {
            top: Color::Inside,
            bottom: Color::Inside,
            left: Color::Inside,
            right: Color::Inside,
            front: Color::Inside,
            back: Color::Inside,
        }; LAYERS]; LAYERS]; LAYERS],
    };

    color_face(cube.x_layer(0), left, |p| &mut p.left);
    color_face(flip(cube.x_layer(LAYERS - 1)), right, |p| &mut p.right);

    color_face(cube.y_layer(0), top, |p| &mut p.top);
    color_face(flip(cube.y_layer(LAYERS - 1)), bottom, |p| &mut p.bottom);

    color_face(cube.z_layer(0), front, |p| &mut p.front);
    color_face(flip(cube.z_layer(LAYERS - 1)), back, |p| &mut p.back);

    cube
}

#[macro_export]
macro_rules! color {
    (w) => {
        $crate::Color::White
    };
    (b) => {
        $crate::Color::Blue
    };
    (r) => {
        $crate::Color::Red
    };
    (o) => {
        $crate::Color::Orange
    };
    (g) => {
        $crate::Color::Green
    };
    (y) => {
        $crate::Color::Yellow
    };
}

#[macro_export]
macro_rules! side {
    (
        [$(
            $($color:ident)*,
        )*]
    ) => {
        [$([$($crate::color!($color)),*]),*]
    };
}

#[macro_export]
macro_rules! cube {
    (
        top: $top:tt,
        bottom: $bottom:tt,
        left: $left:tt,
        right: $right:tt,
        front: $front:tt,
        back: $back:tt,
    ) => {
        cube(
            $crate::side!($top),
            $crate::side!($bottom),
            $crate::side!($left),
            $crate::side!($right),
            $crate::side!($front),
            $crate::side!($back),
        )
    };
}

pub fn solved<const LAYERS: usize>() -> Cube<LAYERS> {
    use Color::*;

    cube(
        [[White; LAYERS]; LAYERS],
        [[Yellow; LAYERS]; LAYERS],
        [[Red; LAYERS]; LAYERS],
        [[Orange; LAYERS]; LAYERS],
        [[Blue; LAYERS]; LAYERS],
        [[Green; LAYERS]; LAYERS],
    )
}

#[test]
fn solved_is_solved() {
    let mut solved = solved::<16>();
    assert!(solved.solved());
}

#[test]
fn one_move_x() {
    let mut cube = cube! {
        top: [
            b b,
            w w,
        ],
        bottom: [
            g g,
            y y,
        ],
        left: [
            r r,
            r r,
        ],
        right: [
            o o,
            o o,
        ],
        front: [
            y b,
            y b,
        ],
        back: [
            g w,
            g w,
        ],
    };

    cube.apply(Move::X(0));

    assert!(cube.solved());
}

#[test]
fn one_move_y() {
    let mut cube = cube! {
        top: [
            w w,
            w w,
        ],
        bottom: [
            y y,
            y y,
        ],
        left: [
            b o,
            b o,
        ],
        right: [
            r g,
            r g,
        ],
        front: [
            o o,
            g g,
        ],
        back: [
            r r,
            b b,
        ],
    };

    dbg!(&cube);
    dbg!(cube.y_layer(0));

    cube.apply(Move::Y(0));

    dbg!(&cube);

    assert!(cube.solved());
}

#[test]
fn test_layer_rotation() {
    // This is supposed to be:
    //
    // 00 01 02 03 04 05 06
    // 07 08 09 10 11 12 13
    // 14 15 16 17 18 19 20
    // 21 22 23 24 25 26 27
    // 28 29 30 31 32 33 34
    // 35 36 37 38 39 40 41
    // 42 43 44 45 46 47 48
    //
    // This is a bit awkward because I decided to make the outer array the x axis and the inner one
    // y. That way you can index with [x][y], but it makes array litterals weird.
    let mut layer = [
        [00, 07, 14, 21, 28, 35, 42],
        [01, 08, 15, 22, 29, 36, 43],
        [02, 09, 16, 23, 30, 37, 44],
        [03, 10, 17, 24, 31, 38, 45],
        [04, 11, 18, 25, 32, 39, 46],
        [05, 12, 19, 26, 33, 40, 47],
        [06, 13, 20, 27, 34, 41, 48],
    ];

    {
        let mut layer: CubeLayer<u32, 7> = CubeLayer(layer.each_mut().map(|x| x.each_mut()));
        layer.rotate();
    }

    assert_eq!(
        layer,
        [
            [42, 43, 44, 45, 46, 47, 48],
            [35, 36, 37, 38, 39, 40, 41],
            [28, 29, 30, 31, 32, 33, 34],
            [21, 22, 23, 24, 25, 26, 27],
            [14, 15, 16, 17, 18, 19, 20],
            [07, 08, 09, 10, 11, 12, 13],
            [00, 01, 02, 03, 04, 05, 06],
        ]
    );
}
