use crate::march::{DenseBinaryCartesianSDF, SDF};
use crate::types::{Color, Dimension3, GPUFormat, Idx3};

use std::borrow::Cow;
use std::default::Default;
use std::ops::{Deref, DerefMut, Index, IndexMut};

// Size of the world in voxels
pub const WORLD_DIM: Dimension3 = (128, 32, 128);
pub const WORLD_NUM_VOXELS: usize = WORLD_DIM.0 * WORLD_DIM.1 * WORLD_DIM.2;

use std::io::Read;

/*************/
/* Voxel */
/*************/
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Voxel {
    Empty = 0,
    Red = 1,
    Blue = 2,
    Green = 3,
}

impl Default for Voxel {
    fn default() -> Self {
        Voxel::Empty
    }
}

impl Voxel {
    pub fn from_color(color: Color) -> Voxel {
        match color {
            (0xff, 0x00, 0x00) => Voxel::Red,
            (0x00, 0xff, 0x00) => Voxel::Green,
            (0x00, 0x00, 0xff) => Voxel::Blue,
            _ => Voxel::Empty,
        }
    }

    pub fn is_empty(&self) -> bool {
        return *self == Voxel::Empty;
    }

    pub fn id(&self) -> u8 {
        return *self as u8;
    }
}

/*************/
/* DenseGrid */
/*************/

pub struct DenseGrid<T> {
    shape: Dimension3,
    grid: Box<[T]>,
}

impl<T: Copy> DenseGrid<T> {
    pub fn fill((x, y, z): Dimension3, val: T) -> DenseGrid<T> {
        DenseGrid {
            shape: (x, y, z),
            grid: vec![val; WORLD_NUM_VOXELS].into_boxed_slice(),
        }
    }

    pub fn shape(&self) -> &Dimension3 {
        &self.shape
    }

    pub fn grid(&self) -> &Box<[T]> {
        &self.grid
    }
}

impl<T> Index<Idx3> for DenseGrid<T> {
    type Output = T;
    fn index(&self, (x, y, z): Idx3) -> &Self::Output {
        return &self.grid[x + self.shape.0 * (y + self.shape.1 * z)];
    }
}

impl<T> IndexMut<Idx3> for DenseGrid<T> {
    fn index_mut(&mut self, (x, y, z): Idx3) -> &mut Self::Output {
        return &mut self.grid[x + self.shape.0 * (y + self.shape.1 * z)];
    }
}

impl<T> Deref for DenseGrid<T> {
    type Target = Box<[T]>;
    fn deref(&self) -> &Self::Target {
        &self.grid
    }
}

impl<T> DerefMut for DenseGrid<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.grid
    }
}

impl<'a> GPUFormat for &'a DenseGrid<Voxel> {
    type GPUType = Box<[u8]>;
    fn gpu_format(&self) -> Self::GPUType {
        Box::from(self.grid.into_iter().map(Voxel::id).collect::<Vec<_>>())
    }
}

/*********/
/* Space */
/*********/

pub struct Space {
    // Grid of voxel data for the world
    pub voxels: DenseGrid<Voxel>,

    // Distance field used to determine step sizes for ray marching
    pub sdf: DenseBinaryCartesianSDF,
}

impl Space {
    pub fn new(shape: Dimension3) -> Self {
        let mut voxels = DenseGrid::fill(shape, Voxel::Empty);
        let mut sdf = DenseBinaryCartesianSDF::zeros(shape);
        for x in 1..shape.0 {
            for y in 1..shape.1 {
                for z in 1..shape.2 {
                    voxels[(x, y, z)] = {
                        if x % 3 == 0 && y % 3 == 0 && z % 5 == 0 && x >= 10 {
                            Voxel::Red
                        } else {
                            Voxel::Empty
                        }
                    };
                }
            }
        }
        sdf.update(&voxels);
        Space { voxels, sdf }
    }

    pub fn from_gox(shape: Dimension3, src: &mut dyn Read) -> Space {
        // goxel format consists of in each line, either a comment beginning with # or a voxel of the format:
        // "posX posY posZ color"
        // example: "111 78 36 ff00ff"

        let mut src_str: String = "".to_string();
        src.read_to_string(&mut src_str).unwrap();
        let lines = src_str.lines();

        // determines if a line is not a comment
        fn is_not_comment(line: &str) -> bool {
            let char = line.chars().next();
            println!("first char of line {} is {:?}", line, char);
            return line.chars().next() != Some('#');
        }

        // parses a hex string into a Color
        fn parse_hex_str(hex_str: &str) -> Option<Color> {
            // if successful, this is a vector containing integers (0 - 255) representing RGB value
            // of the color
            let vals = (0..hex_str.len())
                .step_by(2)
                .map(|i| usize::from_str_radix(&hex_str[i..i + 2], 16))
                .collect::<Vec<_>>();
            Some((
                *vals.get(0).unwrap().as_ref().unwrap() as u8,
                *vals.get(1).unwrap().as_ref().unwrap() as u8,
                *vals.get(2).unwrap().as_ref().unwrap() as u8,
            ))
        }

        // parses a line from gox file into a voxel with a position
        fn process_line(line: &str) -> Option<(Idx3, Voxel)> {
            let tokens = line.split_whitespace().collect::<Vec<&str>>();
            let pos: Idx3 = (
                tokens.get(0).unwrap().parse::<usize>().unwrap(),
                tokens.get(1).unwrap().parse::<usize>().unwrap(),
                tokens.get(2).unwrap().parse::<usize>().unwrap(),
            );
            let color = parse_hex_str(tokens.get(3).unwrap()).unwrap();
            let vox = Voxel::from_color(color);
            println!("{:?} -> {:?}", color, vox);
            Some((pos, vox))
        }

        let mut voxels = DenseGrid::fill(shape, Voxel::Empty);
        let mut sdf = DenseBinaryCartesianSDF::zeros(shape);

        // TODO: make this fail if there are any Nones
        // loop through all valid voxel lines and add them to voxel grid
        for (pos, vox) in lines
            .filter(|x| is_not_comment(&x))
            .map(process_line)
            .filter(Option::is_some)
            .map(Option::unwrap)
        {
            voxels[pos] = vox;
        }
        sdf.update(&voxels);

        Space { voxels, sdf }
    }
}
