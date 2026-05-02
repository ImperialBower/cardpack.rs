//! No-std compile + link smoke binary.
//!
//! Built only under `--no-default-features` against
//! `--target thumbv7em-none-eabihf` to verify cardpack monomorphizes
//! cleanly without `std`. The `std`-feature build (host) is a no-op
//! `main()` so `cargo build --example no_std_smoke` doesn't break
//! locally on a normal dev machine.

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), no_main)]

#[cfg(not(feature = "std"))]
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
#[cfg(not(feature = "std"))]
mod allocator {
    use core::alloc::{GlobalAlloc, Layout};
    use core::sync::atomic::{AtomicUsize, Ordering};

    const HEAP_SIZE: usize = 16 * 1024; // 16 KiB

    // SyncHeap wraps a raw byte array and implements Sync so it can live in a
    // `static`. Safety: access is coordinated via the atomic OFFSET below.
    struct SyncHeap([u8; HEAP_SIZE]);
    unsafe impl Sync for SyncHeap {}

    static HEAP: SyncHeap = SyncHeap([0u8; HEAP_SIZE]);
    static OFFSET: AtomicUsize = AtomicUsize::new(0);

    pub struct BumpAllocator;

    unsafe impl GlobalAlloc for BumpAllocator {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            let start = HEAP.0.as_ptr() as usize;
            let mut cur = OFFSET.load(Ordering::Relaxed);
            loop {
                let aligned = (start + cur + layout.align() - 1) & !(layout.align() - 1);
                let new_offset = aligned + layout.size() - start;
                if new_offset > HEAP_SIZE {
                    return core::ptr::null_mut();
                }
                match OFFSET.compare_exchange(cur, new_offset, Ordering::SeqCst, Ordering::Relaxed)
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

#[cfg(not(feature = "std"))]
#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[cfg(not(feature = "std"))]
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

#[cfg(feature = "std")]
fn main() {
    // Host build is a no-op; the real verification is the no_std target build.
}
