use eframe::egui;

fn main() {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(egui::vec2(300.0, 400.0)),
        ..Default::default()
    };
    let _ = eframe::run_native("Bank Savings Calculator", native_options, Box::new(|_cc| Ok(Box::new(MyEguiApp::default()))));
}

#[derive(Default)]
struct MyEguiApp {
    input_amount: String,
    output: String,
}

impl MyEguiApp {
    fn calculate(&mut self) {
        self.output = self.input_amount.clone();
    }
}

impl eframe::App for MyEguiApp {
   fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
       egui::CentralPanel::default().show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Bank Savings Calculator");
            });

            ui.add_space(20.0);

            ui.horizontal(|ui| {
                ui.label("Amount: $");
                let response = ui.text_edit_singleline(&mut self.input_amount);
                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    self.calculate();
                }
            });

            ui.add_space(10.0);

            if ui.button("Calculate").clicked() {
                self.calculate();
            }

            ui.add_space(20.0);

            ui.separator();

            ui.add_space(10.0);

            ui.label(&self.output);
        });
   }
}