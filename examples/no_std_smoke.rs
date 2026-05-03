//! No-std compile + link smoke binary.
//!
//! Built only against `--target thumbv7em-none-eabihf` (or any other
//! `target_os = "none"` target) to verify cardpack monomorphizes
//! cleanly without `std`. On any host target the binary is a no-op
//! `main()` so `cargo build --example no_std_smoke` doesn't break
//! locally on a normal dev machine.
//!
//! The cfg gate is `target_os = "none"` (not `not(feature = "std")`)
//! because on host targets the sysroot always provides libstd, even
//! under `--no-default-features` — and a `#[panic_handler]` collides
//! with std's at link time. Only true bare-metal targets have no std
//! in the sysroot, so the no_std body must be gated on the target,
//! not on cardpack's `std` feature flag.

#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

#[cfg(target_os = "none")]
use core::panic::PanicInfo;

// A minimal static bump allocator for bare-metal targets.
//
// Why this is needed: cardpack's no_std story uses `extern crate alloc`
// (BTreeMap, BTreeSet, Vec, String) — those types are heap-backed, and
// `alloc` requires the binary to provide a `#[global_allocator]`. When
// std is present, std's default allocator is registered automatically;
// under no_std the binary is on its own.
//
// A bump allocator is the simplest possible implementation: hand out
// chunks of a fixed buffer, never reclaim. That's fine for a smoke
// binary that never actually runs (we only compile + link to verify
// the no_std code path). 16 KiB is far more than the binary's
// monomorphized allocations need (Standard52 deck + BTreeSet/BTreeMap
// of 52 cards is well under 1 KiB).
#[cfg(target_os = "none")]
mod allocator {
    use core::alloc::{GlobalAlloc, Layout};
    use core::sync::atomic::{AtomicUsize, Ordering};

    const HEAP_SIZE: usize = 16 * 1024; // 16 KiB

    // SyncHeap wraps a raw byte array and implements Sync so it can live in a
    // `static`. Safety: access is coordinated via the atomic OFFSET below.
    struct SyncHeap([u8; HEAP_SIZE]);
    unsafe impl Sync for SyncHeap {}

    // OFFSET tracks the number of bytes consumed from the start of HEAP
    // (not an absolute address).
    static HEAP: SyncHeap = SyncHeap([0u8; HEAP_SIZE]);
    static OFFSET: AtomicUsize = AtomicUsize::new(0);

    pub struct BumpAllocator;

    // NOTE: This is a single-core-friendly bump allocator. The Acquire/Release
    // pairing on the CAS makes it sound on single-core Cortex-M / RISC-V, which
    // is what `thumbv7em-none-eabihf` targets. Do NOT copy this code into a
    // multi-core embedded project (RP2040 dual-core, ESP32-S3 dual-core, etc.)
    // without revisiting both the orderings and the use of a static buffer
    // that's shared across cores. For multi-core, use a real allocator like
    // `linked_list_allocator` or `talc`.
    unsafe impl GlobalAlloc for BumpAllocator {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            let start = HEAP.0.as_ptr() as usize;
            let mut cur = OFFSET.load(Ordering::Acquire);
            loop {
                let aligned = (start + cur + layout.align() - 1) & !(layout.align() - 1);
                let new_offset = aligned + layout.size() - start;
                if new_offset > HEAP_SIZE {
                    return core::ptr::null_mut();
                }
                match OFFSET.compare_exchange(cur, new_offset, Ordering::AcqRel, Ordering::Acquire)
                {
                    Ok(_) => return aligned as *mut u8,
                    Err(x) => cur = x,
                }
            }
        }

        unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
            // Bump allocator never frees; the binary loops forever anyway.
        }
    }

    unsafe impl Sync for BumpAllocator {}

    #[global_allocator]
    static ALLOCATOR: BumpAllocator = BumpAllocator;
}

#[cfg(target_os = "none")]
#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[cfg(target_os = "none")]
// Rust 2024 edition reclassifies `no_mangle`, `link_section`, and
// `export_name` as unsafe attributes (because mangled-symbol collisions
// can break soundness at link time). The `#[unsafe(...)]` wrapper is
// the new required form. Cardpack pins `edition = "2024"` (Cargo.toml).
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    use cardpack::prelude::*;
    use rand::{SeedableRng, rngs::StdRng};

    let deck: Pile<Standard52> = Pile::<Standard52>::deck();
    let _ = deck.len();
    let _ = deck.unique_cards();
    let _ = deck.map_by_suit();

    let mut rng = StdRng::seed_from_u64(42);
    let _ = deck.shuffled_with_rng(&mut rng);

    loop {}
}

#[cfg(not(target_os = "none"))]
fn main() {
    // Host build is a no-op; the real verification is the no_std target build.
}
