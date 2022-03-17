use crate::march::{DenseBinaryCartesianSDF, SDF};
use crate::types::{Dimension3, Idx3};

use std::ops::{Deref, DerefMut, Index, IndexMut};

// Size of the world in voxels
pub const WORLD_DIM: Dimension3 = (128, 32, 128);
pub const WORLD_NUM_VOXELS: usize = WORLD_DIM.0 * WORLD_DIM.1 * WORLD_DIM.2;

/*************/
/* DenseGrid */
/*************/

pub struct DenseGrid {
    shape: Dimension3,
    grid: Box<[u8]>,
}

impl DenseGrid {
    pub fn zeros((x, y, z): Dimension3) -> DenseGrid {
        DenseGrid {
            shape: (x, y, z),
            grid: vec![0; WORLD_NUM_VOXELS].into_boxed_slice(),
        }
    }

    pub fn shape(&self) -> &Dimension3 {
        &self.shape
    }

    pub fn grid(&self) -> &Box<[u8]> {
        &self.grid
    }
}

impl Index<Idx3> for DenseGrid {
    type Output = u8;
    fn index(&self, (x, y, z): Idx3) -> &Self::Output {
        return &self.grid[x + self.shape.0 * (y + self.shape.1 * z)];
    }
}

impl IndexMut<Idx3> for DenseGrid {
    fn index_mut(&mut self, (x, y, z): Idx3) -> &mut Self::Output {
        return &mut self.grid[x + self.shape.0 * (y + self.shape.1 * z)];
    }
}

impl Deref for DenseGrid {
    type Target = Box<[u8]>;
    fn deref(&self) -> &Self::Target {
        &self.grid
    }
}

impl DerefMut for DenseGrid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.grid
    }
}

/*********/
/* Space */
/*********/

pub struct Space {
    // Grid of voxel data for the world
    pub voxels: DenseGrid,

    // Distance field used to determine step sizes for ray marching
    pub sdf: DenseBinaryCartesianSDF,
}

impl Space {
    pub fn new(shape: Dimension3) -> Space {
        let mut voxels = DenseGrid::zeros(shape);
        let mut sdf = DenseBinaryCartesianSDF::zeros(shape);
        for x in 1..shape.0 {
            for y in 1..shape.1 {
                for z in 1..shape.2 {
                    let val = {
                        if x % 3 == 0 && y % 3 == 0 && z % 5 == 0 && x >= 10 {
                            1
                        } else {
                            0
                        }
                    };
                    voxels[(x, y, z)] = 1 - val;
                }
            }
        }
        sdf.update(&voxels);
        Space { voxels, sdf }
    }
}
