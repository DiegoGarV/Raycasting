use crate::color::Color;

#[derive(Debug, Clone)]
pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<Color>,
    pub background_color: Color,
    pub current_color: Color,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self{
        let black = Color::new(0,0,0);
        let white = Color::new(255,255,255);
        let buffer_size = width * height;
        let buffer = vec![black; buffer_size];
        Framebuffer{
            width,
            height,
            buffer,
            background_color: black,
            current_color: white,
        }
    }

    pub fn clear(&mut self){
        for elem in self.buffer.iter_mut() {
            *elem = self.background_color;
        }
    }

    pub fn point(&mut self, x: usize, y: usize){
        if x< self.width && y< self.height {
            let index = y*self.width +x;
            self.buffer[index] = self.current_color;
        }
    }
    pub fn color_array_to_u32(&mut self) -> Vec<u32> {
        self.buffer.iter().map(|color| {
            ((color.r as u32) << 16) | ((color.g as u32) << 8) | (color.b as u32)
        }).collect()  // Collect into a Vec<u32>
    }

    // pub fn set_bgcolor(&mut self, color: u32){
    //     self.background_color = Color::from_hex(color);
    // }

    pub fn set_current_color(&mut self, color: u32){
        self.current_color = Color::from_hex(color);
    }

}