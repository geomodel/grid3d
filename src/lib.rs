
mod light_grid;
mod cached_grid;

//  //  //  //  //  //  //  //
pub use types3d;
pub use light_grid::LightGrid;
pub use cached_grid::CachedGrid;

pub trait IndexFromCoord3D {
    fn size(&self) -> usize;
    fn i_max(&self) -> usize;
    fn j_max(&self) -> usize;
    fn k_max(&self) -> usize;
    fn index_from(&self, coord: &types3d::IJK) -> Option<usize>;
}
