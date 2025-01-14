use types3d::*;

use crate::IndexFromCoord3D;
use crate::LightGrid;

//  //  //  //  //  //  //  //
#[derive(Debug, PartialEq)]
pub struct CachedGrid {
    pub(crate) i_max: usize,
    pub(crate) j_max: usize,
    pub(crate) k_max: usize,
    pub(crate) size: usize,
    pub(crate) j_cache: Box<[usize]>,
    pub(crate) k_cache: Box<[usize]>,
}

impl CachedGrid {
    pub fn new_rc(i_max: usize, j_max: usize, k_max: usize) -> std::rc::Rc<Self> {
        std::rc::Rc::new(Self::new(i_max, j_max, k_max))
    }
    pub fn new(i_max: usize, j_max: usize, k_max: usize) -> Self {
        let grid_calculator = LightGrid::new(i_max, j_max, k_max);

        grid_calculator.into()
    }
}

impl From<LightGrid> for CachedGrid {
    fn from(src: LightGrid) -> Self {
        let LightGrid {
            i_max,
            j_max,
            k_max,
            size,
            ..
        } = src;
        let j_cache = create_j_cache(&src);
        let k_cache = create_k_cache(&src);
        Self {
            i_max,
            j_max,
            k_max,
            size,
            j_cache,
            k_cache,
        }
    }
}

impl IndexFromCoord3D for CachedGrid {
    fn index_from(&self, coord: &IJK) -> Option<usize> {
        let IJK { i, j, k } = *coord;
        if i >= self.i_max {
            return None;
        }
        if j >= self.j_max {
            return None;
        }
        if k >= self.k_max {
            return None;
        }

        let result = i + self.j_cache[j] + self.k_cache[k];
        Some(result)
    }

    fn size(&self) -> usize {
        self.size
    }
    fn i_max(&self) -> usize {
        self.i_max
    }
    fn j_max(&self) -> usize {
        self.j_max
    }
    fn k_max(&self) -> usize {
        self.k_max
    }
}

//let result = i + (j + k * self.j_max) * self.i_max;
//  //  //  //  //  //  //  //
fn create_j_cache(src: &LightGrid) -> Box<[usize]> {
    let mut cash = Vec::<usize>::with_capacity(src.j_max());

    for j in 0..src.j_max() {
        cash.push(j * src.i_max());
    }

    cash.into_boxed_slice()
}

fn create_k_cache(src: &LightGrid) -> Box<[usize]> {
    let mut cash = Vec::<usize>::with_capacity(src.k_max());

    for k in 0..src.k_max() {
        cash.push(k * src.i_max() * src.j_max());
    }

    cash.into_boxed_slice()
}

//  //  //  //  //  //  //  //
//        TESTS             //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod fast_coord_grid {
    use super::*;

    #[test]
    fn index_comparation() {
        let c_grid = LightGrid::new(13, 57, 29);
        let f_grid = CachedGrid::from(c_grid.clone());

        for i in 0..f_grid.i_max() {
            for j in 0..f_grid.j_max() {
                for k in 0..f_grid.k_max() {
                    let coord = IJK { i, j, k };
                    assert!(c_grid.index_from(&coord) == f_grid.index_from(&coord));
                    //
                }
            }
        }

        for i in 0..c_grid.i_max() {
            for j in 0..c_grid.j_max() {
                for k in 0..c_grid.k_max() {
                    let coord = IJK { i, j, k };
                    assert!(c_grid.index_from(&coord) == f_grid.index_from(&coord));
                    //
                }
            }
        }
    }

    #[test]
    fn create_from() {
        let c_grid = LightGrid::new(3, 5, 7);
        let f_grid = CachedGrid::from(c_grid.clone());

        assert!(f_grid.i_max() == c_grid.i_max());
        assert!(f_grid.j_max() == c_grid.j_max());
        assert!(f_grid.k_max() == c_grid.k_max());
        assert!(f_grid.size() == c_grid.size());
    }
    #[test]
    fn internal_limits() {
        let grid = CachedGrid::new(3, 5, 7);

        assert!(grid.j_max() == grid.j_cache.len());
        assert!(grid.k_max() == grid.k_cache.len());
    }
}
