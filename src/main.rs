use std::{thread, time};
use std::io::{self, Error, Write};
use std::cmp;

fn circle(a: f64, b: f64, r: f64, x: f64, y: f64) -> bool {
    ((x - a).powf(2.) + (y - b).powf(2.)) <= r.powf(2.)
}

fn main() {
    const WIDTH: usize = 100;
    const HEIGHT: usize = 25;
    const RATIO: f64 = HEIGHT as f64 / WIDTH as f64;
    let max_dist: f64 = 1.0;

    let mut stdout = io::stdout();
    let gradient = "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'. ".to_string();


    // assert_ne!(WIDTH, HEIGHT, "WIDTH should not equal to HEIGHT, because inside the loop function round() will think that the last entry in the first row is in the second row");
    
    let mut screen = [' '; WIDTH * HEIGHT + 1];
    screen[(WIDTH * HEIGHT) as usize] = '\0';

    // loop {
    let ten_millis = time::Duration::from_millis(10);
    let mut t: f64 = 0.0;
    loop {
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                let index = j + i * WIDTH;
                
                
                let y = (2. * i as f64 / HEIGHT as f64 - 1.0) * 2.0 *RATIO;
                let x = (2. * j as f64 / WIDTH as f64 - 1.);
                // let x = x

                // let dist = (x as i32 - 2).pow(2) + (y as i32 - 2).pow(2);
                let (a, b) = (0.0 + 0.8*t.sin(), 0.0 + 0.35*(t).cos());
                let dist = ((x - a).powf(2.) + (y - b).powf(2.)).sqrt();
                let mut gradient_index = dist * (gradient.len() as f64 - 1.0) / max_dist;
                if gradient_index.lt(&0.0) {
                    gradient_index = 0.0;
                } else if gradient_index.gt(&((gradient.len() - 1) as f64)) {
                    gradient_index = (gradient.len() - 1) as f64;
                }
                let mut pixel = gradient.chars().nth(gradient_index.floor() as usize).unwrap();

                // if circle(0.0 + 0.8*t.sin(), 0.0, 0.3, x, y) {
                //     pixel = '@';
                // }

                screen[index] = pixel;

                t += 0.000005;
            }
        }   

        for i in 0..screen.len() {
            write!(stdout, "{}", screen[i]);
        }
        stdout.flush();


        thread::sleep(ten_millis);
    }   
} 
