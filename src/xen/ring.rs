//! Xen paravirtualized ring buffer utilities
use core::{
    cmp,
    sync::atomic::{AtomicU32, Ordering},
};

use volatile::VolatilePtr;

#[derive(Clone, Copy, Debug)]
pub struct XenRing<'a> {
    pub ring: VolatilePtr<'a, [u8]>,
    pub cons: &'a AtomicU32,
    pub prod: &'a AtomicU32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XenRingError {
    /// Data is too large to fit in the ring buffer.
    TooLarge,
    /// Consumer is not ready for receiving the payload
    NotReady,
    /// Misbehaving ring index
    MisbehavingIndex,
}

#[inline(always)]
fn available(prod: usize, cons: usize, len: usize) -> usize {
    match prod.cmp(&cons) {
        cmp::Ordering::Less => cons - prod - 1,
        cmp::Ordering::Equal => len + cons - prod - 1,
        cmp::Ordering::Greater => len - 1,
    }
}

#[inline(always)]
fn queued(prod: usize, cons: usize, len: usize) -> usize {
    len - available(prod, cons, len)
}

impl XenRing<'_> {
    pub fn capacity(&self) -> usize {
        self.ring.len() - 1
    }

    pub fn write(&mut self, buffer: &[u8]) -> Result<(), XenRingError> {
        if buffer.len() >= self.ring.len() {
            return Err(XenRingError::TooLarge);
        }

        let cons = self.cons.load(Ordering::Acquire) as usize % self.ring.len();
        let prod = self.prod.load(Ordering::Acquire) as usize % self.ring.len();

        let dest_prod = (prod + buffer.len()) % self.ring.len();

        if available(prod, cons, self.ring.len()) < buffer.len() {
            return Err(XenRingError::NotReady);
        }

        if prod < cons || buffer.len() <= (self.ring.len() - prod) {
            self.ring.index(prod..dest_prod).copy_from_slice(buffer);
        } else if prod >= cons {
            /*
             * Split the buffer in two parts, one that will be copied at
             * the end of the ring buffer, another at the beginning.
             *
             * [(parts.1)C    P(parts.0)]
             */

            let parts = buffer.split_at(self.ring.len() - prod);
            self.ring.index(prod..).copy_from_slice(parts.0);
            self.ring.index(..dest_prod).copy_from_slice(parts.1);
        }

        self.prod
            .compare_exchange(
                prod as u32,
                dest_prod as u32,
                Ordering::Release,
                Ordering::Relaxed,
            )
            .map_err(|_| XenRingError::MisbehavingIndex)?;

        Ok(())
    }
}
