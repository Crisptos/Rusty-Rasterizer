use crate::global::WIDTH;

/*
    Draw the color value to the pixel at (x,y)
*/
pub fn draw(buffer: &mut Vec<u32>, x: usize, y: usize, xrgb: u32){
    buffer[x + WIDTH*y] = xrgb;
}

/*
    Fill the screen with XRGB
    X: First byte is ignored
    R: Red value byte
    G: Green value byte
    B: Blue value byte
*/
pub fn fill(buffer: &mut Vec<u32>, xrgb: u32) {
    for i in buffer.iter_mut() {
        *i = xrgb; // write something more funny here!
    }
}