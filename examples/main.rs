#![feature(generic_const_exprs)]

/// This program will increment a counter every time it runs.
/// be sure to run the executable directly, as ExecutableStorage
/// assumes the current working is the same the executable is in.
use executable_storage::{Entry, ExecutableStorage};

static mut COUNTER: executable_storage::Entry<usize, 7> = Entry {
    prefix: *b"COUNTER",
    value: 0,
};
fn main() {
    let mut storage = ExecutableStorage::new(unsafe { &mut COUNTER }).unwrap();

    *storage = storage.wrapping_add(1);
}
