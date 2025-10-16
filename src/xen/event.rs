// SPDX-License-Identifier: Apache-2.0
// Copyright (C) 2025 Vates SAS - Teddy Astie

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct EventChannel(pub u32);

const EVENT_CHANNEL_OP: usize = 32;

const EVTCHN_SEND: usize = 4;

impl EventChannel {
    #[cfg(not(feature = "fastabi"))]
    pub fn send(&self) {
        use crate::xen::hypercall::hypercall2;
        use core::ptr::addr_of;

        #[repr(transparent)]
        struct EvtchnSend {
            port: EventChannel,
        }

        let evtchn_send = EvtchnSend { port: *self };

        unsafe {
            hypercall2(
                EVENT_CHANNEL_OP,
                [EVTCHN_SEND, addr_of!(evtchn_send).addr()],
            );
        }
    }

    #[cfg(feature = "fastabi")]
    pub fn send(&self) {
        const FASTABI_MASK: usize = 0x40000000;
        use crate::native_hypercall;

        unsafe {
            native_hypercall!(
                in("rax") EVENT_CHANNEL_OP | FASTABI_MASK,
                lateout("rax") _,
                in("rdi") EVTCHN_SEND,
                in("rsi") self.0,
            );
        }
    }
}
