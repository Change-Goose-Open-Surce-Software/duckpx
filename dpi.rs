pub fn px_to_mm(px: f64) -> f64 {
    px * 0.264583333  // 1px = 0.264583333 mm bei 96 DPI
}

pub fn mm_to_px(mm: f64) -> f64 {
    mm / 0.264583333
}

pub fn px_to_inch(px: f64) -> f64 {
    px / 96.0  // 1 inch = 96px bei 96 DPI
}

pub fn inch_to_px(inch: f64) -> f64 {
    inch * 96.0
}

pub fn mm_to_inch(mm: f64) -> f64 {
    mm / 25.4  // 1 inch = 25.4 mm
}

pub fn inch_to_mm(inch: f64) -> f64 {
    inch * 25.4
}
