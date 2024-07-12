mod ui;
use ui::UiExt;

use posterize::PosterizeMethod;
use posterize::ClusterInit;

use math::prelude::*;

use eframe::egui;

use image::EncodableLayout;
use image::io::Reader as ImageReader;
use image::Rgb;

use anyhow::Result;
use anyhow::Context;

use std::path::PathBuf;
use std::num::NonZero;

/// Open a image.
///
/// This is fast.
fn open_image(path : &PathBuf) -> Result<image::RgbImage> {
    let image = ImageReader::open(&path)
        .with_context(|| format!("Failed to open image {}", path.display()))?
        .decode().with_context(|| format!("Failed to decode image {}", path.display()))?
        .into_rgb8();

    Ok(image)
}

/// Process a image by posterizing it.
///
/// This takes a long time and need to be done in a separate thread.
fn process_image(image : &image::RgbImage, method: PosterizeMethod) -> image::RgbImage {
    let mut samples = image
        .pixels()
        .map(|pixel| pixel.0.map(|subpixel| subpixel as f64))
        .map(Vector::from_array)
        .collect::<Vec<_>>();

    method.posterize(&mut samples);

    let pixels = samples
        .into_iter()
        .map(Vector::into_array)
        .map(|pixel| Rgb(pixel.map(|subpixel| subpixel as u8)));

    let mut image = image.clone();
    image.pixels_mut().zip(pixels).for_each(|(lhs, rhs)| *lhs = rhs);
    image
}

/// Upload a image to GPU for rendering.
///
/// This is fast.
fn upload_image(ctx: &egui::Context, image: &image::RgbImage) -> egui::TextureHandle {
    let width = image.width();
    let height = image.height();
    let image = egui::ColorImage::from_rgb([width as usize, height as usize], image.as_bytes());
    ctx.load_texture("image", image, egui::TextureOptions::default())
}

struct MyEguiApp {
    method: PosterizeMethod,

    path: Option<PathBuf>,

    input: Option<image::RgbImage>,
    input_texture: Option<egui::TextureHandle>,

    output: Option<image::RgbImage>,
    output_texture: Option<egui::TextureHandle>,
}

impl MyEguiApp {
    fn new() -> Self {
        Self {
            method : PosterizeMethod::KMeans {
                cluster_init : ClusterInit::KMeanPlusPlus,
                cluster_count : NonZero::new(3).unwrap(),
            },
            path: None,
            input : None,
            output : None,
            input_texture : None,
            output_texture : None,
        }
    }

    fn update_input(&mut self) {
        if let Some(path) = &self.path {
            match open_image(path) {
                Ok(input) => self.input = Some(input),
                Err(e) => eprintln!("{e}"),
            }
        }
    }

    fn update_output(&mut self) {
        self.output = self.input.as_ref().map(|input| process_image(input, self.method));
    }

    fn update_input_texture(&mut self, ctx: &egui::Context) {
        self.input_texture = self.input.as_ref().map(|input| upload_image(ctx, input));
    }

    fn update_output_texture(&mut self, ctx: &egui::Context) {
        self.output_texture = self.output.as_ref().map(|output| upload_image(ctx, output));
    }

    fn on_open(&mut self, ctx: &egui::Context) {
        if let Some(path) = rfd::FileDialog::new().pick_file() {
            self.path = Some(path);
            self.update_input();
            self.update_input_texture(ctx);
        }
    }

    fn on_run(&mut self, ctx: &egui::Context) {
        self.update_output();
        self.update_output_texture(ctx);
    }

    fn on_save(&mut self) {
        if let Some(output) = &self.output {
            if let Some(path) = rfd::FileDialog::new().save_file() {
                let _ = output.save(path);
            }
        }
    }

    fn ui_controls(&mut self, ctx: &egui::Context) {
        egui::SidePanel::right("Right panel").show(ctx, |ui| {
            // Buttons
            ui.horizontal(|ui| {
                if ui.button("Open").clicked() {
                    self.on_open(ctx);
                }

                if ui.button("Save").clicked() {
                    self.on_save();
                }

                if ui.button("Run").clicked() {
                    self.on_run(ctx);
                }
            });

            // Separator
            ui.separator();

            // Select method
            egui::ComboBox::from_label("Method")
                .selected_text(match self.method {
                    PosterizeMethod::KMeans { .. } => "k means",
                    PosterizeMethod::GaussianMixture { .. } => "gaussian mixture",
                })
                .show_ui(ui, |ui| {
                    let selected = matches!(self.method, PosterizeMethod::KMeans { .. });
                    if ui.selectable_label(selected, "k means").clicked() && !selected {
                        self.method = PosterizeMethod::KMeans {
                            cluster_init : ClusterInit::Llyod,
                            cluster_count : NonZero::new(3).unwrap(),
                        };
                    }

                    let selected = matches!(self.method, PosterizeMethod::GaussianMixture { .. });
                    if ui.selectable_label(selected, "gaussian mixture").clicked() && !selected {
                        self.method = PosterizeMethod::GaussianMixture {
                            cluster_init : ClusterInit::Llyod,
                            cluster_count : NonZero::new(3).unwrap(),
                        };
                    }
                });

            // Method specific options
            match &mut self.method {
                PosterizeMethod::KMeans { cluster_init, cluster_count } => {
                    egui::ComboBox::from_label("Cluster init")
                        .selected_text(match cluster_init {
                            ClusterInit::Llyod => "llyod",
                            ClusterInit::KMeanPlusPlus => "k mean plus plus",
                        })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(cluster_init, ClusterInit::Llyod, "llyod");
                            ui.selectable_value(cluster_init, ClusterInit::KMeanPlusPlus, "k mean plus plus");
                        });

                    let mut cluster_count_inner = cluster_count.get();
                    ui.add(egui::Slider::new(&mut cluster_count_inner, 1..=128).text("Cluster count"));
                    *cluster_count = NonZero::new(cluster_count_inner).unwrap();
                },
                PosterizeMethod::GaussianMixture { cluster_init, cluster_count } => {
                    egui::ComboBox::from_label("Cluster init")
                        .selected_text(match cluster_init {
                            ClusterInit::Llyod => "llyod",
                            ClusterInit::KMeanPlusPlus => "k mean plus plus",
                        })
                        .show_ui(ui, |ui| {
                            ui.selectable_value(cluster_init, ClusterInit::Llyod, "llyod");
                            ui.selectable_value(cluster_init, ClusterInit::KMeanPlusPlus, "k mean plus plus");
                        });

                    let mut cluster_count_inner = cluster_count.get();
                    ui.add(egui::Slider::new(&mut cluster_count_inner, 1..=128).text("Cluster count"));
                    *cluster_count = NonZero::new(cluster_count_inner).unwrap();
                },
            };
        });
    }

    fn ui_images(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.split_vertical(0.5, |top_ui, bottom_ui| {
                    self.input_texture.as_ref().inspect(|image| { top_ui.adaptive_texture(&image); });
                    self.output_texture.as_ref().inspect(|image| { bottom_ui.adaptive_texture(&image); });
                });
            });
        });
    }

    fn ui(&mut self, ctx: &egui::Context) {
        self.ui_controls(ctx);
        self.ui_images(ctx);
    }
}

fn main() -> eframe::Result {
    let mut app = MyEguiApp::new();
    let native_options = eframe::NativeOptions::default();
    eframe::run_simple_native("Posterize", native_options, move |context, _frame| {
        app.ui(context);
    })
}

