use eframe::egui;

/// Extension trait on egui::Ui
pub trait UiExt {
    /// Split available space vertically.
    ///
    /// Unlike [egui::Ui::vertical] which allocate available space on a first come first serve basis,
    /// the splitting obtained always according to the provided fraction.
    fn split_vertical(&mut self, fraction: f32, add_contents: impl FnOnce(&mut egui::Ui, &mut egui::Ui));

    /// Split available space horizontally.
    ///
    /// Unlike [egui::Ui::horizontal] which allocate available space on a first come first serve basis,
    /// the splitting obtained always according to the provided fraction.
    fn split_horizontal(&mut self, fraction: f32, add_contents: impl FnOnce(&mut egui::Ui, &mut egui::Ui));

    /// Render a texture.
    ///
    /// The texture is scaled according to available space while keeping aspect ratio. The texture
    /// is also centered in available space.
    fn adaptive_texture(&mut self, texture: &egui::TextureHandle) -> egui::Response;
}

impl UiExt for egui::Ui {
    fn split_vertical(&mut self, fraction: f32, add_contents: impl FnOnce(&mut egui::Ui, &mut egui::Ui)) {
        let ui_rect = self.max_rect();
        let (top_rect, bottom_rect) = ui_rect.split_top_bottom_at_fraction(fraction);

        let mut top_ui = self.child_ui_with_id_source(top_rect, egui::Layout::default(), "top", None);
        let mut bottom_ui = self.child_ui_with_id_source(bottom_rect, egui::Layout::default(), "bottom", None);
        add_contents(&mut top_ui, &mut bottom_ui);
    }

    fn split_horizontal(&mut self, fraction: f32, add_contents: impl FnOnce(&mut egui::Ui, &mut egui::Ui)) {
        let ui_rect = self.max_rect();
        let (top_rect, bottom_rect) = ui_rect.split_left_right_at_fraction(fraction);

        let mut top_ui = self.child_ui_with_id_source(top_rect, egui::Layout::default(), "top", None);
        let mut bottom_ui = self.child_ui_with_id_source(bottom_rect, egui::Layout::default(), "bottom", None);
        add_contents(&mut top_ui, &mut bottom_ui);
    }

    fn adaptive_texture(&mut self, texture: &egui::TextureHandle) -> egui::Response {
        let mut size = texture.size_vec2();

        let horizontal_ratio = self.available_width() / size.x;
        let vertical_ratio = self.available_height() / size.y;
        size *= f32::min(horizontal_ratio, vertical_ratio);

        self.centered_and_justified(|ui| ui.image((texture.id(), size))).inner
    }
}


