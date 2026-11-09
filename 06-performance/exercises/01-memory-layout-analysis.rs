/// ============================================================================
/// MEMORY LAYOUT ANALYSIS EXERCISES
/// ============================================================================
///
/// Measure size/alignment/offsets of structs, find the padding, and reorder
/// fields to shrink them. Uses std::mem::{size_of, align_of, offset_of}.
///
/// Run: rustc --edition 2021 01-memory-layout-analysis.rs -o ml && ./ml

use std::mem::{align_of, offset_of, size_of};

// ============================================================================
// EXERCISE 1: Measure a padded struct
// ============================================================================

#[repr(C)]
#[derive(Debug)]
pub struct BadLayout {
    pub a: u8,   // offset 0
    pub b: u32,  // offset 4 (3 bytes padding after a)
    pub c: u8,   // offset 8
}                // size 12 (4 trailing padding to align 4)

#[repr(C)]
#[derive(Debug)]
pub struct GoodLayout {
    pub b: u32,  // offset 0
    pub a: u8,   // offset 4
    pub c: u8,   // offset 5
}                // size 8 (3 trailing padding to align 4)

pub fn exercise_measure_layout() {
    println!("BadLayout:  size={} align={}", size_of::<BadLayout>(), align_of::<BadLayout>());
    println!("  offset b = {}", offset_of!(BadLayout, b));
    println!("  offset c = {}", offset_of!(BadLayout, c));
    println!("GoodLayout: size={} align={}", size_of::<GoodLayout>(), align_of::<GoodLayout>());
    println!("  offset a = {}", offset_of!(GoodLayout, a));
    println!("  offset c = {}", offset_of!(GoodLayout, c));
    assert!(size_of::<GoodLayout>() <= size_of::<BadLayout>());
}

// ============================================================================
// EXERCISE 2: The niche optimization — Option<Box<T>> is pointer-sized
// ============================================================================

pub fn exercise_niche_optimization() {
    let box_size = size_of::<Box<u8>>();
    let opt_box = size_of::<Option<Box<u8>>>();
    println!("Box<u8> = {} bytes, Option<Box<u8>> = {} bytes", box_size, opt_box);
    assert_eq!(opt_box, box_size); // None is represented by the null pointer

    // But Option<u8> costs 2 bytes (needs a separate discriminant):
    assert!(size_of::<Option<u8>>() > size_of::<u8>());
}

// ============================================================================
// EXERCISE 3: Hot/cold split with Box
// ============================================================================

#[derive(Debug)]
pub struct HotCold {
    pub id: u64,
    pub kind: u8,
    pub debug_info: Option<Box<String>>, // cold: boxed, keeps struct small
}

pub fn exercise_hot_cold_split() {
    let with_box = size_of::<HotCold>();
    // Without the Box, the struct would carry the full String (24 bytes):
    struct NoBox {
        id: u64,
        kind: u8,
        debug_info: Option<String>,
    }
    let without_box = size_of::<NoBox>();
    println!("HotCold (boxed cold field) = {} bytes vs unboxed = {} bytes", with_box, without_box);
    assert!(with_box < without_box);
}

// ============================================================================
// EXERCISE 4: Cache-line alignment to avoid false sharing
// ============================================================================

#[repr(align(64))] // one full cache line
pub struct CacheLineAligned(pub u64);

pub fn exercise_cache_line_alignment() {
    assert_eq!(align_of::<CacheLineAligned>(), 64);
    assert!(size_of::<CacheLineAligned>() >= 64);
    println!("CacheLineAligned: size={} align={}", size_of::<CacheLineAligned>(), align_of::<CacheLineAligned>());
}

// ============================================================================
// EXERCISE 5: AoS vs SoA memory footprint
// ============================================================================

#[repr(C)]
pub struct Particle {
    pub pos: [f32; 3],
    pub vel: [f32; 3],
}

pub fn exercise_aos_vs_soa() {
    let n = 1000;
    let aos = size_of::<Particle>() * n;
    // SoA: two separate Vec<[f32;3]> — same total bytes, but each field's
    // data is contiguous, which vectorizes much better.
    let soa = size_of::<[f32; 3]>() * n * 2;
    println!("AoS {} bytes vs SoA {} bytes for {} particles", aos, soa, n);
    assert_eq!(aos, soa); // same size — the win is access pattern, not size
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    println!("=== MEMORY LAYOUT ANALYSIS ===\n");
    exercise_measure_layout();
    println!();
    exercise_niche_optimization();
    println!();
    exercise_hot_cold_split();
    println!();
    exercise_cache_line_alignment();
    println!();
    exercise_aos_vs_soa();
    println!("\n=== ALL LAYOUT EXERCISES COMPLETE ===");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_good_layout_smaller() {
        assert!(size_of::<GoodLayout>() < size_of::<BadLayout>());
    }

    #[test]
    fn test_offsets() {
        assert_eq!(offset_of!(BadLayout, b), 4);
        assert_eq!(offset_of!(GoodLayout, b), 0);
    }

    #[test]
    fn test_niche() {
        assert_eq!(size_of::<Option<Box<u8>>>(), size_of::<Box<u8>>());
    }

    #[test]
    fn test_cache_line() {
        assert_eq!(align_of::<CacheLineAligned>(), 64);
    }
}
