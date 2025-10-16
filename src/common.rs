// SPDX-License-Identifier: Apache-2.0
// Copyright © 2019 Intel Corporation

// SAFETY: Requires that addr point to a static, null-terminated C-string.
// The returned slice does not include the null-terminator.
#[cfg(target_arch = "x86_64")]
pub unsafe fn from_cstring(addr: u64) -> &'static [u8] {
    if addr == 0 {
        return &[];
    }
    let start = addr as *const u8;
    let mut size: usize = 0;
    while unsafe { start.add(size).read() } != 0 {
        size += 1;
    }
    unsafe { core::slice::from_raw_parts(start, size) }
}

pub fn ascii_strip(s: &[u8]) -> &str {
    core::str::from_utf8(s).unwrap().trim_matches(char::from(0))
}
