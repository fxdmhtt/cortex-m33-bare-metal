extern crate alloc;

use core::ptr::addr_of_mut;
use embedded_alloc::TlsfHeap as Heap;

// this is the allocator the application will use
#[global_allocator]
static HEAP: Heap = Heap::empty();

pub fn init() {
    use core::mem::MaybeUninit;
    const HEAP_SIZE: usize = 1024;
    static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    unsafe { HEAP.init(addr_of_mut!(HEAP_MEM) as usize, HEAP_SIZE) }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn init_test() {
//         init();

//         let xs = vec![0, 1, 2];
//         println!("{:?}", xs);
//     }
// }
