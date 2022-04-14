#![no_std]
#![no_main]

use aya_bpf::{
    macros::tracepoint,
    programs::TracePointContext,
};
use aya_log_ebpf::info;

#[tracepoint(name="albert_aya3")]
pub fn albert_aya3(ctx: TracePointContext) -> u32 {
    match unsafe { try_albert_aya3(ctx) } {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

unsafe fn try_albert_aya3(ctx: TracePointContext) -> Result<u32, u32> {
    info!(&ctx, "Hello");
    Ok(0)
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
