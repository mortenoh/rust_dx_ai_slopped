use crate::cli::commands::egui::{EguiArgs, EguiCommand};
use anyhow::Result;
use eframe::egui;

pub fn run(args: EguiArgs) -> Result<()> {
    match args.command {
        EguiCommand::Demo => cmd_demo(),
        EguiCommand::Counter => cmd_counter(),
        EguiCommand::Clock => cmd_clock(),
    }
}

fn cmd_demo() -> Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 300.0]),
        ..Default::default()
    };

    eframe::run_native(
        "dx egui demo",
        options,
        Box::new(|_cc| Ok(Box::new(DemoApp::default()))),
    )
    .map_err(|e| anyhow::anyhow!("Failed to run egui: {}", e))
}

#[derive(Default)]
struct DemoApp {
    name: String,
}

impl eframe::App for DemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello egui!");
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Your name:");
                ui.text_edit_singleline(&mut self.name);
            });
            if !self.name.is_empty() {
                ui.label(format!("Hello, {}!", self.name));
            }
            ui.separator();
            ui.label("This is a basic egui demo from dx.");
            ui.hyperlink_to("egui documentation", "https://docs.rs/egui");
        });
    }
}

fn cmd_counter() -> Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([300.0, 200.0]),
        ..Default::default()
    };

    eframe::run_native(
        "dx egui counter",
        options,
        Box::new(|_cc| Ok(Box::new(CounterApp::default()))),
    )
    .map_err(|e| anyhow::anyhow!("Failed to run egui: {}", e))
}

#[derive(Default)]
struct CounterApp {
    counter: i32,
}

impl eframe::App for CounterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Counter");
            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("-").clicked() {
                    self.counter -= 1;
                }
                ui.label(format!("{}", self.counter));
                if ui.button("+").clicked() {
                    self.counter += 1;
                }
            });

            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("-10").clicked() {
                    self.counter -= 10;
                }
                if ui.button("Reset").clicked() {
                    self.counter = 0;
                }
                if ui.button("+10").clicked() {
                    self.counter += 10;
                }
            });
        });
    }
}

fn cmd_clock() -> Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([300.0, 150.0]),
        ..Default::default()
    };

    eframe::run_native(
        "dx egui clock",
        options,
        Box::new(|_cc| Ok(Box::new(ClockApp))),
    )
    .map_err(|e| anyhow::anyhow!("Failed to run egui: {}", e))
}

struct ClockApp;

impl eframe::App for ClockApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Clock");
            ui.separator();

            let now = chrono::Local::now();
            ui.label(
                egui::RichText::new(now.format("%H:%M:%S").to_string())
                    .size(48.0)
                    .monospace(),
            );
            ui.label(now.format("%A, %B %d, %Y").to_string());
        });

        // Request repaint to update the clock
        ctx.request_repaint();
    }
}
