use std::rc::Rc;
use std::time::Instant;

use grid3d::*;

const IM: usize = 333;
const JM: usize = 222;
const KM: usize = 77;

const TM: usize = 5;

//  //  //  //  //  //  //  //
#[test]
fn bench_fast_grid() {
    let start = Instant::now();

    let grid = CachedGrid::new_rc(IM, JM, KM);
    workload(grid.clone());

    println!("Elapsed time: {:.2?}", start.elapsed());
}

//  //  //  //  //  //  //  //
#[test]
fn bench_light_grid() {
    let start = Instant::now();

    let grid = LightGrid::new_rc(IM, JM, KM);
    workload(grid.clone());

    println!("Elapsed time: {:.2?}", start.elapsed());
}

//  //  //  //  //  //  //  //
//  //  //  //  //  //  //  //
fn workload(grid: Rc<dyn IndexFromCoord3D>) {
    let mut a = vec![Some(0_f64); IM * JM * KM];

    for t in 0..TM {
        let cycle_start = Instant::now();

        for i in 0..grid.i_max() {
            for j in 0..grid.j_max() {
                for k in 0..grid.k_max() {
                    for _ in 0..10 {
                        let coord = types3d::IJK { i, j, k };
                        if let Some(index) = grid.index_from(&coord) {
                            let r = cycle_start.elapsed().as_secs_f64() as usize;
                            let x = (i + j + k) * r;
                            a[index] = Some(x as f64);
                        }
                    }
                }
            }
        }
        println!("cycle #{}: {:.2?}", t, cycle_start.elapsed());
    }
}
