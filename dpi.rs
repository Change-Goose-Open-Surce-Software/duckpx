pub fn px_to_mm(px: f64) -> f64 {
    // Standard-DPI: 96 DPI = 37.795275591 px/cm → 1px = 0.264583333 mm
    px * 0.264583333
}

pub fn mm_to_px(mm: f64) -> f64 {
    mm / 0.264583333
}
