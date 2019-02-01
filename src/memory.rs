use x86_64::{VirtAddr,PhysAddr};
use x86_64::structures::paging::{Mapper,Page,PageTable,RecursivePageTable,FrameAllocator,PhysFrame,Size4KiB};

// Return the physical address for the given virtual address
// or 'None' if the VA is not mapped
pub fn translate_addr(addr: u64, recursive_page_table: &RecursivePageTable) -> Option<PhysAddr> {
    let addr = VirtAddr::new(addr);
    let page: Page = Page::containing_address(addr);

    // perform the translation
    let frame = recursive_page_table.translate_page(page);
    frame.map(|frame| frame.start_address() + u64::from(addr.page_offset()))
}

// Create a RecursivePageTable instance from the level 4 Address
// This function is unsafe
pub unsafe fn init(level_4_table_addr: usize) -> RecursivePageTable<'static> {
    /// Rust currently treats the whole body of unsafe functions as an unsafe
    /// block, which makes it difficult to see which operations are unsafe. To
    /// limit the scope of unsafe we use a safe inner function.
    fn init_inner(level_4_table_addr: usize) -> RecursivePageTable<'static> {
        let level_4_table_ptr = level_4_table_addr as *mut PageTable;
        let level_4_table = unsafe { &mut *level_4_table_ptr };
        RecursivePageTable::new(level_4_table).unwrap()
    }

    init_inner(level_4_table_addr)
}

pub fn create_example_mapping(recursive_page_table: &mut RecursivePageTable, frame_allocator: &mut impl FrameAllocator<Size4KiB>) {
    use x86_64::structures::paging::PageTableFlags as Flags;

    let page: Page = Page::containing_address(VirtAddr::new(0x1000));
    let frame = PhysFrame::containing_address(PhysAddr::new(0xb8000));
    let flags = Flags::PRESENT | Flags::WRITABLE;

    let map_to_result = unsafe {
        recursive_page_table.map_to(page, frame, flags, frame_allocator)
    };
    map_to_result.expect("map_to failed").flush();
}

pub struct EmptyFrameAllocator;

impl FrameAllocator<Size4KiB> for EmptyFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        None
    }
}
