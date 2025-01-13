use types3d::*;

use crate::IndexFromCoord3D;

//  //  //  //  //  //  //  //
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct LightGrid {
    pub(crate) i_max: usize,
    pub(crate) j_max: usize,
    pub(crate) k_max: usize,
    pub(crate) ij_max: usize,
    pub(crate) size: usize,
}

impl LightGrid {
    pub fn new(i_max: usize, j_max: usize, k_max: usize) -> Self {
        let ij_max = i_max * j_max;
        Self {
            i_max,
            j_max,
            k_max,
            ij_max,
            size: ij_max * k_max,
        }
    }
}

impl IndexFromCoord3D for LightGrid {
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

        let result = i + (j + k * self.j_max) * self.i_max;
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

//  //  //  //  //  //  //  //
impl LightGrid {
    pub fn index_to_coord(&self, index: usize) -> Option<IJK> {
        if index >= self.size {
            return None;
        }
        let wo_k = index % self.ij_max;
        let i = wo_k % self.i_max;
        let j = (wo_k - i) / self.i_max;
        let k = (index - (i + j * self.i_max)) / self.ij_max;
        Some(IJK { i, j, k })
    }
}

//  //  //  //  //  //  //  //
//        TESTS             //
//  //  //  //  //  //  //  //
#[cfg(test)]
mod ijk_to_index {
    use super::*;

    #[test]
    fn simple_cube() {
        let i_max = 3;
        let j_max = 5;
        let k_max = 7;
        let grid = LightGrid::new(i_max, j_max, k_max);
        let mut index = 0;
        for k in 0..k_max {
            for j in 0..j_max {
                for i in 0..i_max {
                    let coords = IJK { i, j, k };
                    let try_index = grid.index_from(&coords);
                    assert!(try_index != None, "get None for {:?}", coords);
                    assert!(
                        try_index.unwrap() == index,
                        "get try_index={} for {:?} instead of {}",
                        try_index.unwrap(),
                        coords,
                        index
                    );

                    let restored = grid.index_to_coord(index);
                    assert!(restored.is_some(), "get restored=None for {}", index);
                    assert!(
                        restored.unwrap() == coords,
                        "get restored={:?} for index={} instead of {:?}",
                        restored,
                        index,
                        coords
                    );

                    index += 1;
                }
            }
        }
    }

    #[test]
    fn i_only() {
        for v_max in 0..5 {
            for v in 0..v_max {
                let grid = LightGrid::new(v_max, 1, 1);
                let coords = IJK { i: v, j: 0, k: 0 };
                let index = grid.index_from(&coords);
                assert!(index != None, "get None for v_max={}, v={}", v_max, v);
                assert!(
                    index.unwrap() == v,
                    "get index={} for v_max={}, v={}",
                    index.unwrap(),
                    v_max,
                    v
                );

                let restored = grid.index_to_coord(index.unwrap());
                assert!(
                    restored.is_some(),
                    "get restored=None for v_max={}, v={}",
                    v_max,
                    v
                );
                assert!(
                    restored.unwrap() == coords,
                    "get restored={:?} for v_max={}, v={} instead of {:?}",
                    restored,
                    v_max,
                    v,
                    coords
                );
            }
        }
    }
    #[test]
    fn j_only() {
        for v_max in 0..5 {
            for v in 0..v_max {
                let grid = LightGrid::new(1, v_max, 1);
                let coords = IJK { i: 0, j: v, k: 0 };
                let index = grid.index_from(&coords);
                assert!(index != None, "get None for v_max={}, v={}", v_max, v);
                assert!(index.unwrap() == v);

                let restored = grid.index_to_coord(index.unwrap());
                assert!(
                    restored.is_some(),
                    "get restored=None for v_max={}, v={}",
                    v_max,
                    v
                );
                assert!(
                    restored.unwrap() == coords,
                    "get restored={:?} for v_max={}, v={} instead of {:?}",
                    restored,
                    v_max,
                    v,
                    coords
                );
            }
        }
    }
    #[test]
    fn k_only() {
        for v_max in 0..5 {
            for v in 0..v_max {
                let grid = LightGrid::new(1, 1, v_max);
                let coords = IJK { i: 0, j: 0, k: v };
                let index = grid.index_from(&coords);
                assert!(index != None, "get None for v_max={}, v={}", v_max, v);
                assert!(index.unwrap() == v);

                let restored = grid.index_to_coord(index.unwrap());
                assert!(
                    restored.is_some(),
                    "get restored=None for v_max={}, v={}",
                    v_max,
                    v
                );
                assert!(
                    restored.unwrap() == coords,
                    "get restored={:?} for v_max={}, v={} instead of {:?}",
                    restored,
                    v_max,
                    v,
                    coords
                );
            }
        }
    }

    #[test]
    fn k_bounds_error() {
        let grid = LightGrid::new(1, 1, 1);
        let coords = IJK { i: 0, j: 0, k: 1 };
        assert!(grid.index_from(&coords) == None);
    }
    #[test]
    fn j_bounds_error() {
        let grid = LightGrid::new(1, 1, 1);
        let coords = IJK { i: 0, j: 1, k: 0 };
        assert!(grid.index_from(&coords) == None);
    }
    #[test]
    fn i_bounds_error() {
        let grid = LightGrid::new(1, 1, 1);
        let coords = IJK { i: 1, j: 0, k: 0 };
        assert!(grid.index_from(&coords) == None);
    }
}
