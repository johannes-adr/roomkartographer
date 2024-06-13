mod chunk;
pub use chunk::*;

mod world;
use line_drawing::BresenhamCircle;
pub use world::*;



use std::{cell::Cell, fmt::Debug, rc::Rc};

pub type Point = nalgebra::Point2<f32>;
pub type PointInt = nalgebra::Point2<isize>;
pub type PointUInt = nalgebra::Point2<usize>;



// const CELL_WIDTH: f32 = 0.03125; //3,125 cm
// const CHUNK_WIDTH: f32 = 1.0; //< 2!
// const CELLS_PER_ROW: usize = (32.0 * CHUNK_WIDTH) as usize; //sqrt(PAGE_SIZE)

pub const CELL_WIDTH: f32 = 0.03125 * 1.0; //3,125 cm
pub const CHUNK_WIDTH: f32 = 1.0 / 4.0; //< 2!
pub const CELLS_PER_ROW: usize = (CHUNK_WIDTH / CELL_WIDTH) as usize; //sqrt(PAGE_SIZE)

use nalgebra::{vector as v2, Vector2};
#[derive(derive_more::Deref, Clone, Copy)]
pub struct LocalCellIndex(PointUInt);

impl LocalCellIndex {
    pub fn into_global(self, chunk: &Chunk) -> GlobalCellIndex {
        // let local_xf32 = self.x as f32 * CELL_WIDTH;
        // let local_yf32 = self.y as f32 * CELL_WIDTH;
        GlobalCellIndex(PointInt::new(
            self.x as isize + chunk.index.x * CELLS_PER_ROW as isize,
            self.y as isize + chunk.index.y * CELLS_PER_ROW as isize,
        ))
    }
}

#[derive(Clone, Copy, PartialEq, Eq,Debug)]
pub enum Direction {
    // TopLeft,
    Top,
    // TopRight,
    Left,
    Right,
    // BotLeft,
    Bot,
    // BotRight,
}



impl Direction {
    pub fn turn_left(&mut self){
        *self = match self{
            Self::Top => Self::Left,
            Self::Right => Self::Top,
            Self::Bot => Self::Right,
            Self::Left => Self::Bot,
            _=>panic!()
        }
    }

    pub fn turn_right(&mut self){
        *self = match self{
            Self::Top => Self::Right,
            Self::Right => Self::Bot,
            Self::Bot => Self::Left,
            Self::Left => Self::Top,
            _=>panic!()
        }
    }

    pub const fn is_diagonal(&self) -> bool{
        match *self{
            Self::Top | Self::Left | Self::Bot | Self::Right => false,
            _=>true
        }
    }

    pub const fn as_vector(&self) -> nalgebra::Vector2<i8> {
        match *self {
            Direction::Top => v2![0, 1],

            Direction::Left => v2![-1, 0],
            Direction::Right => v2![1, 0],
            Direction::Bot => v2![0, -1],
            // Direction::BotLeft => v2![-1, -1],
            // Direction::TopRight => v2![1, 1],
            // Direction::TopLeft => v2![-1i8, 1i8],
            // Direction::BotRight => v2![1, -1],
        }
    }

    pub const fn slice() -> [(Direction, Vector2<i8>); 4] {
        [
            (Direction::Top, Self::Top.as_vector()),
            (Direction::Left, Self::Left.as_vector()),
            (Direction::Right, Self::Right.as_vector()),
            (Direction::Bot, Self::Bot.as_vector()),
            // (Direction::BotRight, Self::BotRight.as_vector()),
            // (Direction::BotLeft, Self::BotLeft.as_vector()),
            // (Direction::TopRight, Self::TopRight.as_vector()),
            // (Direction::TopLeft, Self::TopLeft.as_vector()),
        ]

        // [(-1,-1,Direction::TopLeft),(0,-1,Direction::Top),(1,-1,Direction::TopRight),  (-1,0,Direction::Left),(1,0,Direction::Right),   (-1,1,Direction::BotLeft),(0,1,Direction::Bot),(1,1,Direction::BotRight)]
    }

