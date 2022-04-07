#![no_std]
#![no_main]

use core::mem;

use albert_aya1_common::PacketLog;
use aya_bpf::{
    bindings::xdp_action,
    macros::{map, xdp},
    programs::XdpContext, maps::PerfEventArray,
};
use memoffset::offset_of;


mod bindings;
use bindings::{ethhdr, iphdr};

#[map(name = "EVENTS")]
static mut EVENTS: PerfEventArray<PacketLog> = PerfEventArray::<PacketLog>::with_max_entries(1024, 0);


#[inline(always)]
unsafe fn ptr_at<T>(ctx: &XdpContext, offset: usize) -> Result<*const T, ()> {
    let start = ctx.data();
    let end = ctx.data_end();
    let len = mem::size_of::<T>();

    if start + offset + len > end {
        return Err(());
    }

    Ok((start + offset) as *const T)
}

#[xdp(name="albert_aya1")]
pub fn albert_aya1(ctx: XdpContext) -> u32 {
    match unsafe { try_albert_aya1(ctx) } {
        Ok(ret) => ret,
        Err(_) => xdp_action::XDP_ABORTED,
    }
}

#[allow(clippy::forget_copy)]
fn try_albert_aya1(ctx: XdpContext) -> Result<u32, ()> {
   let h_proto = u16::from_be(unsafe { *ptr_at(&ctx, offset_of!(ethhdr, h_proto))? });
   if h_proto != ETH_P_IP {
       return Ok(xdp_action::XDP_PASS)
   }

   let source = u32::from_be(unsafe { *ptr_at(&ctx, ETH_HDR_LEN + offset_of!(iphdr, saddr))? });

    let log_entry = PacketLog {
        ipv4_address: source,
        action: xdp_action::XDP_PASS,
    };
    unsafe {
        EVENTS.output(&ctx, &log_entry, 0);
    }
    Ok(xdp_action::XDP_PASS)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}

const ETH_P_IP: u16 = 0x0800;
const ETH_HDR_LEN: usize = mem::size_of::<ethhdr>();
