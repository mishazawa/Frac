extern crate minifb;

use minifb::{Window, Key, WindowOptions, MouseMode};

const WIDTH: usize = 600;
const HEIGHT: usize = 600;
const FRACTAL_DEPTH: u32 = 512;


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

  let mut range = 2.0;

  while window.is_open() && !window.is_key_down(Key::Escape) {
    let mut x_mouse = 0.0;
    let mut y_mouse = 0.0;

    window.get_mouse_pos(MouseMode::Discard).map(|(x, y)| {
      x_mouse = map(x as f64, 0., WIDTH as f64, -1. , 1.);
      y_mouse = map(y as f64, 0., HEIGHT as f64, -1. , 1.);
    });

    let x_min = 0. - range;
    let y_min = 0. - range;

    let x_max = 0. + range;
    let y_max = 0. + range;

    for y in 0..HEIGHT {
      for x in 0..WIDTH {
        let mut real = map(x as f64, 0., WIDTH as f64, x_min , x_max);
        let mut imag = map(y as f64, 0., HEIGHT as f64, y_min, y_max);

        let origin_re = real;
        let origin_im = imag;

        let mut n = 0;

        while n < FRACTAL_DEPTH {
          let re = real.powf(2.) - imag.powf(2.);
          let im = 2. * real * imag;

          real = re + x_mouse;
          imag = im + y_mouse;

          if (real + imag).abs() > 16.0 {
            break;  // Bail
          }
          n += 1;
        }

        let screen_pos = ((y as usize) * WIDTH) + x as usize;
        if FRACTAL_DEPTH == n {
          buffer[screen_pos] = 0x00;
        } else {
          buffer[screen_pos] = n * 16 % 255;
        }
      }
    }

    window.get_scroll_wheel().map(|scroll| {
      if scroll.1 != 0. {
        range /= 2.;
      }
      println!("Zoom {}", range);
    });

    // We unwrap here as we want this code to exit if it fails
    window.update_with_buffer(&buffer).unwrap();
  }
}


fn map (val: f64, start1: f64, stop1: f64, start2: f64, stop2:f64) -> f64 {
  start2 + (stop2 - start2) * ((val - start1) / (stop1 - start1))
}
