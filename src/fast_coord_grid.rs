use types3d::*;

use crate::GridCalculator;

//  //  //  //  //  //  //  //
#[derive(Debug, PartialEq)]
pub struct FastCoordGrid {
    pub i_max: usize,
    pub j_max: usize,
    pub k_max: usize,
    pub size: usize,
    j_cash: Box<[usize]>,
    k_cash: Box<[usize]>,
}

impl FastCoordGrid {
    pub fn new_rc(i_max: usize, j_max: usize, k_max: usize) -> std::rc::Rc<Self> {
        std::rc::Rc::new(Self::new(i_max, j_max, k_max))
    }
    pub fn new(i_max: usize, j_max: usize, k_max: usize) -> Self {
        let grid_calculator = GridCalculator::new(i_max, j_max, k_max);

        grid_calculator.into()
    }
}

impl From<GridCalculator> for FastCoordGrid {
    fn from(src: GridCalculator) -> Self {
        let GridCalculator {
            i_max,
            j_max,
            k_max,
            size,
            ..
        } = src;
        let j_cash = create_j_cash(&src);
        let k_cash = create_k_cash(&src);
        Self {
            i_max,
            j_max,
            k_max,
            size,
            j_cash,
            k_cash,
        }
    }
}

//  //  //  //  //  //  //  //
impl FastCoordGrid {
    pub fn index_from(&self, coord: &IJK) -> Option<usize> {
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

        let result = i + self.j_cash[j] + self.k_cash[k];
        Some(result)
    }
}

//let result = i + (j + k * self.j_max) * self.i_max;
//  //  //  //  //  //  //  //
fn create_j_cash(src: &GridCalculator) -> Box<[usize]> {
    let mut cash = Vec::<usize>::with_capacity(src.j_max);

    for j in 0..src.j_max {
        cash.push(j * src.i_max);
    }

    cash.into_boxed_slice()
}

fn create_k_cash(src: &GridCalculator) -> Box<[usize]> {
    let mut cash = Vec::<usize>::with_capacity(src.k_max);

    for k in 0..src.k_max {
        cash.push(k * src.i_max * src.j_max);
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
        let c_grid = GridCalculator::new(13, 57, 29);
        let f_grid = FastCoordGrid::from(c_grid.clone());

        for i in 0..f_grid.i_max {
            for j in 0..f_grid.j_max {
                for k in 0..f_grid.k_max {
                    let coord = IJK { i, j, k };
                    assert!(c_grid.coord_to_index(&coord) == f_grid.index_from(&coord));
                    //
                }
            }
        }

        for i in 0..c_grid.i_max {
            for j in 0..c_grid.j_max {
                for k in 0..c_grid.k_max {
                    let coord = IJK { i, j, k };
                    assert!(c_grid.coord_to_index(&coord) == f_grid.index_from(&coord));
                    //
                }
            }
        }
    }

    #[test]
    fn create_from() {
        let c_grid = GridCalculator::new(3, 5, 7);
        let f_grid = FastCoordGrid::from(c_grid.clone());

        assert!(f_grid.i_max == c_grid.i_max);
        assert!(f_grid.j_max == c_grid.j_max);
        assert!(f_grid.k_max == c_grid.k_max);
        assert!(f_grid.size == c_grid.size);
    }
    #[test]
    fn internal_limits() {
        let grid = FastCoordGrid::new(3, 5, 7);

        assert!(grid.j_max == grid.j_cash.len());
        assert!(grid.k_max == grid.k_cash.len());
    }
}
