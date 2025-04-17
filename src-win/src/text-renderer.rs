pub struct TextRenderer {
    width: usize,
    height: usize,
    font_size: usize,
}
impl TextRenderer {
    pub fn new(width: usize, height: usize, font_size: usize) -> Self {
        TextRenderer {
            width,
            height,
            font_size,
        }
    }
    
    pub fn draw(&self, buffer: &mut [u32], pos: (usize, usize), text: &str) {
        let (x, y) = pos;
        let size = self.font_size;
        
        for (i, _) in text.chars().enumerate() {
            let char_x = x + i * (size * 6);
            
            for py in 0..size * 7 {
                for px in 0..size * 5 {
                    let buffer_pos = (y + py) * self.width + (char_x + px);
                    if buffer_pos < buffer.len() {
                        buffer[buffer_pos] = 0xFFFFFFFF; // White
                    }
                }
            }
        }
    }
}