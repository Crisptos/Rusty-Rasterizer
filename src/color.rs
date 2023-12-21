// A Color that's comprised of r, g, b values, each 1 byte in size
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    // Create a new Color with the given r, g, b values
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color {
            r: (r),
            g: (g),
            b: (b),
        }
    }

    // Calculate the value of the Color's rgb
    pub fn value(&self) -> u32 {
        let mut xrgb: u32 = 0x0;
        xrgb |= self.r as u32;
        xrgb <<= 8;
        xrgb |= self.g as u32;
        xrgb <<= 8;
        xrgb |= self.b as u32;
        xrgb
    }
}
