pub mod gcm;
pub mod poly1305;

use std::cell::Cell;
use wasmer_runtime_core::{memory::Memory};
use std::mem;


// I had to copy piece below from wasmer/lib/runtime-core/src/memory/ptr.rs
// due to requirement for WasmPtr to work only on ValueType, which 
// cannot be implemented for not crate's types outside of wasmer crate

#[inline(always)]
fn align_pointer(ptr: usize, align: usize) -> usize {
    // clears bits below aligment amount (assumes power of 2) to align pointer
    debug_assert!(align.count_ones() == 1);
    ptr & !(align - 1)
}

#[inline]
pub fn deref<'a, T>(memory: &'a Memory, offset: usize) -> Option<&'a Cell<T>> {
    if offset + mem::size_of::<T>() >= memory.size().bytes().0 {
        return None;
    }
    unsafe {
        let cell_ptr = align_pointer(
            memory.view::<u8>().as_ptr().add(offset) as usize,
            mem::align_of::<T>(),
        ) as *const Cell<T>;
        Some(&*cell_ptr)
    }
}
