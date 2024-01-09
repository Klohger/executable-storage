#![feature(generic_const_exprs)]
/// This program will increment a counter every time it runs.
/// be sure to run the executable directly, as ExecutableStorage
/// assumes the current working is the same the executable is in.
use executable_storage::{ExecutableStorage, StaticStorage};

static mut COUNTER: StaticStorage<usize, 7> = StaticStorage {
    prefix: *b"COUNTER",
    value: 0,
};
fn main() {
    let mut storage = ExecutableStorage::new(unsafe { &mut COUNTER }).unwrap();

    *storage = storage.wrapping_add(1);
}
