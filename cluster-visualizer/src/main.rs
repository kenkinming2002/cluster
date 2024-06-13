mod render;
use render::Render;

mod cluster_state;
use cluster_state::ClusterStateAny;

use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

struct App {
    event_pump : EventPump,
    canvas : Canvas<Window>,
    cluster_state : Option<ClusterStateAny>,
}

impl App {
    fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let event_pump = sdl_context.event_pump().unwrap();
        let canvas = video_subsystem.window("rust-sdl2 demo", 1024, 720)
            .resizable()
            .build().unwrap()
            .into_canvas()
            .build().unwrap();

        Self {
            event_pump,
            canvas,
            cluster_state : Some(ClusterStateAny::reset()),
        }
    }

    fn redraw(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.cluster_state.as_ref().take().unwrap().render(Render::new(&mut self.canvas));
        self.canvas.present();
    }

    fn run(&mut self) {
        loop {
            self.redraw();
            match self.event_pump.wait_event() {
                Event::Quit {..} => break,
                Event::KeyDown { keycode : Some(Keycode::R), .. } => self.cluster_state = Some(ClusterStateAny::reset()),
                Event::KeyDown { keycode : Some(Keycode::K), .. } => self.cluster_state = Some(ClusterStateAny::from_sample_values_k_means(self.cluster_state.take().unwrap().into_sample_values())),
                Event::KeyDown { keycode : Some(Keycode::G), .. } => self.cluster_state = Some(ClusterStateAny::from_sample_values_gaussian_mixture(self.cluster_state.take().unwrap().into_sample_values())),
                Event::KeyDown { keycode : Some(Keycode::A), .. } => self.cluster_state = Some(ClusterStateAny::from_sample_values_agglomerative_single_linkage(self.cluster_state.take().unwrap().into_sample_values())),
                Event::KeyDown { keycode : Some(Keycode::S), .. } => self.cluster_state = Some(self.cluster_state.take().unwrap().step()),
                _ => {},
            }
        }
    }
}

pub fn main() {
    eprintln!("Intructions:");
    eprintln!("  Press r for get new random samples.");
    eprintln!("  Press k for k-means clustering.");
    eprintln!("  Press g for gaussian mixture model.");
    eprintln!("  Press a for agglomerative single linkage model.");
    eprintln!("  Press s to step through the algorithm.");

    App::new().run()
}
