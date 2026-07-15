mod calculator;

use eframe::egui;

use egui::{Color32, Vec2, FontId, Style};
use egui::Widget;
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
        
        if total <= 0.0 {
            return;
        }
        
        if total > 999999.99 {
            self.input_amount = "999999.99".to_string();
            self.total = 999999.99;
            return;
        }
        
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
            .with_inner_size(egui::vec2(450.0, 650.0))
            .with_icon(icon),
        ..Default::default()
    };
    let _ = eframe::run_native(
        "budget splitter",
        native_options,
        Box::new(|cc| {
            let mut style = Style::default();
            style.visuals.dark_mode = true;
            style.visuals.widgets.noninteractive.bg_fill = Color32::from_rgb(20, 22, 28);
            style.visuals.widgets.inactive.bg_fill = Color32::from_rgb(30, 33, 42);
            style.visuals.widgets.active.bg_fill = Color32::from_rgb(45, 50, 62);
            style.visuals.widgets.hovered.bg_fill = Color32::from_rgb(40, 44, 55);
            style.visuals.widgets.noninteractive.fg_stroke.color = Color32::from_rgb(200, 200, 210);
            style.visuals.widgets.inactive.fg_stroke.color = Color32::from_rgb(200, 200, 210);
            style.visuals.override_text_color = Some(Color32::from_rgb(230, 230, 240));
            cc.egui_ctx.set_style_of(egui::Theme::Dark, style);
            Ok(Box::new(MyEguiApp::default()))
        }),
    );
}

impl eframe::App for MyEguiApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // top padding
        ui.add_space(30.0);

        // title
        ui.vertical_centered(|ui| {
            ui.label(
                egui::RichText::new("budget splitter")
                    .color(Color32::from_rgb(140, 170, 255))
                    .size(28.0)
                    .strong(),
            );
            ui.add_space(4.0);
            ui.label(
                egui::RichText::new("monthly salary split")
                    .color(Color32::from_rgb(120, 130, 150))
                    .size(13.0),
            );
        });

        ui.add_space(25.0);

        egui::Frame::new()
            .fill(Color32::from_rgb(28, 31, 40))
            .inner_margin(egui::Margin::symmetric(20, 16))
            .show(ui, |ui| {
                ui.label(
                    egui::RichText::new("enter your total income")
                        .color(Color32::from_rgb(180, 190, 210))
                        .size(12.0),
                );
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("mad")
                            .color(Color32::from_rgb(100, 180, 255))
                            .size(18.0)
                            .strong(),
                    );
                
                    ui.add_space(8.0);
                
                    let response = egui::TextEdit::singleline(&mut self.input_amount)
                        .font(FontId::proportional(13.0))
                        .text_color(Color32::WHITE)
                        .desired_width(f32::INFINITY)
                        .margin(egui::Margin::symmetric(10, 8))
                        .ui(ui);
                
                    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        self.calculate();
                    }
                });
            });

        ui.add_space(16.0);

        // calculate button
        ui.vertical_centered(|ui| {
            let btn = egui::Button::new(
                egui::RichText::new("calculate")
                    .color(Color32::WHITE)
                    .size(16.0)
                    .strong(),
            )
            .fill(Color32::from_rgb(60, 90, 200))
            .min_size(Vec2::new(200.0, 44.0));

            if ui.add(btn).clicked() {
                self.calculate();
            }
        });

        ui.add_space(12.0);
        ui.separator();
        ui.add_space(12.0);

        // breakdown table
        if self.total > 0.0 {
            let result = self.facade.calculate(self.total);

            // header row
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("category")
                        .color(Color32::from_rgb(140, 150, 170))
                        .size(13.0)
                        .strong(),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(
                        egui::RichText::new("amount")
                            .color(Color32::from_rgb(140, 150, 170))
                            .size(13.0)
                            .strong(),
                    );
                    ui.add_space(20.0);
                    ui.label(
                        egui::RichText::new("%")
                            .color(Color32::from_rgb(140, 150, 170))
                            .size(13.0)
                            .strong(),
                    );
                    ui.add_space(10.0);
                });
            });

            ui.add_space(2.0);
            ui.separator();

            for line in &result.lines {
                let color = Self::category_color(&line.category);

                egui::Frame::new()
                    .fill(Color32::from_rgb(24, 27, 35))
                    .inner_margin(egui::Margin::symmetric(12, 8))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            // color dot
                            let (rect, _) = ui.allocate_exact_size(
                                Vec2::new(10.0, 10.0),
                                egui::Sense::hover(),
                            );
                            ui.painter().rect_filled(rect, 3.0, color);

                            ui.add_space(8.0);

                            ui.label(
                                egui::RichText::new(line.category.label())
                                    .color(color)
                                    .size(14.0)
                                    .strong(),
                            );

                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(
                                    egui::RichText::new(format!("{:.2} mad", line.amount))
                                        .color(Color32::WHITE)
                                        .size(14.0),
                                );
                                ui.add_space(20.0);
                                ui.label(
                                    egui::RichText::new(format!("{:.0}%", line.percentage))
                                        .color(Color32::from_rgb(150, 160, 180))
                                        .size(13.0),
                                );
                                ui.add_space(10.0);
                            });
                        });
                    });

                ui.add_space(3.0);
            }

            ui.separator();

            // total row
            egui::Frame::new()
                .fill(Color32::from_rgb(35, 45, 80))
                .inner_margin(egui::Margin::symmetric(12, 10))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new("total")
                                .color(Color32::WHITE)
                                .size(15.0)
                                .strong(),
                        );

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(
                                egui::RichText::new(format!("{:.2} mad", result.total_amount()))
                                    .color(Color32::WHITE)
                                    .size(15.0)
                                    .strong(),
                            );
                            ui.add_space(20.0);
                            ui.label(
                                egui::RichText::new(format!("{:.0}%", result.total_percentage()))
                                    .color(Color32::from_rgb(180, 200, 255))
                                    .size(14.0)
                                    .strong(),
                            );
                            ui.add_space(10.0);
                        });
                    });
                });
        }

        ui.add_space(ui.available_height() - 55.0);
        egui::Frame::new()
            .fill(Color32::TRANSPARENT) // invisible frame
            .show(ui, |ui| {
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 2.0;
                        ui.label(
                            egui::RichText::new("made by ")
                                .color(Color32::from_rgb(80, 90, 110))
                                .size(10.0),
                        );
                        ui.hyperlink_to(
                            egui::RichText::new("ahtalbi")
                                .color(Color32::from_rgb(100, 140, 220))
                                .size(10.0),
                            "https://github.com/ahtalbi",
                        );
                    });

                    ui.add_space(2.0);

                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 2.0;
                        ui.label(
                            egui::RichText::new("free for use ")
                                .color(Color32::from_rgb(60, 70, 90))
                                .size(10.0),
                        );
                        ui.hyperlink_to(
                            egui::RichText::new("project")
                                .color(Color32::from_rgb(100, 140, 220))
                                .size(10.0),
                            "https://github.com/ahtalbi/bank-savings-calculator",
                        );
                    });
                });
            });
    }
}