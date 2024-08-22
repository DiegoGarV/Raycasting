use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::Vec2;
use core::f32::consts::PI;
use framebuffer::Framebuffer;
use loader::load_maze;
use player::{process_event, Player};
use ray_caster::cast_ray;
use std::time::{ Instant,Duration};
use audio::AudioPlayer;

mod framebuffer;
mod color;
mod loader;
mod player;
mod ray_caster;
mod fps;
mod sprite_loader;
mod audio;

struct Goal{
    pos: Vec2,
    sprite: sprite_loader::Sprite,
}

impl Goal{
    fn new(pos:Vec2, sprite:sprite_loader::Sprite) -> Self{
        Goal{
            pos,
            sprite,
        }
    }
}

fn draw_player_view(
    framebuffer: &mut Framebuffer,
    maze: &Vec<Vec<char>>,
    player: &mut Player,
    block_size: usize,
    scale: usize,
    goal: &Goal
){
    framebuffer.clear();
    framebuffer.set_current_color(0x008dfc);
    sprite_loader::draw_block(framebuffer, player.pos.x as usize-block_size/12 ,player.pos.y as usize-block_size/12, block_size/6);
    framebuffer.set_current_color(0xffffff);
    sprite_loader::render2d(framebuffer, maze, scale, player,false);
    let num_rays = 3;
    
    for i in 0..num_rays{
        let current_ray = i as f32/ num_rays as f32;
        let a = player.a -(player.fov / 2.0) + (player.fov * current_ray);
        cast_ray(framebuffer, maze, player, a, block_size, true, &goal);
    }
}

fn draw_minimap(
    framebuffer: &mut Framebuffer,
    maze: &Vec<Vec<char>>,
    player: &mut Player,
    block_size: usize,
    scale: usize,
){
    let minimap_width = (maze[0].len() * scale / 4) - 6;
    let minimap_height = maze.len() * scale;

    framebuffer.set_current_color(0x000000);
    for x in 0..minimap_width {
        for y in 0..minimap_height {
            framebuffer.point(x, y);
        }
    }

    framebuffer.set_current_color(0x008dfc);
    sprite_loader::draw_block(
        framebuffer, 
        (player.pos.x * scale as f32 / block_size as f32) as usize - block_size / 24,
        (player.pos.y * scale as f32 / block_size as f32) as usize - block_size / 24,
        block_size / 12,
    );

    framebuffer.set_current_color(0xffffff);
    sprite_loader::render2d(framebuffer, maze, scale, player, true);
}

fn playing(screen: &mut usize){
    let goal_name = "./src/sprites/prizes/sandwich.bmp";
    let maze_name = "./src/mazes/maze1.txt";
    let audio_player = AudioPlayer::new("./src/audios/theme_song.mp3");

    let maze = load_maze(maze_name);
    let mut goal = Goal::new(
        Vec2::new(0.0, 0.0),
        sprite_loader::Sprite::new(goal_name));
    let numbers = load_maze("./src/mazes/numbers.txt");

    let window_width = 600;
    let window_height = 600;
    
    let block_size = 600 / maze.len();

    let framebuffer_width = 600;
    let framebuffer_height = 600;

    let mut framebuffer = framebuffer::Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut player = Player::new(block_size);
    let frame_delay = Duration::from_millis(0);

    sprite_loader::init_maze(&mut framebuffer, &maze, block_size, &mut player, &mut goal);
    audio_player.play();

    let mut window = Window::new(
        "Space Sandwich Eaters",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    let mut mode = "3D";
    let mut last_time = Instant::now();
    let mut last_input = Instant::now();
    let mut fps_counter = 0;
    let mut fps_last = 10;
    let wall_1 = sprite_loader::Sprite::new("./src/sprites/walls/wallTile1.bmp");
    let wall_2 = sprite_loader::Sprite::new("./src/sprites/walls/wallTile2.bmp");
    let wall_3 = sprite_loader::Sprite::new("./src/sprites/walls/wallTile3.bmp");
    let sprites = [&wall_1, &wall_2, &wall_3];
    while window.is_open(){
        
        if window.is_key_down(Key::Escape) {
            break;
        }

        if window.is_key_pressed(Key::M, minifb::KeyRepeat::No){
            mode = if mode == "2D" {"3D"} else {"2D"}
        }

        if mode == "2D"{ 
            draw_player_view(&mut framebuffer, &maze, &mut player, block_size,block_size, &mut goal);
        } else {
            sprite_loader::render3d(&mut framebuffer, &maze, &mut player, block_size, &sprites, &mut goal);
            draw_minimap(&mut framebuffer, &maze, &mut player, block_size, 8);

            if last_input.elapsed() >= Duration::from_millis(16) {
                let intersect_f = cast_ray(&mut framebuffer, &maze, &player, player.a, block_size, false, &goal);
                let intersect_b = cast_ray(&mut framebuffer, &maze, &player, player.a + PI, block_size, false, &goal);
                let intersect_l = cast_ray(&mut framebuffer, &maze, &player, player.a - PI / 2.0, block_size, false, &goal);
                let intersect_r = cast_ray(&mut framebuffer, &maze, &player, player.a + PI / 2.0, block_size, false, &goal);
                
                let mut wall_f = false;
                if intersect_f.distance < 8.0{
                    wall_f = true;
                }
                let mut wall_b = false;
                if intersect_b.distance < 8.0{
                    wall_b = true;
                }
                let mut wall_l = false;
                if intersect_l.distance < 8.0{
                    wall_l = true;
                }
                let mut wall_r = false;
                if intersect_r.distance < 8.0{
                    wall_r = true;
                }
                process_event(&mut player, &window, wall_f, wall_b, wall_l, wall_r);
                last_input = Instant::now();
            }
        }

        if player.win_condition{
            *screen= 3;
            break;
        }

        fps_counter += 1;
        if last_time.elapsed() >= Duration::from_secs(1) {
            fps_last = fps_counter;
            fps_counter = 0;
            last_time = Instant::now();
        }
        fps::render_fps(&mut framebuffer, &numbers, fps_last);


        window
            .update_with_buffer(
                &framebuffer.color_array_to_u32(),
                framebuffer_width,
                framebuffer_height,
            )
            .unwrap();
        std::thread::sleep(frame_delay);
    }
}

fn main() {
    let mut screen: usize = 0;

    sprite_loader::pre_play(&mut screen);
    if screen!=0{
        playing(&mut screen);
    }
    if screen==3{
        sprite_loader::post_play();
    }
}
