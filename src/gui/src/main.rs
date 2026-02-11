use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Dragon Dance",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

#[derive(Default)]
struct MyApp {
    label: String,
    value: f32,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Center content both horizontally and vertically
            ui.with_layout(
                egui::Layout::centered_and_justified(egui::Direction::TopDown),
                |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("Dragon Dance Home");

                        ui.add_space(20.0);

                        ui.horizontal(|ui| {
                            if ui.button("Prices").clicked() {
                                self.value += 1.0;
                            }

                            ui.add_space(10.0);

                            if ui.button("Options Chains").clicked() {
                                self.value += 1.0;
                            }
                        });
                    });
                },
            );
        });
    }
}
