use crate::color::Color;

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<Color>,
    pub background_color: Color,
    pub current_color: Color,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Framebuffer {
        // Crear colores predeterminados para el fondo y el color actual
        let background_color = Color::from_hex("Black", 0x000000);
        let current_color = Color::from_hex("White", 0xFFFFFF);

        // Inicializar el buffer con el color de fondo
        let buffer = vec![background_color.clone(); width * height];

        Framebuffer {
            width,
            height,
            buffer,
            background_color,
            current_color,
        }
    }

    pub fn point(&mut self, x: isize, y: isize) {
        if x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height {
            let index = (y as usize) * self.width + (x as usize);
            self.buffer[index] = self.current_color.clone();
        }
    }

    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn to_u32_buffer(&self) -> Vec<u32> {
        self.buffer.iter().map(|color| color.to_u32()).collect()
    }
}
