//! Collective communication
//!
//! Developing...
//!
//! # Unfinished features
//!
//! - **5.5**: Varying counts gather operation, `MPI_Gatherv()`
//! - **5.6**: Scatter, `MPI_Scatter()`, `MPI_Scatterv()`
//! - **5.7**: Gather-to-all, `MPI_Allgather()`, `MPI_Allgatherv()`
//! - **5.8**: All-to-all, `MPI_Alltoall()`, `MPI_Alltoallv()`, `MPI_Alltoallw()`
//! - **5.9**: Global reduction operations, `MPI_Reduce()`, `MPI_Op_create()`, `MPI_Op_free()`,
//! `MPI_Allreduce()`, `MPI_Reduce_local()`, `MPI_Op_commutative()`
//! - **5.10**: Reduce-scatter, `MPI_Reduce_scatter_block()`, `MPI_Reduce_scatter()`
//! - **5.11**: Scan, `MPI_Scan()`, `MPI_Exscan()`
//! - **5.12**: Nonblocking collective operations, `MPI_Ibarrier()`, `MPI_Ibcast()`,
//! `MPI_Igather()`, `MPI_Igatherv()`, `MPI_Iscatter()`, `MPI_Iscatterv()`, `MPI_Iallgather()`,
//! `MPI_Iallgatherv()`, `MPI_Ialltoall()`, `MPI_Ialltoallv()`, `MPI_Ialltoallw()`,
//! `MPI_Ireduce()`, `MPI_Iallreduce()`, `MPI_Ireduce_scatter_block()`, `MPI_Ireduce_scatter()`,
//! `MPI_Iscan()`, `MPI_Iexscan()`

use std::{ptr};

use ffi;
use topology::{Rank, Identifier};
use topology::traits::*;
use datatype::traits::*;

pub mod traits;

/// Barrier synchronization among all processes in a `Communicator`
///
/// Calling processes (or threads within the calling processes) will enter the barrier and block
/// execution until all processes in the `Communicator` `&self` have entered the barrier.
///
/// # Standard section(s)
///
/// 5.3
pub trait Barrier {
    /// Partake in a barrier synchronization across all processes in the `Communicator` `&self`.
    ///
    /// # Examples
    /// See `examples/barrier.rs`
    fn barrier(&self);
}

impl<C: RawCommunicator> Barrier for C {
    fn barrier(&self) {
        unsafe { ffi::MPI_Barrier(self.raw()); }
    }
}

/// Something that can take the role of 'root' in a collective operation.
///
/// Many collective operations define a 'root' process that takes a special role in the
/// communication. These collective operations are implemented as traits that have blanket
/// implementations for every type that implements the `Root` trait.
pub trait Root: Communicator {
    /// Rank of the root process
    fn root_rank(&self) -> Rank;
}

impl<'a, C: 'a + RawCommunicator> Root for Identifier<'a, C> {
    fn root_rank(&self) -> Rank {
        self.rank()
    }
}

/// Broadcast of the contents of a buffer
///
/// After the call completes, the `Buffer` on all processes in the `Communicator` of the `Root`
/// `&self` will contain what it contains on the `Root`.
///
/// # Standard section(s)
///
/// 5.4
pub trait BroadcastInto {
    /// Broadcast the contents of `buffer` from the `Root` to the `buffer`s on all other processes.
    ///
    /// # Examples
    /// See `examples/broadcast.rs`
    fn broadcast_into<Buf: Buffer + ?Sized>(&self, buffer: &mut Buf);
}

impl<T: Root> BroadcastInto for T {
    fn broadcast_into<Buf: Buffer + ?Sized>(&self, buffer: &mut Buf) {
        unsafe {
            ffi::MPI_Bcast(buffer.receive_address(), buffer.count(), buffer.datatype().raw(),
                self.root_rank(), self.communicator().raw());
        }
    }
}

/// Gather contents of buffers on `Root`.
///
/// After the call completes, the contents of the `Buffer`s on all ranks will be
/// concatenated into the `Buffer` on `Root`.
// TODO: handle typeof(sendbuf) != typeof(recvbuf[.])
///
/// # Standard section(s)
///
/// 5.5
pub trait GatherInto {
    /// Gather the contents of all `sendbuf`s into `recvbuf` on `Root` `&self`.
    ///
    /// # Examples
    /// See `examples/gather.rs`
    fn gather_into<S: Buffer + ?Sized, R: Buffer + ?Sized>(&self, sendbuf: &S, recvbuf: Option<&mut R>);
}

impl<T: Root> GatherInto for T {
    fn gather_into<S: Buffer + ?Sized, R: Buffer + ?Sized>(&self, sendbuf: &S, recvbuf: Option<&mut R>) {
        unsafe {
            ffi::MPI_Gather(sendbuf.send_address(), sendbuf.count(), sendbuf.datatype().raw(),
                recvbuf.map_or(ptr::null_mut(), |x| x.receive_address()), sendbuf.count(), sendbuf.datatype().raw(),
                self.root_rank(), self.communicator().raw());
        }
    }
}