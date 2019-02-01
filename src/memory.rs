use x86_64::PhysAddr;
use x86_64::structures::paging::PageTable;

// Return the physical address for the given virtual address
// or 'None' if the VA is not mapped
pub fn translate_addr(addr: usize, level_4_table_addr: usize) -> Option<PhysAddr> {
    let level_4_index = (addr >> 39) & 0o777;
    let level_3_index = (addr >> 30) & 0o777;
    let level_2_index = (addr >> 21) & 0o777;
    let level_1_index = (addr >> 12) & 0o777;
    let page_offset = addr & 0o777;

    // check that the level 4 index is mapped
    let level_4_table = unsafe {&*(level_4_table_addr as *const PageTable)};
    if level_4_table[level_4_index].addr().is_null() {
        return None;
    }
    let level_3_table_addr = (level_4_table_addr << 9) | (level_4_index << 12);

    // check that the level 3 index is mapped
    let level_3_table = unsafe {&*(level_3_table_addr as *const PageTable)};
    if level_3_table[level_3_index].addr().is_null() {
        return None;
    }
    let level_2_table_addr = (level_3_table_addr << 9) | (level_3_index << 12);

    // check that the level 2 index is mapped
    let level_2_table = unsafe {&*(level_2_table_addr as *const PageTable)};
    if level_2_table[level_2_index].addr().is_null() {
        return None;
    }
    let level_1_table_addr = (level_2_table_addr << 9) | (level_2_index << 12);

    // check that the level 1 entry is mapped and retrieve the physical address from it
    let level_1_table = unsafe {&*(level_1_table_addr as *const PageTable)};
    let phys_addr = level_1_table[level_1_index].addr();
    if phys_addr.is_null() {
        return None;
    }

    Some(phys_addr + page_offset)
}
