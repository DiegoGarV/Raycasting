use crate::framebuffer::Framebuffer; 
use crate::player::Player;


pub struct Intersect{
    pub distance: f32, 
    pub impact: char,
    pub tx: usize,
}

pub fn cast_ray(
    framebuffer: &mut Framebuffer, 
    maze: &Vec<Vec<char>>, 
    player: &Player, 
    a: f32, 
    block_size: usize, 
    draw_line: bool, 
) -> Intersect { 
    let mut d = 0.0;
    loop{ 
        let cos = d* a.cos();
        let sin = d* a.sin();

        let x = (player.pos.x + cos) as usize; 
        let y = (player.pos.y + sin) as usize;

        let i = x / block_size; 
        let j = y / block_size; 

        let tx = if cos.abs() > sin.abs() {
            y - j * block_size // Impacto en pared vertical
        } else {
            x - i * block_size // Impacto en pared horizontal
        };
        
        if draw_line{
            framebuffer.point(x, y, 0x000000);
        }

        if maze[j][i] != ' ' {
            return Intersect{
                distance:d, 
                impact: maze[j][i],
                tx: tx,
            }
        }


        d += 1.0; 
    }
}