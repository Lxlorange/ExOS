mod address;
mod frame_allocator;
mod heap_allocator;
mod page_table;

pub use address::{
    PhysAddr, PhysPageNum, SimpleRange, SimpleRangeIterator, StepByOne, VirtAddr, VirtPageNum,
    PPNRange, VPNRange,
};
pub use frame_allocator::{frame_alloc, FrameTracker};
pub use page_table::{PageTableEntry, PTEFlags};

pub fn init() {
    heap_allocator::init_heap();
    heap_allocator::heap_test();
    frame_allocator::init_frame_allocator();
    frame_allocator::frame_allocator_test();
}
