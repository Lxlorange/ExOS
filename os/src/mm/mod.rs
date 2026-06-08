mod address;
mod heap_allocator;
mod page_table;

pub use address::{
    PhysAddr, PhysPageNum, SimpleRange, SimpleRangeIterator, StepByOne, VirtAddr, VirtPageNum,
    PPNRange, VPNRange,
};
pub use page_table::{PageTableEntry, PTEFlags};

pub fn init() {
    heap_allocator::init_heap();
    heap_allocator::heap_test();
}
