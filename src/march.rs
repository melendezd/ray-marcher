use crate::types::{Dimension3, GPUFormat, Idx3};
use crate::world::{DenseGrid, Voxel};

use std::ops::Index;

pub trait SDF {
    type CoordT;
    type WorldT;
    fn update(&mut self, world: &Self::WorldT);
}

pub struct DenseBinaryCartesianSDF(DenseGrid<u8>);

impl DenseBinaryCartesianSDF {
    pub fn zeros(shape: Dimension3) -> DenseBinaryCartesianSDF {
        DenseBinaryCartesianSDF(DenseGrid::fill(shape, 0))
    }
}

impl Index<Idx3> for DenseBinaryCartesianSDF {
    type Output = u8;
    fn index(&self, idx: Idx3) -> &Self::Output {
        &self.0[idx]
    }
}

impl SDF for DenseBinaryCartesianSDF {
    type CoordT = (usize, usize, usize);
    type WorldT = DenseGrid<Voxel>;

    fn update(&mut self, level: &Self::WorldT) {
        for x in 0..level.shape().0 {
            for y in 0..level.shape().1 {
                for z in 0..level.shape().2 {
                    let coord = (x, y, z) as (usize, usize, usize);
                    if level[coord].is_empty() {
                        self.0[coord] = 1;
                    } else {
                        self.0[coord] = 0;
                    }
                }
            }
        }
    }
}

impl<'a> GPUFormat for &'a DenseBinaryCartesianSDF {
    type GPUType = &'a [u8];
    fn gpu_format(&self) -> Self::GPUType {
        &*self.0.grid()
    }
}
