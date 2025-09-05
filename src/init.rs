use crate::{
    allocator::ALLOCATOR,
    uefi::{EfiHandle, EfiSystemTable, MemoryMapHolder, exit_from_efi_services},
};

pub fn init_basic_runtime(
    image_handle: EfiHandle,
    efi_system_table: &EfiSystemTable,
) -> MemoryMapHolder {
    let mut memory_map = MemoryMapHolder::new();
    exit_from_efi_services(image_handle, efi_system_table, &mut memory_map);
    ALLOCATOR.init_with_mmap(&memory_map);
    memory_map
}
