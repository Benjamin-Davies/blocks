use blocks_game::terrain::{block::Block, chunk::Chunk, subchunk::Subchunk, Terrain};
use glam::{ivec2, ivec3, IVec2, IVec3};

pub struct ChunkNeighborhood<'a> {
    chunk_pos: IVec2,
    center: &'a Chunk,
    west: Option<&'a Chunk>,
    east: Option<&'a Chunk>,
    north: Option<&'a Chunk>,
    south: Option<&'a Chunk>,
}

pub struct SubchunkNeighborhood<'a> {
    pub subchunk_pos: IVec3,
    center: &'a Subchunk,
    west: Option<&'a Subchunk>,
    east: Option<&'a Subchunk>,
    top: Option<&'a Subchunk>,
    bottom: Option<&'a Subchunk>,
    north: Option<&'a Subchunk>,
    south: Option<&'a Subchunk>,
}

impl SubchunkNeighborhood<'_> {
    pub fn is_dirty(&self) -> bool {
        self.center.dirty
            || self.west.map_or(false, |c| c.dirty)
            || self.east.map_or(false, |c| c.dirty)
            || self.top.map_or(false, |c| c.dirty)
            || self.bottom.map_or(false, |c| c.dirty)
            || self.north.map_or(false, |c| c.dirty)
            || self.south.map_or(false, |c| c.dirty)
    }

    pub fn block(&self, pos: IVec3) -> Block {
        match (
            pos.x.div_euclid(16),
            pos.y.div_euclid(16),
            pos.z.div_euclid(16),
        ) {
            (0, 0, 0) => self
                .center
                .block(pos.x as usize, pos.y as usize, pos.z as usize),
            (1, 0, 0) => self.west.map_or(Block::AIR, |c| {
                c.block((pos.x - 16) as usize, pos.y as usize, pos.z as usize)
            }),
            (-1, 0, 0) => self.east.map_or(Block::AIR, |c| {
                c.block((pos.x + 16) as usize, pos.y as usize, pos.z as usize)
            }),
            (0, 1, 0) => self.top.map_or(Block::AIR, |c| {
                c.block(pos.x as usize, (pos.y - 16) as usize, pos.z as usize)
            }),
            (0, -1, 0) => self.bottom.map_or(Block::AIR, |c| {
                c.block(pos.x as usize, (pos.y + 16) as usize, pos.z as usize)
            }),
            (0, 0, 1) => self.north.map_or(Block::AIR, |c| {
                c.block(pos.x as usize, pos.y as usize, (pos.z - 16) as usize)
            }),
            (0, 0, -1) => self.south.map_or(Block::AIR, |c| {
                c.block(pos.x as usize, pos.y as usize, (pos.z + 16) as usize)
            }),
            _ => Block::AIR,
        }
    }
}

pub trait ChunkNeighborhoods<'a> {
    fn chunk_neighborhoods(self) -> impl Iterator<Item = ChunkNeighborhood<'a>>;
}

pub trait SubchunkNeighborhoods<'a> {
    fn subchunk_neighborhoods(self) -> impl Iterator<Item = SubchunkNeighborhood<'a>>;
}

impl<'a, T> SubchunkNeighborhoods<'a> for T
where
    T: ChunkNeighborhoods<'a>,
{
    fn subchunk_neighborhoods(self) -> impl Iterator<Item = SubchunkNeighborhood<'a>> {
        self.chunk_neighborhoods()
            .flat_map(|c| c.subchunk_neighborhoods())
    }
}

impl<'a> ChunkNeighborhoods<'a> for &'a Terrain {
    fn chunk_neighborhoods(self) -> impl Iterator<Item = ChunkNeighborhood<'a>> {
        self.chunks.iter().map(|(&(x, z), center)| {
            let chunk_pos = ivec2(x, z);
            let west = self.chunks.get(&(x + 1, z));
            let east = self.chunks.get(&(x - 1, z));
            let north = self.chunks.get(&(x, z + 1));
            let south = self.chunks.get(&(x, z - 1));

            ChunkNeighborhood {
                chunk_pos,
                center,
                west,
                east,
                north,
                south,
            }
        })
    }
}

impl<'a> SubchunkNeighborhoods<'a> for ChunkNeighborhood<'a> {
    fn subchunk_neighborhoods(self) -> impl Iterator<Item = SubchunkNeighborhood<'a>> {
        self.center
            .subchunks
            .iter()
            .enumerate()
            .map(move |(y, center)| {
                let subchunk_pos = ivec3(self.chunk_pos.x, y as i32, self.chunk_pos.y);
                let west = self.west.and_then(|c| c.subchunks.get(y));
                let east = self.east.and_then(|c| c.subchunks.get(y));
                let north = self.north.and_then(|c| c.subchunks.get(y));
                let south = self.south.and_then(|c| c.subchunks.get(y));
                let top = self.center.subchunks.get(y.wrapping_add(1));
                let bottom = self.center.subchunks.get(y.wrapping_sub(1));

                SubchunkNeighborhood {
                    subchunk_pos,
                    center,
                    west,
                    east,
                    top,
                    bottom,
                    north,
                    south,
                }
            })
    }
}
