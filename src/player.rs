use nalgebra_glm::Vec2;
use core::f32::consts::PI;
use minifb::{Key, Window};


const ROTATION_SPEED: f32 = PI/50.0;

pub struct Player{
    pub pos:nalgebra_glm::Vec2,
    pub a: f32,
    pub fov: f32,
    pub win_condition: bool,
    move_speed: f32,
    rotation_speed: f32,
    last_mouse_x: Option<f32>,
}

impl Player{
    pub fn new(block_size: usize)->Self{
        Player{
            pos: Vec2::new(0.0, 0.0),
            a: PI/3.0,
            fov: PI/3.0,
            win_condition: false,
            move_speed: block_size as f32/10.0,
            rotation_speed: ROTATION_SPEED,
            last_mouse_x: None,
        }
    }

    pub fn set_pos(&mut self, x: f32, y: f32){
        self.pos.x = x;
        self.pos.y = y;
    }
    pub fn rotate(&mut self, delta_x: f32){ // true - Right, false - left
        self.a += delta_x * self.rotation_speed;
    }
    pub fn inc_pos(&mut self, direction: Vec2){
        let delta_x = self.move_speed*direction.x;
        let delta_y = self.move_speed*direction.y;

        self.pos.x += delta_x * self.a.cos() - delta_y * self.a.sin();
        self.pos.y += delta_x * self.a.sin() + delta_y * self.a.cos();

    }
}

pub fn process_event(player: &mut Player, window: &Window, wall_f: bool, wall_b: bool, wall_l: bool, wall_r: bool){

    let mut direction = Vec2::new(0.0, 0.0);

    if window.is_key_down(Key::W) && !wall_f {
        direction.x += 1.0;
    }
    if window.is_key_down(Key::A) && !wall_l {
        direction.y -= 1.0;
    }
    if window.is_key_down(Key::S) && !wall_b {
        direction.x -= 1.0;
    }
    if window.is_key_down(Key::D) && !wall_r {
        direction.y += 1.0;
    }

    if direction.magnitude() != 0.0 {
        direction = direction.normalize();
        player.inc_pos(direction);
    }

    if let Some(mouse_pos) = window.get_mouse_pos(minifb::MouseMode::Pass) {
        if let Some(last_x) = player.last_mouse_x {
            let delta_x = mouse_pos.0 - last_x;
            player.rotate(delta_x);
        }
        player.last_mouse_x = Some(mouse_pos.0);
    }
}