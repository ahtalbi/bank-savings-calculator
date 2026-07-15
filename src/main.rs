mod calculator;

use eframe::egui;
use egui::Color32;
use calculator::facade::BudgetFacade;
use calculator::types::BudgetCategory;

struct MyEguiApp {
    facade: BudgetFacade,
    input_amount: String,
    total: f64,
}

impl Default for MyEguiApp {
    fn default() -> Self {
        Self {
            facade: BudgetFacade::new(),
            input_amount: String::new(),
            total: 0.0,
        }
    }
}

impl MyEguiApp {
    fn calculate(&mut self) {
        let total: f64 = match self.input_amount.parse() {
            Ok(v) => v,
            Err(_) => return,
        };
        self.total = total;
    }

    fn category_color(cat: &BudgetCategory) -> Color32 {
        match cat {
            BudgetCategory::Rent => Color32::from_rgb(255, 99, 71),
            BudgetCategory::Taxes => Color32::from_rgb(100, 149, 237),
            BudgetCategory::Living => Color32::from_rgb(60, 179, 113),
            BudgetCategory::LongTermInvestment => Color32::from_rgb(255, 215, 0),
            BudgetCategory::ShortTermInvestment => Color32::from_rgb(147, 112, 219),
        }
    }
}

fn load_icon() -> egui::IconData {
    let path = std::path::Path::new("src/logo.png");
    if !path.exists() {
        return egui::IconData::default();
    }
    match image::open(path) {
        Ok(img) => {
            let img = img.to_rgba8();
            let (w, h) = img.dimensions();
            egui::IconData {
                rgba: img.into_raw(),
                width: w,
                height: h,
            }
        }
        Err(_) => egui::IconData::default(),
    }
}

fn main() {
    let icon = load_icon();
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(egui::vec2(450.0, 600.0))
            .with_icon(icon),
        ..Default::default()
    };
    let _ = eframe::run_native(
        "budget splitter",
        native_options,
        Box::new(|_cc| Ok(Box::new(MyEguiApp::default()))),
    );
}

impl eframe::App for MyEguiApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.vertical_centered(|ui| {
            ui.heading(
                egui::RichText::new("budget splitter")
                    .color(Color32::from_rgb(50, 50, 150))
                    .size(24.0),
            );
        });

        ui.add_space(10.0);
        ui.separator();
        ui.add_space(10.0);

        // input section
        ui.label(
            egui::RichText::new("enter your amount in mad:")
                .size(14.0)
                .strong(),
        );

        ui.add_space(5.0);

        ui.horizontal(|ui| {
            ui.label("mad");
            let response = ui.text_edit_singleline(&mut self.input_amount);
            if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                self.calculate();
            }
        });

        ui.add_space(10.0);

        if ui.button("calculate").clicked() {
            self.calculate();
        }

        ui.add_space(15.0);
        ui.separator();
        ui.add_space(10.0);

        // breakdown table
        if self.total > 0.0 {
            let result = self.facade.calculate(self.total);

            // header row
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("category")
                        .color(Color32::DARK_GRAY)
                        .strong()
                        .size(14.0),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        egui::RichText::new("amount")
                            .color(Color32::DARK_GRAY)
                            .strong()
                            .size(14.0),
                    );
                    ui.add_space(15.0);
                    ui.label(
                        egui::RichText::new("%")
                            .color(Color32::DARK_GRAY)
                            .strong()
                            .size(14.0),
                    );
                    ui.add_space(10.0);
                });
            });

            ui.separator();

            for line in &result.lines {
                let color = Self::category_color(&line.category);

                ui.horizontal(|ui| {
                    let (rect, _) = ui.allocate_exact_size(
                        egui::vec2(10.0, 10.0),
                        egui::Sense::hover(),
                    );
                    let painter = ui.painter();
                    painter.rect_filled(rect, 2.0, color);

                    ui.add_space(5.0);
                    ui.label(
                        egui::RichText::new(line.category.label())
                            .color(color)
                            .strong()
                            .size(14.0),
                    );

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(
                            egui::RichText::new(format!("{:.2} mad", line.amount))
                                .size(14.0),
                        );
                        ui.add_space(15.0);
                        ui.label(
                            egui::RichText::new(format!("{:.0}%", line.percentage))
                                .color(Color32::GRAY)
                                .size(14.0),
                        );
                        ui.add_space(10.0);
                    });
                });

                ui.add_space(2.0);
            }

            ui.separator();

            // total row
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("total")
                        .strong()
                        .size(14.0),
                );

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        egui::RichText::new(format!("{:.2} mad", result.total_amount()))
                            .strong()
                            .size(14.0),
                    );
                    ui.add_space(15.0);
                    ui.label(
                        egui::RichText::new(format!("{:.0}%", result.total_percentage()))
                            .strong()
                            .size(14.0),
                    );
                    ui.add_space(10.0);
                });
            });
        }
    }
}