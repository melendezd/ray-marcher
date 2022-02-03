// Trait for converting data structures for uploading to the GPU
pub trait ToGPU {
    type GPUType;
    fn to_gpu(&self) -> Self::GPUType;
}

// Trait for handling caching data
pub trait HasCache {
    fn dirty(&self) -> bool;
    fn update_cache(&mut self);
}

// Trait for resources residing on the GPU
/*
pub trait GPUResource {
    type 
}
*/

// Dimensions
pub struct Dimension3 {
    pub x: usize,
    pub y: usize,
    pub z: usize
}

pub struct Dimension2 {
    pub x: usize,
    pub y: usize,
}
