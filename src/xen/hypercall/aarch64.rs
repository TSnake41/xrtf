// SPDX-License-Identifier: Apache-2.0
// Copyright (C) 2025 Vates SAS - Teddy Astie

use core::arch::asm;

const XEN_IMM: u16 = 0xEA1;

#[inline(always)]
pub unsafe fn hypercall5(cmd: usize, param: [usize; 5]) -> usize {
    let output: usize;

    asm!(
        "hvc #{XEN_IMM}",
        in("x0") cmd,
        lateout("x0") output,
        inout("x1") param[0] => _,
        inout("x2") param[1] => _,
        inout("x3") param[2] => _,
        inout("x4") param[3] => _,
        inout("x5") param[4] => _,
        inout("x16") 0 => _,
        XEN_IMM = const XEN_IMM,
    );

    output
}

#[inline(always)]
pub unsafe fn hypercall4(cmd: usize, param: [usize; 4]) -> usize {
    let output: usize;

    asm!(
        "hvc #{XEN_IMM}",
        in("x0") cmd,
        lateout("x0") output,
        inout("x1") param[0] => _,
        inout("x2") param[1] => _,
        inout("x3") param[2] => _,
        inout("x4") param[3] => _,
        inout("x16") 0 => _,
        XEN_IMM = const XEN_IMM,
    );

    output
}

#[inline(always)]
pub unsafe fn hypercall3(cmd: usize, param: [usize; 3]) -> usize {
    let output: usize;

    asm!(
      "hvc #{XEN_IMM}",
      in("x0") cmd,
      lateout("x0") output,
      inout("x1") param[0] => _,
      inout("x2") param[1] => _,
      inout("x3") param[2] => _,
      inout("x16") 0 => _,
      XEN_IMM = const XEN_IMM,
    );

    output
}

#[inline(always)]
pub unsafe fn hypercall2(cmd: usize, param: [usize; 2]) -> usize {
    let output: usize;

    asm!(
        "hvc #{XEN_IMM}",
        in("x0") cmd,
        lateout("x0") output,
        inout("x1") param[0] => _,
        inout("x2") param[1] => _,
        inout("x16") 0 => _,
        XEN_IMM = const XEN_IMM,
    );

    output
}

#[inline(always)]
pub unsafe fn hypercall1(cmd: usize, param: usize) -> usize {
    let output: usize;

    asm!(
        "hvc #{XEN_IMM}",
        in("x0") cmd,
        lateout("x0") output,
        inout("x1") param => _,
        inout("x16") 0 => _,
        XEN_IMM = const XEN_IMM,
    );

    output
}

#[inline(always)]
pub unsafe fn hypercall0(cmd: usize) -> usize {
    let output: usize;

    asm!(
        "hvc #{XEN_IMM}",
        in("x0") cmd,
        lateout("x0") output,
        inout("x16") 0 => _,
        XEN_IMM = const XEN_IMM,
    );

    output
}
