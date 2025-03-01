use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct ThreeApp {}

impl ThreeApp {
    pub fn name() -> &'static str {
        "three"
    }
}

impl eframe::App for ThreeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.5);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("This is a ui.heading. ");

            ui.label("This is a ui.label");

            // This literally creates the button AND checks to see if it was clicked
            if ui.button("Quit").clicked() {
                std::process::exit(0);
            };
        });
    }
}
