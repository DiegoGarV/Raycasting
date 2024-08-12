use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::color::Color;

pub fn cast_ray(
    framebuffer: &mut Framebuffer,
    map: &Vec<Vec<char>>,
    player: &Player,
    a: f32,
    block_size: usize,
) {
    let mut d= 0.0;

    framebuffer.set_current_color(Color::from_hex("ray", 0xFFF333));

    loop {
        let cos = d * a.cos();
        let sin = d * a.sin();
        let x = (player.pos.x + cos) as usize;
        let y = (player.pos.y + sin) as usize;

        // coordinates in pixels to indices in the maze
        let i = x / block_size;
        let j = y / block_size;

        // if wall breaks loop
        if map[j][i] != ' ' {
            return;
        }

        framebuffer.point(x as isize, y as isize);

        d += 0.1;
    }
}