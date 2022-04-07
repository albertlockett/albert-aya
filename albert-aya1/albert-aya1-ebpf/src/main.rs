#![no_std]
#![no_main]

use albert_aya1_common::PacketLog;
use aya_bpf::{
    bindings::xdp_action,
    macros::{map, xdp},
    programs::XdpContext, maps::PerfEventArray,
};

#[map(name = "EVENTS")]
static mut EVENTS: PerfEventArray<PacketLog> = PerfEventArray::<PacketLog>::with_max_entries(1024, 0);

#[xdp(name="albert_aya1")]
pub fn albert_aya1(ctx: XdpContext) -> u32 {
    match unsafe { try_albert_aya1(ctx) } {
        Ok(ret) => ret,
        Err(_) => xdp_action::XDP_ABORTED,
    }
}

unsafe fn try_albert_aya1(_ctx: XdpContext) -> Result<u32, u32> {
    Ok(xdp_action::XDP_PASS)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
