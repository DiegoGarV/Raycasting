use crate::framebuffer::Framebuffer;

pub fn render_fps(framebuffer: &mut Framebuffer, numbers: &Vec<Vec<char>>, num: usize){
    framebuffer.set_current_color(0xffffff);
    text_format(framebuffer, numbers, num);
}

pub fn text_format(framebuffer: &mut Framebuffer, numbers: &Vec<Vec<char>>, num: usize, ){
    for i in 0..6{
        for col in 0..numbers[i].len(){
            if numbers[i][col]!=' '{
                render_box(framebuffer, (i+2)*3, (col+50)*3, 3, 3);
            }  
        }
    }
    let num_str: Vec<char>= num.to_string().chars().rev().collect();
    let size = num_str.len();
    for digit_pos in 0..size{
        let digit = num_str[size-digit_pos-1].to_digit(10).expect("Not a valid digit") as usize;
        for i in 0..6{
            for col in 0..numbers[(6*(digit+1))+i-1].len(){
                if numbers[(6*(digit+1))+i-1][col]!=' '{
                    render_box(framebuffer, (i+1)*3, (col+50)*3+(45+12*digit_pos), 3, 3);
                }  
            }
        }
    }

}

pub fn render_box(framebuffer: &mut Framebuffer, xo: usize, yo: usize, w: usize, h: usize){
    for i in xo..xo+h{
        for j in yo..yo+w{
            framebuffer.point(j, i);
        }
    }
}