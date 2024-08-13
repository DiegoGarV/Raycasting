use nalgebra_glm::Vec2;
use minifb::{Key, Window, MouseMode};
use crate::maze::is_wall;

pub struct Player {
    pub pos: Vec2,
    pub a: f32,
    pub fov: f32,
    pub prev_mouse_x: Option<f32>,
}

impl Player {
    pub fn move_player(&mut self, dir: Vec2, maze: &Vec<Vec<char>>, block_size: usize) {
        let new_pos = self.pos + dir;

        // Verificar colisiones
        let i = (new_pos.x / block_size as f32) as usize;
        let j = (new_pos.y / block_size as f32) as usize;

        if !is_wall(i, j, maze) {
            self.pos = new_pos;
        }
    }

    // Rotar al jugador basado en el movimiento del mouse
    pub fn rotate_with_mouse(&mut self, window: &Window) {
        if let Some((mouse_x, _)) = window.get_mouse_pos(MouseMode::Pass) {
            if let Some(prev_x) = self.prev_mouse_x {
                let delta_x = mouse_x as f32 - prev_x;
                self.a += delta_x * 0.005;
            }

            self.prev_mouse_x = Some(mouse_x);
        }
    }
}

pub fn procces(window: &Window, player: &mut Player, maze: &Vec<Vec<char>>, block_size: usize) {
    let move_speed = 5.0;

    if window.is_key_down(Key::W) {
        let dir = Vec2::new(player.a.cos() * move_speed, player.a.sin() * move_speed);
        player.move_player(dir, maze, block_size);
    }

    if window.is_key_down(Key::A) {
        let dir = Vec2::new(player.a.sin() * move_speed, -player.a.cos() * move_speed);
        player.move_player(dir, maze, block_size);
    }

    if window.is_key_down(Key::S) {
        let dir = Vec2::new(-player.a.cos() * move_speed, -player.a.sin() * move_speed);
        player.move_player(dir, maze, block_size);
    }

    if window.is_key_down(Key::D) {
        let dir = Vec2::new(-player.a.sin() * move_speed, player.a.cos() * move_speed);
        player.move_player(dir, maze, block_size);
    }

    player.rotate_with_mouse(window); // Rotar con el mouse
}
