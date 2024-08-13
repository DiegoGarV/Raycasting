use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::{Vec2};
use std::f32::consts::PI;
use std::sync::Arc;
use once_cell::sync::Lazy;

mod framebuffer;
mod bm;
mod color;
mod maze;
mod player; 
mod caster; 
mod texture; 

use std::time::{Duration, Instant};
use framebuffer::Framebuffer;
use player::{procces, Player};
use maze::load_maze;
use caster::{cast_ray, Intersect};
use texture::Texture; 

static WALL1: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("./assets/wallTile1.png")));


fn cell_to_texture_color(cell: char, tx: u32, ty:u32)-> u32{
    let default_color = 0x000000;
    return WALL1.get_pixel_color(tx,ty)
    
}



// recibe donde va a estar, el tamaño de los cuadrados y para ponerle diferentes colores una celda
fn drawcell(framebuffer: &mut Framebuffer, xo: usize, yo: usize, block_size: usize, cell: char){
    for x in xo..xo + block_size{
        for y in yo..yo + block_size{
            if cell != ' '{    
                framebuffer.point(x,y,0x000000);
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
            framebuffer.point(x, y, 0xFFFFA500);
        }
    }
}

fn render2d(framebuffer: &mut Framebuffer, player: &Player, maze: &Vec<Vec<char>>, block_size: usize) {
    // for de dos dimensiones
    for row in 0..maze.len(){
        for col in 0..maze[row].len(){
            drawcell(framebuffer, col * block_size,row * block_size , block_size, maze[row][col]);
        }
    }

    render_2d_player(framebuffer, player, block_size / 10);

    let num_rayos = 5; 
    for i in 0..num_rayos{ 
        let current_ray = i as f32 / num_rayos as f32; 
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray); 
        cast_ray(framebuffer, &maze, player, a, block_size, true);
    }
}

fn render3d(framebuffer: &mut Framebuffer, player: &Player, maze: &Vec<Vec<char>>, block_size: usize){
    let num_rayos = framebuffer.width; 

    let hh = framebuffer.height as f32/ 2.0;

    for i in 0..num_rayos{ 
        let current_ray = i as f32 / num_rayos as f32; 
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray); 
        let intersect = cast_ray(framebuffer, &maze, player, a, block_size, false);

        let stake_heigth = (framebuffer.height as f32 / intersect.distance) * 60.0; 
        let stake_top = (hh - (stake_heigth / 2.0 )) as usize;
        let stake_bottom = (hh + (stake_heigth / 2.0 )) as usize;

        let tx = (intersect.tx as f32 / block_size as f32 * WALL1.width as f32) as u32;
        let texture_height = WALL1.height as f32;
        let texture_width = WALL1.width as f32;

        for y in stake_top..stake_bottom{
            let ty = ((y as f32 - (hh - (stake_heigth / 2.0))) / stake_heigth * texture_height) as u32;
            let color = cell_to_texture_color(intersect.impact, tx, ty);
            framebuffer.point(i, y, color);
        }
    }

}

fn main() {
    // Cargar el mapa
    let maze = load_maze("./map.txt");

    // Definir un block_size fijo
    let block_size = 40;

    // Calcular las dimensiones del framebuffer basadas en el tamaño del mapa y el block_size
    let framebuffer_width = maze[0].len() * block_size;
    let framebuffer_height = maze.len() * block_size;

    let frame_delay = Duration::from_millis(1);

    let mut framebuffer = framebuffer::Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new(
        "Maze Game",
        framebuffer_width,
        framebuffer_height,
        WindowOptions::default(),
    ).unwrap();

    // Oculta el cursor
    window.set_cursor_visibility(false);

    let mut player1 = Player{
        pos: Vec2::new(100.0, 100.0),
        a: PI/3.0,
        fov: PI/3.0, 
        prev_mouse_x: None,
    };

    let mut mode = "3D";
    
    let mut last_time = Instant::now();
    let mut frame_count = 0;


    while window.is_open() {

        let current_time = Instant::now();
        let duration = current_time.duration_since(last_time);

        // Centra el cursor
        let (window_width, window_height) = window.get_size();

        if window.is_key_down(Key::Escape) {
            break;
        }

        if window.is_key_down(Key::M){
            mode = if mode == "2D" {"3D"} else {"2D"};
        }

        if mode == "3D" {
            // Movimiento del personaje
            procces(&mut window, &mut player1, &maze, block_size);
        }

        framebuffer.clear();

        // Renderiza el juego
        if mode == "2D"{
            render2d(&mut framebuffer, &mut player1, &maze, block_size);
        }
        else{
            render3d(&mut framebuffer, &mut player1, &maze, block_size);
             
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
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }

    

}