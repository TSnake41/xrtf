pub mod event;
pub mod hypercall;
pub mod ring;

const HVM_OP: usize = 34;
const HVMOP_GET_PARAM: usize = 1;

const DOMID_SELF: u16 = 0x7FF0;

#[cfg(feature = "fastabi")]
pub(super) unsafe fn hvm_get_param(index: u32) -> u64 {
    const FASTABI_MASK: usize = 0x40000000;
    use crate::native_hypercall;

    let mut output;

    unsafe {
        native_hypercall!(
            in("rax") HVM_OP | FASTABI_MASK,
            lateout("rax") _,
            in("rdi") HVMOP_GET_PARAM,
            in("rsi") DOMID_SELF,
            in("r8") index,
            lateout("r9") output
        );
    }

    output
}

#[cfg(not(feature = "fastabi"))]
#[repr(C)]
#[derive(Clone, Copy, Default)]
struct XenHvmParam {
    pub domid: u16,
    pub _pad: u16,
    pub index: u32,
    pub value: u64,
}

#[cfg(not(feature = "fastabi"))]
pub(super) unsafe fn hvm_get_param(index: u32) -> u64 {
    use core::ptr::addr_of_mut;
    use hypercall::hypercall2;

    let mut param = XenHvmParam {
        domid: DOMID_SELF,
        index,
        ..Default::default()
    };

    unsafe { hypercall2(HVM_OP, [HVMOP_GET_PARAM, addr_of_mut!(param).addr()]) };

    param.value
}
