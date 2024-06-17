mod utils;

mod render;
use render::Render;

mod clusterer;

use clusterer::random_samples;
use clusterer::image_samples;
use clusterer::ImagePlane;

use clusterer::Clusterer;
use clusterer::NoneClusterer;
use clusterer::KMeansClusterer;
use clusterer::GaussianMixtureClusterer;
use clusterer::AgglomerativeSingleLinkageClusterer;
use clusterer::AffinityPropagationClusterer;
use clusterer::DbscanClusterer;
use clusterer::SlinkClusterer;

use sdl2::event::Event;
use sdl2::event::EventType;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

pub fn main() {
    eprintln!("Intructions:");
    eprintln!("  Press r for get new random samples.");

    eprintln!("  Press numpad 1 to load samples from RG plane of image.");
    eprintln!("  Press numpad 2 to load samples from RB plane of image.");
    eprintln!("  Press numpad 3 to load samples from GB plane of image.");

    eprintln!("  Press k for k-means clustering.");
    eprintln!("  Press g for gaussian mixture model.");
    eprintln!("  Press a for agglomerative single linkage model.");
    eprintln!("  Press p for affinity propagation");
    eprintln!("  Press d for dbscan(Density-based spatial clustering of applications with noise)");
    eprintln!("  Press t to to start/stop stepping through the algorithm automatically");
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
    let mut running = false;
    loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        clusterer.render(Render::new(&mut canvas));
        canvas.present();

        if let Some(event) = if running { event_pump.poll_event() } else { Some(event_pump.wait_event()) } {
            event_pump.disable_event(EventType::KeyDown); // Prevent events from piling up if computation takes a long time
            match event {
                // New clusterer with new samples
                Event::KeyDown { keycode : Some(Keycode::R), .. } => clusterer = NoneClusterer::new(random_samples()),
                Event::KeyDown { keycode : Some(Keycode::Kp1), .. } => clusterer = NoneClusterer::new(image_samples(ImagePlane::RG)),
                Event::KeyDown { keycode : Some(Keycode::Kp2), .. } => clusterer = NoneClusterer::new(image_samples(ImagePlane::RB)),
                Event::KeyDown { keycode : Some(Keycode::Kp3), .. } => clusterer = NoneClusterer::new(image_samples(ImagePlane::GB)),

                // Change Clusterer but keep samples
                Event::KeyDown { keycode : Some(Keycode::K), .. } => clusterer = KMeansClusterer::new(clusterer.into_raw(), 10),
                Event::KeyDown { keycode : Some(Keycode::G), .. } => clusterer = GaussianMixtureClusterer::new(clusterer.into_raw(), 10),
                Event::KeyDown { keycode : Some(Keycode::A), .. } => clusterer = AgglomerativeSingleLinkageClusterer::new(clusterer.into_raw(), 10),
                Event::KeyDown { keycode : Some(Keycode::P), .. } => clusterer = AffinityPropagationClusterer::new(clusterer.into_raw(), -0.1, 0.7),
                Event::KeyDown { keycode : Some(Keycode::D), .. } => clusterer = DbscanClusterer::new(clusterer.into_raw(), 0.03, 8),
                Event::KeyDown { keycode : Some(Keycode::L), .. } => clusterer = SlinkClusterer::new(clusterer.into_raw(), 10),

                // Update Clusterer
                Event::KeyDown { keycode : Some(Keycode::T), .. } => running = !running,
                Event::KeyDown { keycode : Some(Keycode::S), .. } => clusterer.update(),

                // Misc
                Event::Quit {..} => break,
                _ => {},
            }
            event_pump.enable_event(EventType::KeyDown);
        }

        if running {
            clusterer.update();
        }
    }
}

