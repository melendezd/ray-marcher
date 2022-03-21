use glium::texture::{
    unsigned_texture3d::UnsignedTexture3d, ClientFormat, MipmapsOption, RawImage3d,
    UncompressedUintFormat,
};


use crate::march::DenseBinaryCartesianSDF;
use crate::types::GPUFormat;
use crate::world::{DenseGrid, Voxel, WORLD_DIM};

use std::borrow::Cow;

pub trait AsGPUResource {
    type GPUResourceT;
    fn as_gpu_resource(&self, facade: &dyn glium::backend::Facade) -> Self::GPUResourceT;
}

impl AsGPUResource for DenseBinaryCartesianSDF {
    type GPUResourceT = UnsignedTexture3d;
    fn as_gpu_resource(&self, facade: &dyn glium::backend::Facade) -> UnsignedTexture3d {
        let sdf_raw = self.gpu_format();
        let sdf_image = RawImage3d::<'_, u8> {
            data: Cow::from(sdf_raw),
            width: WORLD_DIM.0 as u32,
            height: WORLD_DIM.1 as u32,
            depth: WORLD_DIM.2 as u32,
            format: ClientFormat::U8,
        };

        UnsignedTexture3d::with_format(
            facade,
            sdf_image,
            UncompressedUintFormat::U8,
            MipmapsOption::NoMipmap,
        )
        .unwrap()
    }
}

impl AsGPUResource for DenseGrid<Voxel> {
    type GPUResourceT = UnsignedTexture3d;
    fn as_gpu_resource(&self, facade: &dyn glium::backend::Facade) -> UnsignedTexture3d {
        let level_raw = self.gpu_format();
        let level_image = RawImage3d::<'_, u8> {
            // TODO: figure out why this works
            data: Cow::Borrowed(&level_raw),
            width: WORLD_DIM.0 as u32,
            height: WORLD_DIM.1 as u32,
            depth: WORLD_DIM.2 as u32,
            format: ClientFormat::U8,
        };

        UnsignedTexture3d::with_format(
            facade,
            level_image,
            UncompressedUintFormat::U8,
            MipmapsOption::NoMipmap,
        )
        .unwrap()
    }
}
