mod address;
mod heap_allocator;

pub use address::{
    PhysAddr, PhysPageNum, SimpleRange, SimpleRangeIterator, StepByOne, VirtAddr, VirtPageNum,
    PPNRange, VPNRange,
};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PageTableEntry {
    pub bits: usize,
}

pub fn init() {
    heap_allocator::init_heap();
    heap_allocator::heap_test();
}
