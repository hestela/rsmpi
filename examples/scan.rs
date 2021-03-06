extern crate mpi;

use mpi::traits::*;
use mpi::topology::Rank;
use mpi::collective::SystemOperation;

fn fac(n: Rank) -> Rank {
    (1..n + 1).fold(1, |x, y| x * y)
}

fn main() {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let rank = world.rank();

    let mut x = 0;
    world.scan_into(&rank, &mut x, SystemOperation::sum());
    assert_eq!(x, (rank * (rank + 1)) / 2);

    let y = rank + 1;
    let mut z = 0;
    world.exclusive_scan_into(&y, &mut z, SystemOperation::product());
    if rank > 0 {
        assert_eq!(z, fac(y - 1));
    }
}
