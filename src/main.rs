mod framebuffer;
mod map;
mod color;
mod render;
mod player;
mod cast_ray;

use framebuffer::Framebuffer;
use map::load_map;
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
// use color::Color;
use player::Player;
use nalgebra_glm::Vec2;
use cast_ray::cast_ray;

fn main() {
    let map = load_map("./map.txt");

    let block_size = 35;
    let framebuffer_width = map[0].len() * block_size;
    let framebuffer_height = map.len() * block_size;

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Map View",
        framebuffer_width,
        framebuffer_height,
        WindowOptions::default(),
    )
    .unwrap();

    let frame_delay = Duration::from_millis(16);

    // Encuentra la posición del jugador
    let mut player_pos = Vec2::new(0.0,0.0);
    for (y, row) in map.iter().enumerate() {
        if let Some(x) = row.iter().position(|&c| c == 'p') {
            player_pos = Vec2::new(
                (x * block_size) as f32,
                (y * block_size) as f32,
            );
            break;
        }
    }

    // Inicializar al jugador
    let player = Player {
        pos: player_pos, 
        a: 0.0,
    };

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        // Renderiza el mapa
        render::render(&mut framebuffer, &map, block_size);

        // Proyecta el rayo de visión del jugador
        cast_ray(&mut framebuffer, &map, &player, player.a, block_size);

        let u32_buffer = framebuffer.to_u32_buffer();
        window
            .update_with_buffer(&u32_buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
