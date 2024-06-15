use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::Point;

pub struct Render<'a> {
    canvas : &'a mut Canvas<Window>,
    width : u32,
    height : u32,
}

impl<'a> Render<'a> {
    pub fn new(canvas : &'a mut Canvas<Window>) -> Self {
        let (width, height) = canvas.window().size();
        Self { canvas, width, height, }
    }

    pub fn draw_point(&mut self, r : u8, g : u8, b : u8, x : f64, y : f64, size : f64) {
        self.canvas.set_draw_color(Color::RGB(r, g, b));
        self.canvas.set_scale(size as f32, size as f32).unwrap();
        self.canvas.draw_point(((x / size * self.width as f64) as i32, (y / size * self.height as f64) as i32)).unwrap();
    }

    pub fn draw_line(&mut self, r : u8, g : u8, b : u8, x1 : f64, y1 : f64, x2 : f64, y2 : f64) {
        self.canvas.set_draw_color(Color::RGB(r, g, b));
        self.canvas.set_scale(1.0, 1.0).unwrap();
        self.canvas.draw_line(
            ((x1 * self.width as f64) as i32, (y1 * self.height as f64) as i32),
            ((x2 * self.width as f64) as i32, (y2 * self.height as f64) as i32),
        ).unwrap();
    }

    pub fn draw_ellipse_scaled(&mut self, r : u8, g : u8, b : u8, x : f64, y : f64, rx : f64, ry : f64, angle : f64, sx : f64, sy : f64) {
        self.canvas.set_draw_color(Color::RGB(r, g, b));
        self.canvas.set_scale(1.0, 1.0).unwrap();

        const STEP: usize = 200;
        for step in 0..STEP {
            let angle0 = ((step + 0) as f64 / STEP as f64) * 2.0 * std::f64::consts::PI;
            let angle1 = ((step + 1) as f64 / STEP as f64) * 2.0 * std::f64::consts::PI;

            let x0 = rx * angle0.cos();
            let y0 = ry * angle0.sin();

            let x1 = rx * angle1.cos();
            let y1 = ry * angle1.sin();

            let real_x0 = x + (x0 * angle.cos() - y0 * angle.sin()) * sx;
            let real_y0 = y + (x0 * angle.sin() + y0 * angle.cos()) * sy;

            let real_x1 = x + (x1 * angle.cos() - y1 * angle.sin()) * sx;
            let real_y1 = y + (x1 * angle.sin() + y1 * angle.cos()) * sy;

            let point0 = Point::new((real_x0 * self.width as f64) as i32, (real_y0 * self.height as f64) as i32);
            let point1 = Point::new((real_x1 * self.width as f64) as i32, (real_y1 * self.height as f64) as i32);

            self.canvas.draw_line(point0, point1).unwrap();
        }
    }
}
