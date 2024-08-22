use std::fs::File;
use std::io::BufReader;
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use crate::color::Color;
use bmp::{from_reader, Pixel};
use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::Goal;
use crate::ray_caster::cast_ray;
use crate::audio::AudioPlayer;

pub struct Sprite{
    pub buffer: Vec<Color>,
    pub height: usize,
    pub width: usize,
}

impl Sprite{
    pub fn render_screen(framebuffer: &mut Framebuffer, sprite: &Sprite) {
        for x in 0..framebuffer.width - 1 {
            // Transforming x to sprite coordinates
            let trans_x = ((x as f32) / framebuffer.width as f32) * (sprite.width) as f32;
            let current_line = sprite.get_line(trans_x as usize);
            for y in 0..framebuffer.height - 1 {
                // Transforming y to sprite coordinates
                let trans_y = ((y as f32) / framebuffer.height as f32)* sprite.height as f32;
                let color = Color::to_hex(&current_line[trans_y as usize]);
                framebuffer.set_current_color(color);
                framebuffer.point(x as usize, y);
            }
        }
    }
    
    pub fn get_line(&self, x:usize)->Vec<Color>{
        let mut line_buffer: Vec<Color> = Vec::new();
        for y in 0..self.height{
            line_buffer.push(self.buffer[x+(y*self.width)]);
        }
        return line_buffer;
    }

    pub fn new(file_path: &str) -> Self{
        let mut buffer: Vec<Color>  = Vec::new();
        let mut height = 0;
        let mut width = 0;
        match read_bmp_to_framebuffer(file_path) {
            Ok(sprite) => {
                buffer=sprite;
            }
            Err(e) => eprintln!("Failed to read BMP file: {}", e),
        }
        match get_dimentions(file_path) {
            Ok(dim) => {
                height = dim.0;
                width = dim.1;
            }
            Err(e) => eprintln!("Failed to read BMP file: {}", e),
        }

        Sprite{
            buffer,
            height,
            width,
        }
    }
}
pub fn read_bmp_to_framebuffer(file_path: &str) -> Result<Vec<Color>, String>{
    let file = File::open(file_path).map_err(|e| e.to_string())?;
    let mut reader = BufReader::new(file);

    // Load the BMP image

    let bmp_image = from_reader(&mut reader).map_err(|e| e.to_string())?;

    let mut framebuffer: Vec<Color> = Vec::new();

    for (x, y) in bmp_image.coordinates() {
        let pixel: Pixel = bmp_image.get_pixel(x, y);
        let color_value: u32 = ((pixel.r as u32) << 16) | ((pixel.g as u32) << 8) | (pixel.b as u32);
        framebuffer.push(Color::from_hex(color_value));
    }
    Ok(framebuffer)
}

pub fn get_dimentions(file_path: &str) -> Result<(usize,usize), String>{
    let file = File::open(file_path).map_err(|e| e.to_string())?;
    let mut reader = BufReader::new(file);

    // Load the BMP image

    let bmp_image = from_reader(&mut reader).map_err(|e| e.to_string())?;
    let height = bmp_image.get_height();
    let width = bmp_image.get_width();
    Ok((height as usize,width as usize))
}

pub fn init_maze(
    framebuffer: &mut Framebuffer, 
    maze: &Vec<Vec<char>>,
    block_size: usize,
    player: &mut Player,
    goal: &mut Goal,
){

    for row in 0..maze.len(){
        for col in 0..maze[row].len(){
                if maze[row][col] =='p'{
                    player.set_pos((row*block_size) as f32,
                     (col*block_size) as f32);
                } else if maze[row][col] =='g'{
                    goal.pos.x =((row)*block_size +block_size/2) as f32;
                    goal.pos.y = (col*block_size +block_size/2) as f32;
                }
            
                match maze[row][col] {
                    'g' =>(),
                    ' ' => (),
                    'p' =>(),
                    _ => {
                        draw_block(framebuffer, row*block_size, col*block_size, block_size);
                    },
                }
            
        }
    }
}

pub fn render2d(
    framebuffer: &mut Framebuffer, 
    maze: &Vec<Vec<char>>,
    block_size: usize,
    player: &mut Player,
    minimaze: bool
){
    for row in 0..maze.len(){
        for col in 0..maze[row].len(){
                match maze[row][col] {
                    'g' => {
                        framebuffer.set_current_color(0x03fc0f);
                        draw_block(framebuffer,
                             ((row as f32+0.25)*block_size as f32) as usize, 
                             ((col as f32+0.25)*block_size as f32) as usize, 
                             block_size/2);
                    },
                    ' ' => (),
                    'p' =>(),
                    _ => {
                        draw_block(framebuffer, row*block_size, col*block_size, block_size);
                    },
                }
                framebuffer.set_current_color(0xffffff);
        }
    }
    if !minimaze{
        if (maze[(player.pos.x/block_size as f32) as usize][(player.pos.y/block_size as f32) as usize])=='g'{
            player.win_condition=true;
        }
    }
}

pub fn draw_block(framebuffer: &mut Framebuffer, xo: usize, yo: usize, block_size: usize){
    for i in 0..block_size{
        for j in 0..block_size{
            framebuffer.point(xo+i, yo+j)
        }
    }
}

