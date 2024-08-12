use crate::color::Color;
use crate::framebuffer::Framebuffer;

pub fn render(framebuffer: &mut Framebuffer, map: &[Vec<char>], block_size: usize) {
    for (y, row) in map.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            let color = match cell {
                '+' | '-' | '|' => framebuffer.background_color.clone(), // Paredes
                'p' => Color::from_hex("player", 0xFF0000),
                'g' => Color::from_hex("goal", 0x00FF00),
                _ => Color::from_hex("corridors", 0x0000FF),
            };

            // Pintar un bloque de tamaño `block_size` x `block_size` para cada celda
            for dy in 0..block_size {
                for dx in 0..block_size {
                    let pixel_x = (x * block_size + dx) as isize;
                    let pixel_y = (y * block_size + dy) as isize;
                    framebuffer.set_current_color(color.clone()); // Establecer el color actual
                    framebuffer.point(pixel_x, pixel_y); // Dibujar el punto en el color establecido
                }
            }
        }
    }
}
