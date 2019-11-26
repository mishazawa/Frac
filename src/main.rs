extern crate minifb;

use minifb::{Window, Key, WindowOptions, MouseMode};

const WIDTH: usize = 600;
const HEIGHT: usize = 600;
const ITERATIONS: f64 = 100.0;


fn main() {
  let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

  let mut window = match Window::new(
    "F",
    WIDTH,
    HEIGHT,
    WindowOptions {
      ..WindowOptions::default()
    }) {
      Ok(win) => win,
      Err(err) => {
        println!("Unable to create window {}", err);
        return;
      }
    };

  let mut range = 4.0;

  while window.is_open() && !window.is_key_down(Key::Escape) {
      let mut x_mouse = 0.0;
      let mut y_mouse = 0.0;
      window.get_mouse_pos(MouseMode::Discard).map(|(x, y)| {
        x_mouse = map(x as f64, 0., WIDTH as f64, -1. , 1.);
        y_mouse = map(y as f64, 0., HEIGHT as f64, -1. , 1.);
      });
      let x_min = -2.;
      let y_min = -2.;

      let x_max = x_min + range;
      let y_max = y_min + range;
      let dx = (x_max - x_min) / WIDTH as f64;
      let dy = (y_max - y_min) / HEIGHT as f64;
      let mut y_step = y_min;

      for y in 0..HEIGHT {

        let mut x_step = x_min;

        for x in 0..WIDTH {
          let mut a = x_step as f64;
          let mut b = y_step as f64;

          let mut n = 0.;

          while n < ITERATIONS {
            let aa = a * a;
            let bb = b * b;
            let twoab = 2.0 * a * b;
            a = aa - bb + x_mouse as f64;
            b = twoab + y_mouse as f64;
            if aa + bb > 16.0 {
              break;  // Bail
            }
            n += 1.0;
          }

          let screen_pos = ((y as usize) * WIDTH) + x as usize;
          if ITERATIONS == n {
            buffer[screen_pos] = 0x00;
          } else {
            buffer[screen_pos] = n as u32 * 16 % 255;
          }

          x_step += dx;
        }
        y_step += dy;
      }

      window.get_scroll_wheel().map(|scroll| {
        range += 0.1 * scroll.1 as f64;
        println!("Scrolling {}, {}:{}", range, dx, dy);
      });

      // We unwrap here as we want this code to exit if it fails
      window.update_with_buffer(&buffer).unwrap();
  }
}


fn map (val: f64, start1: f64, stop1: f64, start2: f64, stop2:f64) -> f64 {
  start2 + (stop2 - start2) * ((val - start1) / (stop1 - start1))
}
