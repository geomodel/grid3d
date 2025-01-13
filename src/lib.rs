
mod light_grid;
mod fast_grid;

//  //  //  //  //  //  //  //
pub use types3d;
pub use light_grid::*;
pub use fast_grid::*;

pub trait IndexFromCoord3D {
    fn size(&self) -> usize;
    fn i_max(&self) -> usize;
    fn j_max(&self) -> usize;
    fn k_max(&self) -> usize;
    fn index_from(&self, coord: &types3d::IJK) -> Option<usize>;
}
