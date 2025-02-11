use skia::Color4f;

pub const fn rgba(hex: u32) -> Color4f {
    let r = (hex >> 24) as f32 / 255.;
    let g = (hex >> 16) as f32 / 255.;
    let b = (hex >> 8) as f32 / 255.;
    let a = hex as f32 / 255.;

    Color4f::new(r, g, b, a)
}

pub const OUTLINE: Color4f = Color4f::new(0.15294118, 0.15294118, 0.15294118, 1.0);
pub const LIGHT_SQUARE: Color4f = Color4f::new(0.8509804, 0.8509804, 0.8509804, 1.0);
pub const DARK_SQUARE: Color4f = Color4f::new(0.59607846, 0.59607846, 0.59607846, 1.0);

// This should be colorblind-friendly enough.
pub const TOKEN_COLORS: [Color4f; 6] = [
    rgba(0x882255FF), // Pink-Purple
    rgba(0x44AA99FF), // Green-blue
    rgba(0x88CCEEFF), // Sky blue
    rgba(0xDDCC77FF), // Yellow
    rgba(0xCC6677FF), // Light red
    rgba(0xAA4499FF), // Pink
];
