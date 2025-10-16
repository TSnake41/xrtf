// SPDX-License-Identifier: Apache-2.0
// Copyright (C) 2025 Vates SAS - Teddy Astie

#[macro_export]
macro_rules! native_hypercall {
    ($($t:tt)*) => {
        match $crate::arch::x86_64::CPU_VENDOR {
            crate::arch::x86_64::CpuVendor::Intel => core::arch::asm!("vmcall", $($t)*),
            crate::arch::x86_64::CpuVendor::Amd => core::arch::asm!("vmmcall", $($t)*),
        }
    };
}

#[inline(always)]
pub unsafe fn hypercall5(cmd: usize, param: [usize; 5]) -> usize {
    let output: usize;

    unsafe {
        native_hypercall!(
          in("rax") cmd,
          lateout("rax") output,
          in("rdi") param[0],
          in("rsi") param[1],
          in("rdx") param[2],
          in("r10") param[3],
          in("r8") param[4],
        );
    }

    output
}

#[inline(always)]
pub unsafe fn hypercall4(cmd: usize, param: [usize; 4]) -> usize {
    let output: usize;

    unsafe {
        native_hypercall!(
          in("rax") cmd,
          lateout("rax") output,
          in("rdi") param[0],
          in("rsi") param[1],
          in("rdx") param[2],
          in("r10") param[3],
          out("r8") _,
        );
    }

    output
}

#[inline(always)]
pub unsafe fn hypercall3(cmd: usize, param: [usize; 3]) -> usize {
    let output: usize;

    unsafe {
        native_hypercall!(
          in("rax") cmd,
          lateout("rax") output,
          in("rdi") param[0],
          in("rsi") param[1],
          in("rdx") param[2],
          out("r10") _,
          out("r8") _,
        );
    }

    output
}

#[inline(always)]
pub unsafe fn hypercall2(cmd: usize, param: [usize; 2]) -> usize {
    let output: usize;

    unsafe {
        native_hypercall!(
          in("rax") cmd,
          lateout("rax") output,
          in("rdi") param[0],
          in("rsi") param[1],
          out("rdx") _,
          out("r10") _,
          out("r8") _,
        );
    }

    output
}

#[inline(always)]
pub unsafe fn hypercall1(cmd: usize, param: usize) -> usize {
    let output: usize;

    unsafe {
        native_hypercall!(
          in("rax") cmd,
          lateout("rax") output,
          in("rdi") param,
          out("rsi") _,
          out("rdx") _,
          out("r10") _,
          out("r8") _,
        );
    }

    output
}

#[inline(always)]
pub unsafe fn hypercall0(cmd: usize) -> usize {
    let output: usize;

    unsafe {
        native_hypercall!(
          in("rax") cmd,
          lateout("rax") output,
          out("rdi") _,
          out("rsi") _,
          out("rdx") _,
          out("r10") _,
          out("r8") _,
        );
    }
    output
}
