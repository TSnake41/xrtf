// SPDX-License-Identifier: Apache-2.0
// Copyright 2020 Google LLC
// Copyright 2025 Vates SAS

/*
 * Memory layout
 *
 * 0M-2M : Identity
 * 2M-4M : L1 mappings
 * 4M-1G : Identity
 */

use core::cell::SyncUnsafeCell;

use x86_64::{
    PhysAddr, VirtAddr,
    instructions::tlb,
    registers::control::Cr3,
    structures::paging::{PageSize, PageTable, PageTableFlags, PhysFrame, Size2MiB, Size4KiB},
};

#[unsafe(no_mangle)]
static L4_TABLE: SyncUnsafeCell<PageTable> = SyncUnsafeCell::new(PageTable::new());
#[unsafe(no_mangle)]
static L3_TABLE: SyncUnsafeCell<PageTable> = SyncUnsafeCell::new(PageTable::new());
#[unsafe(no_mangle)]
static L2_TABLE: SyncUnsafeCell<PageTable> = SyncUnsafeCell::new(PageTable::new());
#[unsafe(no_mangle)]
static L1_TABLE: SyncUnsafeCell<PageTable> = SyncUnsafeCell::new(PageTable::new());

#[unsafe(no_mangle)]
pub static mut MEMORY_ENCRYPT_FLAG: PageTableFlags = PageTableFlags::empty();

pub fn setup() {
    // SAFETY: This function is idempontent and only writes to static memory and
    // CR3. Thus, it is safe to run multiple times or on multiple threads.
    // A SyncUnsafeCell pointer is never null.
    let (l4, l3, l2, l1) = unsafe {
        (
            &mut *L4_TABLE.get(),
            &mut *L3_TABLE.get(),
            &mut *L2_TABLE.get(),
            &mut *L1_TABLE.get(),
        )
    };
    let pt_flags =
        PageTableFlags::PRESENT | PageTableFlags::WRITABLE | unsafe { MEMORY_ENCRYPT_FLAG };

    l2[0].set_addr(PhysAddr::zero(), pt_flags | PageTableFlags::HUGE_PAGE);
    l2[1].set_addr(phys_addr_linear(l1), pt_flags);

    let mut next_addr = PhysAddr::new(2 * Size2MiB::SIZE);
    for l2e in l2.iter_mut().skip(2) {
        l2e.set_addr(next_addr, pt_flags | PageTableFlags::HUGE_PAGE);
        next_addr += Size2MiB::SIZE;
    }

    // Point L3 at L2
    l3[0].set_addr(phys_addr_linear(l2), pt_flags);
    // Point L4 at L3
    l4[0].set_addr(phys_addr_linear(l3), pt_flags);

    // Point Cr3 at L4
    let (cr3_frame, cr3_flags) = Cr3::read();
    let l4_frame = PhysFrame::from_start_address(phys_addr_linear(l4)).unwrap();
    if cr3_frame != l4_frame {
        unsafe { Cr3::write(l4_frame, cr3_flags) };
    }
}

fn phys_addr_linear<T>(virt_addr: *const T) -> PhysAddr {
    PhysAddr::new(virt_addr as u64)
}

pub unsafe fn map_frame(frame: PhysFrame<Size4KiB>, encrypted: bool) -> Option<VirtAddr> {
    let l1 = unsafe { &mut *L1_TABLE.get() };
    // Find a spare L1 entry
    let (index, entry) = l1.iter_mut().enumerate().find(|(_, l1e)| l1e.is_unused())?;
    let mut flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;

    if encrypted {
        flags |= unsafe { MEMORY_ENCRYPT_FLAG };
    }

    entry.set_frame(frame, flags);

    let addr = VirtAddr::new(Size2MiB::SIZE + (index as u64) * Size4KiB::SIZE);
    tlb::flush(addr);
    Some(addr)
}