pub fn render3d(
    framebuffer: &mut Framebuffer,
    maze: &Vec<Vec<char>>,
    player: &mut Player,
    block_size: usize,
    sprites: &[&Sprite],
    goal: &mut Goal,
) {
    let num_rays = framebuffer.width;
    let hh = (framebuffer.height / 2) as f32;
    let mut try_sprite = false;
    let mut sprite_center = 0;
    let mut sprite_distance = 0.0;
    let background_color = Color::from_hex(0x323638);
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        let intersect = cast_ray(framebuffer, maze, player, a, block_size, false, goal);

        if intersect.in_goal{
            try_sprite = true;
            sprite_center = i;
            sprite_distance = intersect.d_to_goal;
        }
        let mut sprite_index = 2;
        if intersect.impact=='+'{
            sprite_index = 0;
        } else if intersect.impact=='-'{
            sprite_index = 1;
        }

        let d_to_wall = if intersect.distance > 10.0 {
            intersect.distance
        } else {
            10.0
        };
        let d_to_plane: f32 = block_size as f32;

        let current_line = sprites[sprite_index].get_line((intersect.texture_index * sprites[sprite_index].width as f32) as usize);

        let stake_height = ((hh + block_size as f32) as f32 / d_to_wall) * d_to_plane;
        let stake_top = (hh + (stake_height / 2.0)) as usize;
        let stake_bottom = (hh - (stake_height / 2.0)) as usize;

        for y in 0..framebuffer.height {
            if (y > stake_bottom) & (y < stake_top) {
                let trans_y =
                    (sprites[sprite_index].height as f32) * (y as f32 - hh + (stake_height / 2.0)) / stake_height;
                framebuffer.set_current_color(Color::to_hex(&current_line[trans_y as usize]));
                framebuffer.point(i, y);
            } else if y <= stake_bottom {
                framebuffer.set_current_color(Color::to_hex(&(background_color*(1.5-(y as f32/hh)))));
            } else {
                framebuffer.set_current_color(Color::to_hex(&(background_color*(-0.5+(y as f32/hh)))));
            }
            framebuffer.point(i, y);
        }
    }
    if try_sprite & (sprite_distance>10.0){
        draw_sprite(framebuffer, block_size, goal, sprite_distance, sprite_center)
    } else if try_sprite & (sprite_distance<10.0){
        player.win_condition=true;
    }
    framebuffer.set_current_color(0xffffff);
}

pub fn draw_sprite(
    framebuffer: &mut Framebuffer,
    block_size: usize,
    goal: &mut Goal,
    sprite_distance: f32,
    sprite_center: usize
){

        let hh = (framebuffer.height / 2) as f32;
        let sprite_height = ((hh/2.0 + block_size as f32) as f32 / sprite_distance) * block_size as f32;
        let draw_start_y = (hh - (sprite_height / 2.0)) as usize;
        let draw_end_y = (hh + (sprite_height / 2.0)) as usize;

        let draw_start_x = -sprite_height as i32/2 +sprite_center as i32;
        let mut draw_end_x = sprite_height as i32/2 + sprite_center as i32;
        if draw_end_x >= framebuffer.width as i32 {draw_end_x = framebuffer.width as i32- 1};

        for x in draw_start_x..draw_end_x{
            if x >= 0{
                let trans_x = ((x as f32-draw_start_x as f32)/sprite_height as f32)*(goal.sprite.height) as f32;
                let current_line = goal.sprite.get_line(trans_x as usize);
                for y in draw_start_y..draw_end_y{
                    let trans_y =
                    (goal.sprite.height as f32) * (y as f32 - hh + (sprite_height / 2.0)) / sprite_height;
                    let color = Color::to_hex(&current_line[trans_y as usize]);
                    if color!=0xFFFFFF{
                        framebuffer.set_current_color(color);
                        framebuffer.point(x as usize, y);
                    }

                }
            }
        }
    
}

pub fn pre_play(screen: &mut usize) {
    let home = Sprite::new("./src/sprites/screens/start/home_screen.bmp");

    let window_width = 600;
    let window_height = 600;
    let framebuffer_width = 600;
    let framebuffer_height = 600;
    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Space Sandwich Eaters",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    // Starting menu window loop
    while window.is_open() {
        // Cases for closing window
        if window.is_key_down(Key::Escape) {
            break;
        }
        if window.is_key_down(Key::Enter) {
            *screen = 1;
            break;
        }

        Sprite::render_screen(&mut framebuffer, &home);

        window
            .update_with_buffer(
                &framebuffer.color_array_to_u32(),
                framebuffer_width,
                framebuffer_height,
            )
            .unwrap();
        std::thread::sleep(Duration::from_millis(0));
    }
}

pub fn post_play() {
    let part1 = Sprite::new("./src/sprites/screens/end/Part_1.bmp");
    let part2 = Sprite::new("./src/sprites/screens/end/Part_2.bmp");
    let part3 = Sprite::new("./src/sprites/screens/end/Part_3.bmp");
    let part4 = Sprite::new("./src/sprites/screens/end/Part_4.bmp");
    let mut animation_frame = 0;
    let window_width = 600;
    let window_height = 600;
    let framebuffer_width = 600;
    let framebuffer_height = 600;
    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    let audio_player = AudioPlayer::new("./src/audios/victory.mp3");

    Sprite::render_screen(&mut framebuffer, &part1);
    let mut window = Window::new(
        "Space Sandwich Eaters",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    audio_player.play();

    while window.is_open() {
        if window.is_key_down(Key::Escape) | window.is_key_down(Key::Enter){
            break;
        }

        if animation_frame == 0 {
            Sprite::render_screen(&mut framebuffer, &part1);
        } else if animation_frame == 1 {
            Sprite::render_screen(&mut framebuffer, &part2);         
        } else if animation_frame == 2 {
            Sprite::render_screen(&mut framebuffer, &part3);         
        } else if animation_frame == 3 {
            Sprite::render_screen(&mut framebuffer, &part4); 
            animation_frame = -1;        
        }
        animation_frame += 1;

        window
            .update_with_buffer(
                &framebuffer.color_array_to_u32(),
                framebuffer_width,
                framebuffer_height,
            )
            .unwrap();
        std::thread::sleep(Duration::from_millis(250));
    }
}