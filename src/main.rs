#![no_main]

use core::arch::asm;
use core::fmt::Write;

use wasabi::{
    graphics::{Bitmap, draw_test_pattern, fill_rect},
    uefi::{
        EfiHandle, EfiMemoryType, EfiSystemTable, MemoryMapHolder, VramTextWriter,
        exit_from_efi_services, init_vram,
    },
};

#[unsafe(no_mangle)]
fn efi_main(image_handle: EfiHandle, efi_system_table: &EfiSystemTable) {
    let mut vram = init_vram(efi_system_table).expect("init_vram failed");

    let vw = vram.width();
    let vh = vram.height();

    fill_rect(&mut vram, 0x000000, 0, 0, vw, vh).expect("fill_rect failed");
    draw_test_pattern(&mut vram);

    let mut w = VramTextWriter::new(&mut vram);
    for i in 0..4 {
        writeln!(w, "i = {i}").unwrap();
    }

    let mut memory_map = MemoryMapHolder::new();
    let status = efi_system_table
        .boot_services
        .get_memory_map(&mut memory_map);

    writeln!(w, "{status:?}").unwrap();

    let mut total_memory_pages = 0;
    for e in memory_map.iter() {
        if e.memory_type() != EfiMemoryType::CONVENTIONAL_MEMORY {
            continue;
        }
        total_memory_pages += e.number_of_pages();
        writeln!(w, "{e:?}").unwrap();
    }

    let total_memory_size_mib = total_memory_pages * 4096 / 1024 / 1024;
    writeln!(
        w,
        "Total: {total_memory_pages} pages = {total_memory_size_mib} MiB"
    )
    .unwrap();

    exit_from_efi_services(image_handle, efi_system_table, &mut memory_map);
    writeln!(w, "Hello, Non-UEFIworld!").unwrap();

    loop {
        hlt();
    }
}

pub fn hlt() {
    unsafe { asm!("hlt") }
}
