use nalgebra::Point2;

use super::*;

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum ChunkCell {
    #[default]
    Unknown,
    Open,
    Closed,
    CloseToClose
}

impl ChunkCell{
    pub fn is_solid(&self) -> bool{
        *self == ChunkCell::Closed
    }

    pub fn is_unknown(&self) -> bool{
        *self == ChunkCell::Unknown
    }

    pub fn is_walkable(&self) -> bool{
        *self == ChunkCell::CloseToClose || *self == ChunkCell::Open
    }
}

pub struct Chunk {
    pub(super) index: ChunkIndex,
    // pub(super) dimensions: Rectangle,
    data: Box<[[Cell<ChunkCell>; CELLS_PER_ROW]; CELLS_PER_ROW]>,
}

impl Debug for Chunk{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Chunk").field("index", &self.index).field("data", &"[]").finish()
    }
}

impl Chunk {
    pub fn cells_per_row(&self) -> usize{
        CELLS_PER_ROW
    }

    fn point_to_cell_index(&self, point: Point) -> LocalCellIndex {
        let chunk_index = ChunkIndex::from_point(point);

        let local_x = (point.x - (chunk_index.x as f32 * CHUNK_WIDTH)).floor();
        let local_y = (point.y - (chunk_index.y as f32 * CHUNK_WIDTH)).floor();

        debug_assert!(
            local_x >= 0.0
                && point.y >= 0.0
                && local_x < CELLS_PER_ROW as f32
                && point.y < CELLS_PER_ROW as f32
        );

        LocalCellIndex(PointUInt::new(local_x as usize, local_y as usize))
    }

    pub fn iterate(&self,mut func: impl FnMut(nalgebra::Point2<f32>,(f32,f32),ChunkCell)){
        let mut offset = Point2::new(self.index.x as f32, self.index.y as f32);
        offset *= CHUNK_WIDTH;

        for (y,row) in self.data.iter().enumerate(){
            let y = y as f32 * CELL_WIDTH;
            for (x,cell) in row.iter().enumerate(){
                let x = x as f32 * CELL_WIDTH;
                func(offset + Point2::new(x,y).coords,(CELL_WIDTH,CELL_WIDTH),cell.get());
            }
        }
    }

    pub fn get_cell_from_point_integer(&self, point: LocalCellIndex) -> &Cell<ChunkCell> {
        &self.data[point.y as usize][point.x as usize]
    }

    pub(super) fn from_position(position: Point) -> Self {
        Self {
            index: ChunkIndex::from_point(position),
            // dimensions,
            data: Box::new(std::array::from_fn(|_| {
                std::array::from_fn(|_| Default::default())
            })),
        }
    }

    pub(super) fn from_index(index: ChunkIndex) -> Self {
        Self {
            index,
            // dimensions,
            data: Box::new(std::array::from_fn(|_| {
                std::array::from_fn(|_| Default::default())
            })),
        }
    }


    pub fn get_cell_local(&self, pos: LocalCellIndex) -> &Cell<ChunkCell> {
        &self.data[pos.y][pos.x]
    }

    pub fn build_contour<'a>(&'a self) -> impl Iterator<Item = Box<[Point]>> + 'a {
        let mut result = Vec::with_capacity(CELLS_PER_ROW * CELLS_PER_ROW);

        for rows in self.data.iter() {
            for cell in rows {
                match cell.get() {
                    ChunkCell::Closed => result.push(1.0),
                    _=>result.push(0.0)
                }
            }
        }

        let c = contour::ContourBuilder::new(CELLS_PER_ROW as u32, CELLS_PER_ROW as u32, false);
        let res = c
            .contours(&result, &[0.5])
            .unwrap()
            .into_iter()
            .map(|c| c.into_inner().0)
            .flatten()
            .map(|p| {
                let coords = p.exterior().0.iter().map(|coord| {
                    LocalCellIndex(PointUInt::new(coord.x as usize, coord.y as usize))
                        .into_global(self)
                        .into_point()
                });

                let res = coords.collect::<Box<[_]>>();
                res
            });
        res
    }
}
