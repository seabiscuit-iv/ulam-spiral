use core::panic;
use std::{path::Iter};

use macroquad::{prelude::*, time};

const FONT_SIZE : f32 = 40.0;
const X_SPACE : f32 = 40.0;
const Y_SPACE : f32 = 40.0;
const RADIUS: f32 = 5.0;

const NUMBERS: bool = false;

enum Side {
    Right,
    Up,
    Down,
    Left
}

#[macroquad::main("Main")]
async fn main() {    
    use Side::*;

    let mut scale = 1.0;

    loop {
        clear_background(BLACK);

        let (w, h) = (screen_width(), screen_height());

        let limit = (2*(scale as u32 + 8)).pow(3);

        // draw_circle(w/2.0, h/2.0, 8.0, WHITE);
        for i in 1..limit {
            if i == 1 {
                draw_num(i, (0, 0), (w, h), scale);

                continue;
            } 

            let mut l = 1;

            let layer = loop {
                let pow = 2*l - 1;
                if i <= pow * pow {
                    break l;
                }

                l += 1;
            };

            //on layer i, there are 2i numbers per side
            //8i numbers per layer, excluding first layer

            let basenum = if layer == 1 {1} else { (2*(layer-1) - 1).pow(2) };

            let side = match (i - basenum - 1) / (2*(layer-1)) {
                0 => Right,
                1 => Up,
                2 => Left,
                3 => Down,
                _ => { 
                    panic!("Side of 4 calculated, this should not be possible")
                }
            };

            let side_base = match side {
                Right => basenum,
                Up => basenum + 2*(layer-1),
                Left => basenum + 4*(layer-1),
                Down => basenum + 6*(layer-1),
            };

            // first degree in the direction of side is now layer
            // find the second degree, which must be in the range (-layer, layer]

            //this can be in the range [1, 2*layer]
            let side_offset = i-side_base;

            let side_coord: i32 = (side_offset as i32) - ((layer-1) as i32);

            let coords = match side {
                Right => (layer as i32 - 1, side_coord),
                Up => (-side_coord, layer as i32 - 1),
                Left => (-(layer as i32 - 1), -side_coord),
                Down => (side_coord, -(layer as i32 - 1)),
            };

            
            //text drawing
            draw_num(i, coords, (w, h), scale);
        }

        scale += time::get_frame_time() / 4.0;

        next_frame().await
    }
}


fn draw_num(num: u32, coords: (i32, i32), center: (f32, f32), scale: f32) {
    
    let fontsize = FONT_SIZE / ((1.18_f32).powf((num as f32).log10().floor() + 1.0));

    let text_dimensions = measure_text(&num.to_string(), None, fontsize as u16, 1.0);

    let x = (center.0/2.0 +  X_SPACE * coords.0 as f32 * (1.0/scale));
    let y = (center.1/2.0 - Y_SPACE * coords.1 as f32 * (1.0/scale));

    //text size debugging
    // draw_rectangle_lines(x - text_dimensions.width/2.0, y - text_dimensions.height/2.0, text_dimensions.width, text_dimensions.height, 1.0, RED);

    if NUMBERS {
        draw_text(&num.to_string(), x - text_dimensions.width/2.0, y + text_dimensions.height/2.0, fontsize, WHITE);
    } else {
        draw_circle(x, y, RADIUS / scale, WHITE);
    }
    
}