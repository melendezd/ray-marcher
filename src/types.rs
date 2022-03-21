// Trait for converting data structures for uploading to the GPU
pub trait GPUFormat {
    type GPUType;
    fn gpu_format(&self) -> Self::GPUType;
}

// Trait for handling caching data
pub trait HasCache {
    fn dirty(&self) -> bool;
    fn update_cache(&mut self);
}

// Dimensions
pub type Dimension3 = (usize, usize, usize);

pub type Dimension2 = (usize, usize);

pub type Idx3 = (usize, usize, usize);
pub type Color = (u8, u8, u8);
