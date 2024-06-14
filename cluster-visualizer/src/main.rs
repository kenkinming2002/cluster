mod render;
use render::Render;

mod clusterer;

use clusterer::random_samples;

use clusterer::Clusterer;
use clusterer::NoneClusterer;
use clusterer::KMeansClusterer;
use clusterer::GaussianMixtureClusterer;
use clusterer::AgglomerativeSingleLinkageClusterer;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

pub fn main() {
    eprintln!("Intructions:");
    eprintln!("  Press r for get new random samples.");
    eprintln!("  Press k for k-means clustering.");
    eprintln!("  Press g for gaussian mixture model.");
    eprintln!("  Press a for agglomerative single linkage model.");
    eprintln!("  Press s to step through the algorithm.");

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut canvas = video_subsystem.window("rust-sdl2 demo", 1024, 720)
        .resizable()
        .build().unwrap()
        .into_canvas()
        .build().unwrap();

    let mut clusterer = NoneClusterer::new(random_samples()) as Box<dyn Clusterer>;
    loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        clusterer.render(Render::new(&mut canvas));
        canvas.present();

        match event_pump.wait_event() {
            // New clusterer with random samples
            Event::KeyDown { keycode : Some(Keycode::R), .. } => clusterer = NoneClusterer::new(random_samples()),

            // Change Clusterer but keep samples
            Event::KeyDown { keycode : Some(Keycode::K), .. } => clusterer = KMeansClusterer::new(clusterer.into_raw(), 10),
            Event::KeyDown { keycode : Some(Keycode::G), .. } => clusterer = GaussianMixtureClusterer::new(clusterer.into_raw(), 10),
            Event::KeyDown { keycode : Some(Keycode::A), .. } => clusterer = AgglomerativeSingleLinkageClusterer::new(clusterer.into_raw(), 10),

            // Update Clusterer
            Event::KeyDown { keycode : Some(Keycode::S), .. } => clusterer.update(),

            // Misc
            Event::Quit {..} => break,
            _ => {},
        }
    }
}