    pub fn away_score(&self, other: Direction) -> i8 {
        self.as_vector().dot(&other.as_vector())
    }
}

#[derive(derive_more::Deref, derive_more::DerefMut, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct GlobalCellIndex(pub PointInt);

impl GlobalCellIndex {
    #[inline]
    pub fn neighbours(&self) -> [(GlobalCellIndex, Direction); 4] {
        let res = Direction::slice().map(|(direction, vec)| {
            let mut pos = *self;
            pos.x += vec.x as isize;
            pos.y += vec.y as isize;
            (pos, direction)
        });
        res
    }

    #[inline]
    pub fn neighbour(&self, direction: Direction) -> GlobalCellIndex{
        let mut cpy = *self;
        let dirvec = direction.as_vector();
        cpy.x += dirvec.x as isize;
        cpy.y += dirvec.y as isize;
        cpy
    }

    pub fn radius(&self, radius: isize) -> impl Iterator<Item = GlobalCellIndex> {
        BresenhamCircle::new(self.x, self.y, radius).map(|p|{
            GlobalCellIndex(PointInt::new(p.0,p.1))
        })
    }

    #[inline]
    pub fn manhatten_distance(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    pub fn from_point(point: Point) -> Self {
        let cell_x = (point.x / CELL_WIDTH).floor() as isize;
        let cell_y = (point.y / CELL_WIDTH).floor() as isize;
        Self(PointInt::new(cell_x, cell_y))
    }

    pub fn into_point(self) -> Point {
        Point::new(self.x as f32 * CELL_WIDTH, self.y as f32 * CELL_WIDTH)
    }
}

#[derive(derive_more::Deref, Clone, Copy, Debug, PartialEq)]
pub struct ChunkIndex(PointInt);

impl ChunkIndex {
    #[inline]
    fn from_point(point: Point) -> Self {
        assert!(point.x.is_finite() && point.y.is_finite());
        let cell_x = (point.x / CHUNK_WIDTH).floor() as isize;
        let cell_y = (point.y / CHUNK_WIDTH).floor() as isize;
        Self(PointInt::new(cell_x, cell_y))
    }
}

pub trait CellAdress: Copy {
    fn into_chunkindex(&self) -> ChunkIndex;
    fn into_global_cellindex(&self) -> GlobalCellIndex;
    fn into_local_cellindex(&self) -> LocalCellIndex;
}

impl CellAdress for Point {
    #[inline]
    fn into_chunkindex(&self) -> ChunkIndex {
        ChunkIndex::from_point(*self)
    }

    #[inline]
    fn into_global_cellindex(&self) -> GlobalCellIndex {
        GlobalCellIndex::from_point(*self)
    }

    #[inline]
    fn into_local_cellindex(&self) -> LocalCellIndex {
        self.into_global_cellindex().into_local_cellindex()
    }
}
impl CellAdress for GlobalCellIndex {
    #[inline]
    fn into_chunkindex(&self) -> ChunkIndex {
        ChunkIndex(PointInt::new(
            (self.x as f32 / CELLS_PER_ROW as f32).floor() as isize,
            (self.y as f32 / CELLS_PER_ROW as f32).floor() as isize,
        ))
    }

    #[inline]
    fn into_global_cellindex(&self) -> GlobalCellIndex {
        *self
    }

    #[inline]
    fn into_local_cellindex(&self) -> LocalCellIndex {
        let local_x =
            ((self.x % CELLS_PER_ROW as isize) + CELLS_PER_ROW as isize) % CELLS_PER_ROW as isize;
        let local_y =
            ((self.y % CELLS_PER_ROW as isize) + CELLS_PER_ROW as isize) % CELLS_PER_ROW as isize;
        LocalCellIndex(PointUInt::new(local_x as usize, local_y as usize))
    }
}
