//! QEMU Machine Protocol (QMP) structure representation library.

pub mod generic;

mod qmp_monitor;
mod vm_run_state;
// TODO other modules.

pub use qmp_monitor::query_version;
// TODO pub use all the command constructors.
