#![no_std]
#![no_main]

use aya_bpf::{bindings::xdp_action, macros::xdp, programs::XdpContext};
use aya_bpf::cty::c_void;
use aya_bpf::helpers::bpf_redirect_map;
use aya_log_ebpf::info;
use libbpf_rs::{MapFlags, MapHandle, MapType};
use libbpf_rs::libbpf_sys::{__u32, bpf_map_create_opts};
use once_cell::sync::Lazy;

static XDP_STATS_MAP: Lazy<MapHandle> = Lazy::new(|| {
    let opts = bpf_map_create_opts::default();
    MapHandle::create(MapType::PercpuArray, Some("xdp_stats_map"),
                      core::mem::size_of::<__u32>() as u32,
                      core::mem::size_of::<__u32>() as u32,
                      4096, &opts)
        .expect("create xdp_stats_map failed!")
});

static mut XSKS_MAP: Lazy<MapHandle> = Lazy::new(|| {
    let opts = bpf_map_create_opts::default();
    MapHandle::create(MapType::Xskmap, Some("xsks_map"),
                      core::mem::size_of::<__u32>() as u32,
                      core::mem::size_of::<__u32>() as u32,
                      4096, &opts)
        .expect("create xsks_map failed!")
});

#[xdp]
pub fn open_coroutine(ctx: XdpContext) -> u32 {
    // can't compile, give up
    info!(&ctx, "received a packet");
    unsafe {
        let index = core::mem::transmute((*ctx.ctx).rx_queue_index);
        if let Ok(Some(vev)) = XDP_STATS_MAP.lookup(&index, MapFlags::ANY) {
            let pkt_count = vev.as_ptr() as *const __u32;
            if !pkt_count.is_null() {
                let mut count = *pkt_count;
                count = count + 1;
                if count & 1 != 0 {
                    return xdp_action::XDP_PASS;
                }
            }
        }
        if let Ok(ok) = XSKS_MAP.lookup(&index, MapFlags::ANY) {
            let xsks_map= Lazy::force_mut(&mut XSKS_MAP);
            return bpf_redirect_map(xsks_map as *mut c_void,core::mem::transmute(index),0) as u32;
        }
    }
    xdp_action::XDP_PASS
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
