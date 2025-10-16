// SPDX-License-Identifier: Apache-2.0
// Copyright (C) 2025 Vates SAS - Teddy Astie

#[inline(always)]
pub unsafe fn hypercall5(_cmd: usize, _param: [usize; 5]) -> usize {
    0
}

#[inline(always)]
pub unsafe fn hypercall4(_cmd: usize, _param: [usize; 4]) -> usize {
    0
}

#[inline(always)]
pub unsafe fn hypercall3(_cmd: usize, _param: [usize; 3]) -> usize {
    0
}

#[inline(always)]
pub unsafe fn hypercall2(_cmd: usize, _param: [usize; 2]) -> usize {
    0
}

#[inline(always)]
pub unsafe fn hypercall1(_cmd: usize, _param: usize) -> usize {
    0
}

#[inline(always)]
pub unsafe fn hypercall0(_cmd: usize) -> usize {
    0
}
