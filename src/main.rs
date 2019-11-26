extern crate minifb;

use minifb::{Window, Key, WindowOptions};

const WIDTH: usize = 600;
const HEIGHT: usize = 600;
const FRACTAL_DEPTH: u32 = 128;


fn main() {
  let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

  let mut window = match Window::new(
    "Fractal - ESC to exit",
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

  let range = 2.0;

  let mut angle: f64 = 0.0;
  while window.is_open() && !window.is_key_down(Key::Escape) {

    let x_min = 0. - range;
    let y_min = 0. - range;

    let x_max = 0. + range;
    let y_max = 0. + range;

    for y in 0..HEIGHT {
      for x in 0..WIDTH {
        let mut real = map(x as f64, 0., WIDTH as f64, x_min , x_max);
        let mut imag = map(y as f64, 0., HEIGHT as f64, y_min, y_max);

        let mut n = 0;

        while n < FRACTAL_DEPTH {
          let re = real.powf(2.) - imag.powf(2.);
          let im = 2. * real * imag;

          real = re + angle.cos();
          imag = im + angle.sin();

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
    // We unwrap here as we want this code to exit if it fails
    window.update_with_buffer(&buffer).unwrap();
    angle += 0.1;
  }
}


fn map (val: f64, start1: f64, stop1: f64, start2: f64, stop2:f64) -> f64 {
  start2 + (stop2 - start2) * ((val - start1) / (stop1 - start1))
}
