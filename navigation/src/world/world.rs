use super::*;

#[derive(Debug)]
pub struct World {
    last_used: Rc<Chunk>, //Works as a cache, nearby calls into an chunk can get reused instead of search in "chunks"
    chunks: Vec<Rc<Chunk>>,
}

impl Default for World {
    fn default() -> Self {
        let start_chunk = Rc::new(Chunk::from_position(Default::default()));
        let this = Self {
            chunks: vec![start_chunk.clone()],
            last_used: start_chunk,
        };
        this
    }
}




impl World {
    #[inline]
    fn get_chunk_from_index(
        &mut self,
        pos: ChunkIndex,
    ) -> &Rc<Chunk> {
        if pos == self.last_used.index {
            return &self.last_used;
        }

        if let Some(w) = self.chunks.iter().position(|c| pos == c.index) {
            return &self.chunks[w];
        } else {
            let chunk = Rc::new(Chunk::from_index(pos));
            self.chunks.push(chunk.clone());
            self.last_used = chunk;
            &self.last_used
        }
    }

    fn get_chunk_if_exist(&self, pos: ChunkIndex) -> Option<&Rc<Chunk>>{
        if pos == self.last_used.index {
            return Some(&self.last_used);
        }
        self.chunks.iter().find(|c| pos == c.index)
    }

    pub fn get_cell<T: CellAdress>(&self,pos: T) -> ChunkCell{
        if let Some(s) = self.get_chunk_if_exist(pos.into_chunkindex()){
            s.get_cell_local(pos.into_local_cellindex()).get()
        }else{
            ChunkCell::Unknown
        }
    }

    pub fn get_cell_mut<T: CellAdress>(&mut self, pos: T) -> &Cell<ChunkCell> {
        let chunk = self.get_chunk_from_index(pos.into_chunkindex());
        chunk.get_cell_local(pos.into_local_cellindex())
    }

    pub fn get_chunks(&self) -> &Vec<Rc<Chunk>> {
        &self.chunks
    }
}
