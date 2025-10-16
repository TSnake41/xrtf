// SPDX-License-Identifier: Apache-2.0
// Copyright (C) 2025 Vates SAS - Teddy Astie

use core::{fmt, sync::atomic::AtomicU32};

use volatile::{VolatileFieldAccess, VolatilePtr};

use crate::{
    arch::map_4k_frame,
    xen::{
        event::EventChannel,
        hvm_get_param,
        ring::{XenRing, XenRingError},
    },
};

#[repr(C)]
#[derive(VolatileFieldAccess)]
pub struct XenConsInterface {
    in_buffer: [u8; 1024],
    out_buffer: [u8; 2048],
    in_cons: u32,
    in_prod: u32,
    out_cons: u32,
    out_prod: u32,
}

const HVM_PARAM_CONSOLE_PFN: u32 = 17;
const HVM_PARAM_CONSOLE_EVTCHN: u32 = 18;

pub struct XenConsole {
    interface: XenRing<'static>,
    event_channel: EventChannel,
}

unsafe impl Send for XenConsole {}
unsafe impl Sync for XenConsole {}

impl XenConsole {
    pub unsafe fn new() -> Option<Self> {
        let pfn = unsafe { hvm_get_param(HVM_PARAM_CONSOLE_PFN) };
        let evtchn = unsafe { hvm_get_param(HVM_PARAM_CONSOLE_EVTCHN) };

        if pfn == 0 {
            return None;
        }

        let console = unsafe { VolatilePtr::new(map_4k_frame(pfn, false)?) };

        console.out_prod().write(1);

        Some(Self {
            interface: XenRing {
                ring: console.out_buffer().as_slice(),
                cons: unsafe { AtomicU32::from_ptr(console.out_cons().as_raw_ptr().as_ptr()) },
                prod: unsafe { AtomicU32::from_ptr(console.out_prod().as_raw_ptr().as_ptr()) },
            },
            event_channel: EventChannel(evtchn as u32),
        })
    }
}

impl fmt::Write for XenConsole {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            if byte == b'\n' {
                self.interface.write(b"\r").ok();
            }

            while let Err(e) = self.interface.write(&[byte]) {
                assert_ne!(e, XenRingError::NotReady);

                self.event_channel.send();
            }
        }

        self.event_channel.send();
        Ok(())
    }
}
