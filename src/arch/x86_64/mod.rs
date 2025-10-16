// SPDX-License-Identifier: Apache-2.0
// Copyright (C) 2022 Akira Moroo

use core::arch::x86_64::__cpuid;

pub mod asm;
pub mod gdt;
pub mod idt;
pub mod layout;
pub mod mm;
pub mod sev;
pub mod sse;

pub enum CpuVendor {
    Intel,
    Amd,
}

pub fn setup_cpu_vendor() {
    let leaf = unsafe { __cpuid(0) };

    match (
        &leaf.ebx.to_ne_bytes(),
        &leaf.edx.to_ne_bytes(),
        &leaf.ecx.to_ne_bytes(),
    ) {
        (b"Auth", b"enti", b"cAMD") => unsafe { CPU_VENDOR = CpuVendor::Amd },
        _ => (),
    }
}

pub static mut CPU_VENDOR: CpuVendor = CpuVendor::Intel;
