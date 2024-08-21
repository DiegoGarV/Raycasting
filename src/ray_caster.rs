use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::Goal;
pub struct Intersect {
    pub distance: f32,
    pub impact: char,
    pub texture_index: f32,
    pub in_goal: bool,
    pub d_to_goal: f32,
}

pub fn cast_ray(
    framebuffer: &mut Framebuffer,
    maze: &Vec<Vec<char>>,
    player: &Player,
    a: f32,
    block_size: usize,
    draw_line: bool,
    goal: &Goal,
)-> Intersect{
    let mut d = 0.0;
    let mut in_goal = false;
    let mut d_to_goal = 0.0;
    loop{
        let cos = d*a.cos();
        let sin = d*a.sin();
        let x = (player.pos.x + cos) as usize;
        let y = (player.pos.y + sin) as usize;

        let i = x/block_size;
        let j = y/block_size;

        if (maze[i][j] != ' ') & (maze[i][j] != 'p'){
            if maze[i][j] == 'g'{
                if (x) as usize == (goal.pos.x) as usize{
                    if (y) as usize == (goal.pos.y) as usize{
                        in_goal= true;
                        d_to_goal = d;
                    }
                }

            } else{
                return Intersect{
                    distance: d,
                    impact: maze[i][j],
                    texture_index: ((x+y)%block_size) as f32/block_size as f32,
                    in_goal,
                    d_to_goal,
                };
            }
        }
        if draw_line{
            framebuffer.point(x, y);
            d+=10.0;
        }else{
            d+=1.0;
        }
    }
}