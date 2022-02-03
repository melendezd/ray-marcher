use crate::types::{ToGPU, Dimension3, HasCache};
use glium::texture::{
    unsigned_texture3d::UnsignedTexture3d, ClientFormat, MipmapsOption, RawImage3d,
    UncompressedUintFormat,
};
use glium::uniforms::{AsUniformValue, UniformValue};

use std::ops::{Deref, DerefMut};

use std::borrow::Cow;


// Size of the world in voxels
pub const WORLD_DIM: Dimension3 = Dimension3 {x: 128, y: 32, z: 128};

// Using a dense array for both VoxelGrid and SDF for now
type GridArray = [[[u8; WORLD_DIM.x]; WORLD_DIM.y]; WORLD_DIM.z];
pub struct WorldGrid {
    grid: GridArray,
    texture: UnsignedTexture3d,
    // Dirty bit used to determine whether the texture should be updated
    dirty: bool
}
type VoxelGrid = WorldGrid;
type SDF = WorldGrid;

// Deref to GridArray so we can modify the world grid directly
impl Deref for WorldGrid {
    type Target = GridArray;
    fn deref(&self) -> &Self::Target {
        return &self.grid;
    }
}

impl DerefMut for WorldGrid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.dirty = true;
        return &mut self.grid;
    }
}

/*
impl GPUResource for WorldGrid {
    type GPUType = UnsignedTexture3d;
    type CPUType = GridArray;

    fn upload(&mut self, resource: Self::GPUType) {

    }

    fn get_gpu_resource(&self) -> &Self::GPUType {
        return &self.texture;
    }
}
*/

/*
impl HasCache for WorldGrid {
    fn dirty(&self) -> bool {
        return self.dirty;
    }

    fn update_cache(&mut self) {
        // Create 3D image to transfer SDF to GPU
        let sdf_data = self.grid.to_gpu();

        let sdf_image = RawImage3d::<'_, u8> {
            data: Cow::from(&sdf_data[..]),
            width: WORLD_DIM.x as u32,
            height: WORLD_DIM.y as u32,
            depth: WORLD_DIM.z as u32,
            format: ClientFormat::U8,
        };

       self.texture = UnsignedTexture3d::with_format(
            display,
            sdf_image,
            UncompressedUintFormat::U8,
            MipmapsOption::NoMipmap,
        ).unwrap()
    }
}
*/

impl AsUniformValue for &WorldGrid {
    fn as_uniform_value(&self) -> UniformValue {
        UniformValue::UnsignedTexture3d(&self.texture, None)
    }
}

// Implements ToGPU for 3D u8 array
impl ToGPU for GridArray {
    type GPUType = [u8; WORLD_DIM.x * WORLD_DIM.y * WORLD_DIM.z];
    fn to_gpu(&self) -> Self::GPUType {
        unsafe {
            std::mem::transmute::<GridArray, Self::GPUType>(*self)
        }
    }
}

pub struct World {
    // Grid of voxel data for the world
    pub voxels: Box<VoxelGrid>,

    // Distance field used to determine step sizes for ray marching
    pub sdf: Box<SDF>,
}

impl World {
    pub fn new(display: &glium::Display) -> World {
        [[[0; WORLD_DIM.x]; WORLD_DIM.y]; WORLD_DIM.z].iter_mut();
        // For now, initialize voxel grid and SDF simultaneously for testing purposes
        // TODO: Load in Magi
        let mut world = World {
            voxels: Box::new({
                let mut grid = [[[0; WORLD_DIM.x]; WORLD_DIM.y]; WORLD_DIM.z];
                for (z, xy_slice) in grid.iter_mut().enumerate() {
                    for (y, x_slice) in xy_slice.iter_mut().enumerate() {
                        for (x, elem) in x_slice.iter_mut().enumerate() {
                            if x % 4 == 0 && y % 4 == 0 && z % 2 == 0 && z < 100 {
                                *elem = 1;
                            } else {
                                *elem = 0
                            }
                        }
                    }
                }
                WorldGrid {
                    dirty: true,
                    grid: grid,
                    texture: {
                        let sdf_data = grid.to_gpu();
                        let sdf_image = RawImage3d::<'_, u8> {
                            data: Cow::from(&sdf_data[..]),
                            width: WORLD_DIM.x as u32,
                            height: WORLD_DIM.y as u32,
                            depth: WORLD_DIM.z as u32,
                            format: ClientFormat::U8,
                        };

                       UnsignedTexture3d::with_format(
                            display,
                            sdf_image,
                            UncompressedUintFormat::U8,
                            MipmapsOption::NoMipmap,
                        ).unwrap()
                    }
                }
            }),
            sdf: Box::new({
                let mut grid = [[[0; WORLD_DIM.x]; WORLD_DIM.y]; WORLD_DIM.z];
                for (z, xy_slice) in grid.iter_mut().enumerate() {
                    for (y, x_slice) in xy_slice.iter_mut().enumerate() {
                        for (x, elem) in x_slice.iter_mut().enumerate() {
                            if x % 4 == 0 && y % 4 == 0 && z % 2 == 0 && z < 100 {
                                *elem = 0;
                            } else {
                                *elem = 1
                            }
                        }
                    }
                }
                WorldGrid {
                    dirty: true,
                    grid: grid,
                    texture: {
                        let sdf_data = grid.to_gpu();
                        let sdf_image = RawImage3d::<'_, u8> {
                            data: Cow::from(&sdf_data[..]),
                            width: WORLD_DIM.x as u32,
                            height: WORLD_DIM.y as u32,
                            depth: WORLD_DIM.z as u32,
                            format: ClientFormat::U8,
                        };

                       UnsignedTexture3d::with_format(
                            display,
                            sdf_image,
                            UncompressedUintFormat::U8,
                            MipmapsOption::NoMipmap,
                        ).unwrap()
                    }
                }
            }),
        };

        /*
        for (z, xy_slice) &n world.voxels.iter_mut().enumerate() {
            for (y, x_slice) in xy_slice.iter_mut().enumerate() {
                for (x, elem) in x_slice.iter_mut().enumerate() {
                    if x % 4 == 0 && y % 4 == 0 && z % 2 == 0 && z < 100 {
                        *elem = 1;
                        world.sdf[x][y][z] = 0;
                    } else {
                        world.sdf[x][y][z] = 1;
                    }
                }
            }
        }
        */

        world
    }
}
