use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::{Vec2};
use std::f32::consts::PI;
use std::sync::Arc;
use once_cell::sync::Lazy;

mod framebuffer;
mod color;
mod maze;
mod player; 
mod cast_ray; 
mod texture; 

use std::time::{Duration, Instant};
use framebuffer::Framebuffer;
use player::{procces, Player};
use maze::load_maze;
use cast_ray::{cast_ray, Intersect};
use texture::Texture; 

static WALL1: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("./assets/DarkWall_S.jpg")));

fn cell_to_texture_color(cell: char, tx: u32, ty:u32)-> u32{
    let default_color = 0x000000;
    return WALL1.get_pixel_color(tx,ty)   
}

// recibe donde va a estar, el tamaño de los cuadrados y para ponerle diferentes colores una celda
fn drawcell(framebuffer: &mut Framebuffer, xo: usize, yo: usize, block_size: usize, cell: char){
    for x in xo..xo + block_size{
        for y in yo..yo + block_size{
            if cell != ' '{    
                framebuffer.point(x as isize, y as isize);
            }
        }
    }

}

fn render_2d_player(framebuffer: &mut Framebuffer, player: &Player, block_size: usize) {
    let player_size = block_size ; // Tamaño del jugador (puede ser ajustado)
    let player_x = player.pos.x as usize;
    let player_y = player.pos.y as usize;

    for y in player_y..(player_y + player_size) {
        for x in player_x..(player_x + player_size) {
            framebuffer.point(x as isize, y as isize);
        }
    }
}

fn render_2d(framebuffer: &mut Framebuffer, player: &Player) {
    let maze = load_maze("./map.txt");
    let block_size = 35;

    // for de dos dimensiones
    for row in 0..maze.len(){
        for col in 0..maze[row].len(){
            drawcell(framebuffer, col * block_size,row * block_size , block_size, maze[row][col]);
        }
    }

    render_2d_player(framebuffer, player, 5);

    let num_rayos = 5; 
    for i in 0..num_rayos{ 
        let current_ray = i as f32 / num_rayos as f32; 
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray); 
        cast_ray(framebuffer, &maze, player, a, block_size, true);
    }
}

fn render_3d(framebuffer: &mut Framebuffer, player: &Player){
    let maze = load_maze("./map.txt");
    let num_rayos = framebuffer.width; 
    let block_size = 35; 

    let hh = framebuffer.height as f32/ 2.0;

    for i in 0..num_rayos{ 
        let current_ray = i as f32 / num_rayos as f32; 
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray); 
        let intersect = cast_ray(framebuffer, &maze, player, a, block_size, false);

        let stake_heigth = (framebuffer.height as f32 / intersect.distance) * 60.0; 

        let stake_top = (hh - (stake_heigth / 2.0 )) as usize;
        let stake_bottom = (hh + (stake_heigth / 2.0 )) as usize;

        for y in stake_top..stake_bottom{
            framebuffer.point(i as isize, y as isize);
        }
    }

}

fn main() {
    let window_width = 1300;
    let window_height = 900;

    let framebuffer_width = 1300;
    let framebuffer_height = 900;

    let frame_delay = Duration::from_millis(1);

    let mut framebuffer = framebuffer::Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new(
        "MAZE GAME",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    let mut player1 = Player{
        pos: Vec2::new(100.0, 100.0),
        a: PI/3.0,
        fov: PI/3.0, 
    };

    let mut mode = "2D";
    
    let mut last_time = Instant::now();
    let mut frame_count = 0;


    while window.is_open() {

        let current_time = Instant::now();
        let duration = current_time.duration_since(last_time);

        if window.is_key_down(Key::Escape) {
            break;
        }

        if window.is_key_down(Key::M){
            mode = if mode == "2D" {"3D"} else {"2D"};
        }
        procces(&mut window, &mut player1); // para los controles y movimiento 

       

        framebuffer.clear();

        if mode == "2D"{
            render_2d(&mut framebuffer, &mut player1);
        }
        else{
             // Incrementar el contador de frames
             render_3d(&mut framebuffer, &mut player1);
             
        }
            
        frame_count += 1;
        
        // Calcular y mostrar FPS cada segundo
        if duration >= Duration::from_secs(1) {
            let fps = frame_count as f64 / duration.as_secs_f64();
            println!("FPS: {}", fps);
            frame_count = 0;
            last_time = Instant::now();
        }

        window
            .update_with_buffer(
                &framebuffer.buffer.iter().map(|color| color.to_u32()).collect::<Vec<u32>>(),
                framebuffer_width,
                framebuffer_height
            )
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}