#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Block(u8);

impl Block {
    pub const AIR: Self = Self(0);
    pub const STONE: Self = Self(1);
    pub const GRASS: Self = Self(2);
    pub const DIRT: Self = Self(3);
}
