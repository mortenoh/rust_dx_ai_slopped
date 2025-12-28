use crate::cli::commands::egui::{EguiArgs, EguiCommand};
use anyhow::Result;
use eframe::egui::{self, Color32, RichText};

// Layout constants for consistent styling
const MARGIN: f32 = 16.0;
const SECTION_SPACING: f32 = 16.0;
const ITEM_SPACING: f32 = 8.0;
const GRID_SPACING: [f32; 2] = [12.0, 8.0];
const SCROLL_HEIGHT_SMALL: f32 = 120.0;
const SCROLL_HEIGHT_MEDIUM: f32 = 180.0;
#[allow(dead_code)]
const SCROLL_HEIGHT_LARGE: f32 = 250.0;

pub fn run(args: EguiArgs) -> Result<()> {
    match args.command {
        // Existing
        EguiCommand::Demo => cmd_demo(),
        EguiCommand::Counter => cmd_counter(),
        EguiCommand::Clock => cmd_clock(),
        EguiCommand::Work => cmd_work(),
        // Generators
        EguiCommand::Uuid => cmd_uuid(),
        EguiCommand::Password => cmd_password(),
        EguiCommand::Qrcode => cmd_qrcode(),
        EguiCommand::Lorem => cmd_lorem(),
        EguiCommand::Color => cmd_color(),
        // Encoders/Decoders
        EguiCommand::Hash => cmd_hash(),
        EguiCommand::Base64 => cmd_base64(),
        EguiCommand::Hex => cmd_hex(),
        EguiCommand::Url => cmd_url(),
        // Converters
        EguiCommand::Timestamp => cmd_timestamp(),
        EguiCommand::Units => cmd_units(),
        EguiCommand::Base => cmd_base(),
        EguiCommand::Json => cmd_json(),
        // Utilities
        EguiCommand::Regex => cmd_regex(),
        EguiCommand::Diff => cmd_diff(),
        EguiCommand::Stopwatch => cmd_stopwatch(),
        EguiCommand::Calculator => cmd_calculator(),
        // Text Tools
        EguiCommand::Case => cmd_case(),
        EguiCommand::TextStats => cmd_text_stats(),
        EguiCommand::Markdown => cmd_markdown(),
        EguiCommand::Timer => cmd_timer(),
    }
}

// ============================================================================
// EXISTING TOOLS
// ============================================================================

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
            egui::Frame::none().inner_margin(MARGIN).show(ui, |ui| {
                ui.heading("Hello egui!");
                ui.add_space(SECTION_SPACING);

                egui::Frame::group(ui.style())
                    .inner_margin(ITEM_SPACING)
                    .show(ui, |ui| {
                        egui::Grid::new("demo_grid")
                            .num_columns(2)
                            .spacing(GRID_SPACING)
                            .show(ui, |ui| {
                                ui.label("Your name:");
                                ui.text_edit_singleline(&mut self.name);
                                ui.end_row();
                            });
                    });

                if !self.name.is_empty() {
                    ui.add_space(ITEM_SPACING);
                    ui.label(
                        RichText::new(format!("Hello, {}!", self.name))
                            .size(18.0)
                            .color(Color32::GREEN),
                    );
                }

                ui.add_space(SECTION_SPACING);
                ui.label("This is a basic egui demo from dx.");
                ui.hyperlink_to("egui documentation", "https://docs.rs/egui");
            });
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
            egui::Frame::none().inner_margin(MARGIN).show(ui, |ui| {
                ui.heading("Counter");
                ui.add_space(SECTION_SPACING);

                egui::Frame::group(ui.style())
                    .inner_margin(ITEM_SPACING)
                    .show(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.label(
                                RichText::new(format!("{}", self.counter))
                                    .size(48.0)
                                    .monospace(),
                            );
                        });
                    });

                ui.add_space(SECTION_SPACING);

                ui.horizontal(|ui| {
                    if ui.button("-").clicked() {
                        self.counter -= 1;
                    }
                    if ui.button("+").clicked() {
                        self.counter += 1;
                    }
                });

                ui.add_space(ITEM_SPACING);

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
            egui::Frame::none().inner_margin(MARGIN).show(ui, |ui| {
                ui.heading("Clock");
                ui.add_space(SECTION_SPACING);

                egui::Frame::group(ui.style())
                    .inner_margin(ITEM_SPACING)
                    .show(ui, |ui| {
                        let now = chrono::Local::now();
                        ui.vertical_centered(|ui| {
                            ui.label(
                                RichText::new(now.format("%H:%M:%S").to_string())
                                    .size(48.0)
                                    .monospace(),
                            );
                            ui.add_space(ITEM_SPACING);
                            ui.label(now.format("%A, %B %d, %Y").to_string());
                        });
                    });
            });
        });

        ctx.request_repaint();
    }
}

fn cmd_work() -> Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([450.0, 350.0]),
        ..Default::default()
    };

    eframe::run_native(
        "dx egui work",
        options,
        Box::new(|_cc| Ok(Box::new(WorkApp::default()))),
    )
    .map_err(|e| anyhow::anyhow!("Failed to run egui: {}", e))
}

struct WorkApp {
    x: f32,
    y: f32,
    processing: bool,
    progress: f32,
    start_time: Option<std::time::Instant>,
    duration_secs: f32,
    result: Option<f32>,
}

impl Default for WorkApp {
    fn default() -> Self {
        Self {
            x: 10.0,
            y: 5.0,
            processing: false,
            progress: 0.0,
            start_time: None,
            duration_secs: 2.0,
            result: None,
        }
    }
}

impl eframe::App for WorkApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update progress if processing
        if self.processing {
            if let Some(start) = self.start_time {
                let elapsed = start.elapsed().as_secs_f32();
                self.progress = (elapsed / self.duration_secs).min(1.0);

                if self.progress >= 1.0 {
                    self.processing = false;
                    self.result = Some(self.x * self.y);
                    self.start_time = None;
                }
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Frame::none().inner_margin(MARGIN).show(ui, |ui| {
                ui.heading("Work Simulator");
                ui.add_space(SECTION_SPACING);

                // Input section
                egui::Frame::group(ui.style())
                    .inner_margin(ITEM_SPACING)
                    .show(ui, |ui| {
                        egui::Grid::new("work_inputs")
                            .num_columns(2)
                            .spacing(GRID_SPACING)
                            .show(ui, |ui| {
                                ui.label("X value:");
                                ui.add_enabled(
                                    !self.processing,
                                    egui::DragValue::new(&mut self.x).speed(0.1),
                                );
                                ui.end_row();

                                ui.label("Y value:");
                                ui.add_enabled(
                                    !self.processing,
                                    egui::DragValue::new(&mut self.y).speed(0.1),
                                );
                                ui.end_row();

                                ui.label("Duration (s):");
                                ui.add_enabled(
                                    !self.processing,
                                    egui::DragValue::new(&mut self.duration_secs)
                                        .speed(0.1)
                                        .range(0.5..=10.0),
                                );
                                ui.end_row();
                            });
                    });

                ui.add_space(SECTION_SPACING);

                // Process button
                ui.horizontal(|ui| {
                    let button = ui.add_enabled(!self.processing, egui::Button::new("Process"));
                    if button.clicked() {
                        self.processing = true;
                        self.progress = 0.0;
                        self.start_time = Some(std::time::Instant::now());
                        self.result = None;
                    }

                    if self.processing {
                        ui.spinner();
                        ui.label("Processing...");
                    }
                });

                ui.add_space(SECTION_SPACING);

                // Progress section
                egui::Frame::group(ui.style())
                    .inner_margin(ITEM_SPACING)
                    .show(ui, |ui| {
                        ui.label("Progress:");
                        let progress_bar = egui::ProgressBar::new(self.progress)
                            .text(format!("{:.0}%", self.progress * 100.0))
                            .animate(self.processing);
                        ui.add(progress_bar);

                        ui.add_space(ITEM_SPACING);

                        // Status
                        let status = if self.processing {
                            RichText::new(format!("Computing {} × {}...", self.x, self.y))
                                .color(Color32::YELLOW)
                        } else if let Some(result) = self.result {
                            RichText::new(format!("Result: {} × {} = {}", self.x, self.y, result))
                                .color(Color32::GREEN)
                                .strong()
                        } else {
                            RichText::new("Ready to process").color(Color32::GRAY)
                        };
                        ui.label(status);
                    });
            });
        });

        // Request repaint while processing for smooth animation
        if self.processing {
            ctx.request_repaint();
        }
    }
}

// ============================================================================
// GENERATORS
// ============================================================================

fn cmd_uuid() -> Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([450.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native(
        "dx egui uuid",
        options,
        Box::new(|_cc| Ok(Box::new(UuidApp::default()))),
    )
    .map_err(|e| anyhow::anyhow!("Failed to run: {}", e))
}

#[derive(PartialEq, Clone, Copy)]
enum UuidVersion {
    V4,
    V7,
}

#[derive(PartialEq, Clone, Copy)]
enum UuidFormat {
    Standard,
    Simple,
    Urn,
    Braced,
}

struct UuidApp {
    version: UuidVersion,
    format: UuidFormat,
    count: usize,
    uuids: Vec<String>,
}

impl Default for UuidApp {
    fn default() -> Self {
        Self {
            version: UuidVersion::V4,
            format: UuidFormat::Standard,
            count: 5,
            uuids: Vec::new(),
        }
    }
}

impl UuidApp {
    fn generate(&mut self) {
        self.uuids.clear();
        for _ in 0..self.count {
            let id = match self.version {
                UuidVersion::V4 => uuid::Uuid::new_v4(),
                UuidVersion::V7 => uuid::Uuid::now_v7(),
            };
            let s = match self.format {
                UuidFormat::Standard => id.to_string(),
                UuidFormat::Simple => id.simple().to_string(),
                UuidFormat::Urn => id.urn().to_string(),
                UuidFormat::Braced => id.braced().to_string(),
            };
            self.uuids.push(s);
        }
    }
}

impl eframe::App for UuidApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Frame::none().inner_margin(MARGIN).show(ui, |ui| {
                ui.heading("UUID Generator");
                ui.add_space(SECTION_SPACING);

                egui::Frame::group(ui.style())
                    .inner_margin(ITEM_SPACING)
                    .show(ui, |ui| {
                        egui::Grid::new("uuid_controls")
                            .num_columns(2)
                            .spacing(GRID_SPACING)
                            .show(ui, |ui| {
                                ui.label("Version:");
                                ui.horizontal(|ui| {
                                    ui.radio_value(
                                        &mut self.version,
                                        UuidVersion::V4,
                                        "v4 (random)",
                                    );
                                    ui.radio_value(
                                        &mut self.version,
                                        UuidVersion::V7,
                                        "v7 (time-based)",
                                    );
                                });
                                ui.end_row();

                                ui.label("Format:");
                                egui::ComboBox::from_id_salt("uuid_format")
                                    .selected_text(match self.format {
                                        UuidFormat::Standard => "standard",
                                        UuidFormat::Simple => "simple",
                                        UuidFormat::Urn => "urn",
                                        UuidFormat::Braced => "braced",
                                    })
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            &mut self.format,
                                            UuidFormat::Standard,
                                            "standard",
                                        );
                                        ui.selectable_value(
                                            &mut self.format,
                                            UuidFormat::Simple,
                                            "simple",
                                        );
                                        ui.selectable_value(
                                            &mut self.format,
                                            UuidFormat::Urn,
                                            "urn",
                                        );
                                        ui.selectable_value(
                                            &mut self.format,
                                            UuidFormat::Braced,
                                            "braced",
                                        );
                                    });
                                ui.end_row();

                                ui.label("Count:");
                                ui.add(egui::Slider::new(&mut self.count, 1..=50));
                                ui.end_row();
                            });
                    });

                ui.add_space(SECTION_SPACING);

                if ui.button("Generate").clicked() {
                    self.generate();
                }

                ui.add_space(SECTION_SPACING);

                egui::Frame::group(ui.style())
                    .inner_margin(ITEM_SPACING)
                    .show(ui, |ui| {
                        egui::ScrollArea::vertical()
                            .max_height(SCROLL_HEIGHT_MEDIUM)
                            .show(ui, |ui| {
                                egui::Grid::new("uuid_grid")
                                    .num_columns(2)
                                    .spacing(GRID_SPACING)
                                    .show(ui, |ui| {
                                        for (i, uuid) in self.uuids.iter().enumerate() {
                                            ui.label(RichText::new(uuid).monospace());
                                            if ui.small_button("Copy").clicked() {
                                                ui.output_mut(|o| o.copied_text = uuid.clone());
                                            }
                                            if i < self.uuids.len() - 1 {
                                                ui.end_row();
                                            }
                                        }
                                    });
                            });
                    });
            });
        });
    }
}

fn cmd_password() -> Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([450.0, 450.0]),
        ..Default::default()
    };

    eframe::run_native(
        "dx egui password",
        options,
        Box::new(|_cc| Ok(Box::new(PasswordApp::default()))),
    )
    .map_err(|e| anyhow::anyhow!("Failed to run: {}", e))
}

struct PasswordApp {
    length: usize,
    use_upper: bool,
    use_lower: bool,
    use_digits: bool,
    use_symbols: bool,
    passwords: Vec<String>,
    count: usize,
}

impl Default for PasswordApp {
    fn default() -> Self {
        Self {
            length: 16,
            use_upper: true,
            use_lower: true,
            use_digits: true,
            use_symbols: true,
            passwords: Vec::new(),
            count: 5,
        }
    }
}

impl PasswordApp {
    fn generate(&mut self) {
        use rand::Rng;
        let mut charset = String::new();
        if self.use_upper {
            charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        }
        if self.use_lower {
            charset.push_str("abcdefghijklmnopqrstuvwxyz");
        }
        if self.use_digits {
            charset.push_str("0123456789");
        }
        if self.use_symbols {
            charset.push_str("!@#$%^&*()-_=+[]{}|;:,.<>?");
        }

        if charset.is_empty() {
            return;
        }

        let chars: Vec<char> = charset.chars().collect();
        let mut rng = rand::rng();

        self.passwords.clear();
        for _ in 0..self.count {
            let password: String = (0..self.length)
                .map(|_| chars[rng.random_range(0..chars.len())])
                .collect();
            self.passwords.push(password);
        }
    }

    fn strength(&self) -> (f32, &'static str, Color32) {
        let mut pool = 0usize;
        if self.use_upper {
            pool += 26;
        }
        if self.use_lower {
            pool += 26;
        }
        if self.use_digits {
            pool += 10;
        }
        if self.use_symbols {
            pool += 26;
        }

        if pool == 0 {
            return (0.0, "None", Color32::GRAY);
        }

        let entropy = (self.length as f64) * (pool as f64).log2();
        if entropy < 28.0 {
            (0.2, "Weak", Color32::RED)
        } else if entropy < 36.0 {
            (0.4, "Fair", Color32::from_rgb(255, 165, 0))
        } else if entropy < 60.0 {
            (0.6, "Good", Color32::YELLOW)
        } else if entropy < 80.0 {
            (0.8, "Strong", Color32::from_rgb(144, 238, 144))
        } else {
            (1.0, "Very Strong", Color32::GREEN)
        }
    }
}

impl eframe::App for PasswordApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Frame::none().inner_margin(MARGIN).show(ui, |ui| {
                ui.heading("Password Generator");
                ui.add_space(SECTION_SPACING);

                egui::Frame::group(ui.style())
                    .inner_margin(ITEM_SPACING)
                    .show(ui, |ui| {
                        egui::Grid::new("password_controls")
                            .num_columns(2)
                            .spacing(GRID_SPACING)
                            .show(ui, |ui| {
                                ui.label("Length:");
                                ui.add(egui::Slider::new(&mut self.length, 8..=128));
                                ui.end_row();

                                ui.label("Characters:");
                                ui.horizontal(|ui| {
                                    ui.checkbox(&mut self.use_upper, "A-Z");
                                    ui.checkbox(&mut self.use_lower, "a-z");
                                    ui.checkbox(&mut self.use_digits, "0-9");
                                    ui.checkbox(&mut self.use_symbols, "!@#");
                                });
                                ui.end_row();

                                ui.label("Count:");
                                ui.add(egui::DragValue::new(&mut self.count).range(1..=20));
                                ui.end_row();
                            });

                        ui.add_space(ITEM_SPACING);

                        let (strength, label, color) = self.strength();
                        ui.horizontal(|ui| {
                            ui.label("Strength:");
                            ui.add(egui::ProgressBar::new(strength).text(label).fill(color));
                        });
                    });

                ui.add_space(SECTION_SPACING);

                if ui.button("Generate").clicked() {
                    self.generate();
                }

                ui.add_space(SECTION_SPACING);

                egui::Frame::group(ui.style())
                    .inner_margin(ITEM_SPACING)
                    .show(ui, |ui| {
                        egui::ScrollArea::vertical()
                            .max_height(SCROLL_HEIGHT_SMALL)
                            .show(ui, |ui| {
                                for password in &self.passwords {
                                    ui.horizontal(|ui| {
                                        ui.label(RichText::new(password).monospace());
                                        if ui.small_button("Copy").clicked() {
                                            ui.output_mut(|o| o.copied_text = password.clone());
                                        }
                                    });
                                }
                            });
                    });
            });
        });
    }
}

fn cmd_qrcode() -> Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([450.0, 550.0]),
        ..Default::default()
    };

    eframe::run_native(
        "dx egui qrcode",
        options,
        Box::new(|_cc| Ok(Box::new(QrcodeApp::default()))),
    )
    .map_err(|e| anyhow::anyhow!("Failed to run: {}", e))
}

struct QrcodeApp {
    input: String,
    invert: bool,
    scale: usize,
}

impl Default for QrcodeApp {
    fn default() -> Self {
        Self {
            input: "https://github.com/mortenoh/rust_dx_ai_slopped".to_string(),
            invert: false,
            scale: 4,
        }
    }
}

impl eframe::App for QrcodeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Frame::none().inner_margin(MARGIN).show(ui, |ui| {
                ui.heading("QR Code Generator");
                ui.add_space(SECTION_SPACING);

                egui::Frame::group(ui.style())
                    .inner_margin(ITEM_SPACING)
                    .show(ui, |ui| {
                        ui.label("Text/URL:");
                        ui.add(
                            egui::TextEdit::multiline(&mut self.input)
                                .desired_width(f32::INFINITY)
                                .desired_rows(3),
                        );

                        ui.add_space(ITEM_SPACING);

                        ui.horizontal(|ui| {
                            ui.checkbox(&mut self.invert, "Invert colors");
                            ui.label("Scale:");
                            ui.add(egui::Slider::new(&mut self.scale, 2..=8));
                        });
                    });

                ui.add_space(SECTION_SPACING);

                if !self.input.is_empty() {
                    if let Ok(code) = qrcode::QrCode::new(self.input.as_bytes()) {
                        let colors = code.to_colors();
                        let width = code.width();

                        let (fg, bg) = if self.invert {
                            (Color32::WHITE, Color32::BLACK)
                        } else {
                            (Color32::BLACK, Color32::WHITE)
                        };

                        let size =
                            egui::vec2((width * self.scale) as f32, (width * self.scale) as f32);

                        egui::Frame::none()
                            .fill(bg)
                            .inner_margin(ITEM_SPACING)
                            .show(ui, |ui| {
                                let (rect, _response) =
                                    ui.allocate_exact_size(size, egui::Sense::hover());
                                let painter = ui.painter_at(rect);

                                for (i, color) in colors.iter().enumerate() {
                                    let x = i % width;
                                    let y = i / width;
                                    let c = match color {
                                        qrcode::Color::Dark => fg,
                                        qrcode::Color::Light => bg,
                                    };
                                    let pos = rect.min
                                        + egui::vec2(
                                            (x * self.scale) as f32,
                                            (y * self.scale) as f32,
                                        );
                                    painter.rect_filled(
                                        egui::Rect::from_min_size(
                                            pos,
                                            egui::vec2(self.scale as f32, self.scale as f32),
                                        ),
                                        0.0,
                                        c,
                                    );
                                }
                            });
                    }
                }
            });
        });
    }
}

fn cmd_lorem() -> Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([500.0, 450.0]),
        ..Default::default()
    };

    eframe::run_native(
        "dx egui lorem",
        options,
        Box::new(|_cc| Ok(Box::new(LoremApp::default()))),
    )
    .map_err(|e| anyhow::anyhow!("Failed to run: {}", e))
}

#[derive(PartialEq, Clone, Copy)]
enum LoremUnit {
    Words,
    Sentences,
    Paragraphs,
}

struct LoremApp {
    unit: LoremUnit,
    count: usize,
    text: String,
}

impl Default for LoremApp {
    fn default() -> Self {
        Self {
            unit: LoremUnit::Paragraphs,
            count: 3,
            text: String::new(),
        }
    }
}

impl LoremApp {
    fn generate(&mut self) {
        self.text = match self.unit {
            LoremUnit::Words => lipsum::lipsum(self.count),
            LoremUnit::Sentences => {
                (0..self.count)
                    .map(|_| lipsum::lipsum(10))
                    .collect::<Vec<_>>()
                    .join(". ")
                    + "."
            }
            LoremUnit::Paragraphs => (0..self.count)
                .map(|_| lipsum::lipsum(50))
                .collect::<Vec<_>>()
                .join("\n\n"),
        };
    }
}

impl eframe::App for LoremApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Frame::none().inner_margin(MARGIN).show(ui, |ui| {
                ui.heading("Lorem Ipsum Generator");
                ui.add_space(SECTION_SPACING);

                egui::Frame::group(ui.style())
                    .inner_margin(ITEM_SPACING)
                    .show(ui, |ui| {
                        egui::Grid::new("lorem_controls")
                            .num_columns(2)
                            .spacing(GRID_SPACING)
                            .show(ui, |ui| {
                                ui.label("Unit:");
                                ui.horizontal(|ui| {
                                    ui.radio_value(&mut self.unit, LoremUnit::Words, "Words");
                                    ui.radio_value(
                                        &mut self.unit,
                                        LoremUnit::Sentences,
                                        "Sentences",
                                    );
                                    ui.radio_value(
                                        &mut self.unit,
                                        LoremUnit::Paragraphs,
                                        "Paragraphs",
                                    );
                                });
                                ui.end_row();

                                ui.label("Count:");
                                ui.add(egui::DragValue::new(&mut self.count).range(1..=100));
                                ui.end_row();
                            });
                    });

                ui.add_space(SECTION_SPACING);

                ui.horizontal(|ui| {
                    if ui.button("Generate").clicked() {
                        self.generate();
                    }
                    if ui.button("Copy").clicked() {
                        ui.output_mut(|o| o.copied_text = self.text.clone());
                    }
                });

                ui.add_space(SECTION_SPACING);

                egui::Frame::group(ui.style())
                    .inner_margin(ITEM_SPACING)
                    .show(ui, |ui| {
                        egui::ScrollArea::vertical()
                            .max_height(SCROLL_HEIGHT_MEDIUM)
                            .show(ui, |ui| {
                                ui.add(
                                    egui::TextEdit::multiline(&mut self.text.as_str())
                                        .desired_width(f32::INFINITY),
                                );
                            });
                    });
            });
        });
    }
}

fn cmd_color() -> Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([450.0, 500.0]),
        ..Default::default()
    };

    eframe::run_native(
        "dx egui color",
        options,
        Box::new(|_cc| Ok(Box::new(ColorApp::default()))),
    )
    .map_err(|e| anyhow::anyhow!("Failed to run: {}", e))
}

struct ColorApp {
    color: [f32; 3],
    hex_input: String,
}

impl Default for ColorApp {
    fn default() -> Self {
        Self {
            color: [0.5, 0.3, 0.8],
            hex_input: String::new(),
        }
    }
}

impl ColorApp {
    fn to_hex(&self) -> String {
        format!(
            "#{:02X}{:02X}{:02X}",
            (self.color[0] * 255.0) as u8,
            (self.color[1] * 255.0) as u8,
            (self.color[2] * 255.0) as u8
        )
    }

    fn to_rgb(&self) -> String {
        format!(
            "rgb({}, {}, {})",
            (self.color[0] * 255.0) as u8,
            (self.color[1] * 255.0) as u8,
            (self.color[2] * 255.0) as u8
        )
    }

    fn to_hsl(&self) -> String {
        let r = self.color[0];
        let g = self.color[1];
        let b = self.color[2];
        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let l = (max + min) / 2.0;

        if (max - min).abs() < f32::EPSILON {
            return format!("hsl(0, 0%, {}%)", (l * 100.0) as i32);
        }

        let d = max - min;
        let s = if l > 0.5 {
            d / (2.0 - max - min)
        } else {
            d / (max + min)
        };

        let h = if (max - r).abs() < f32::EPSILON {
            (g - b) / d + if g < b { 6.0 } else { 0.0 }
        } else if (max - g).abs() < f32::EPSILON {
            (b - r) / d + 2.0
        } else {
            (r - g) / d + 4.0
        };

        format!(
            "hsl({}, {}%, {}%)",
            (h * 60.0) as i32,
            (s * 100.0) as i32,
            (l * 100.0) as i32
        )
    }
}

impl eframe::App for ColorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Frame::none().inner_margin(MARGIN).show(ui, |ui| {
                ui.heading("Color Picker");
                ui.add_space(SECTION_SPACING);

                egui::Frame::group(ui.style())
                    .inner_margin(ITEM_SPACING)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.color_edit_button_rgb(&mut self.color);
                            let color = Color32::from_rgb(
                                (self.color[0] * 255.0) as u8,
                                (self.color[1] * 255.0) as u8,
                                (self.color[2] * 255.0) as u8,
                            );
                            let size = egui::vec2(100.0, 50.0);
                            let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
                            ui.painter().rect_filled(rect, 4.0, color);
                        });
                    });

                ui.add_space(SECTION_SPACING);

                egui::Frame::group(ui.style())
                    .inner_margin(ITEM_SPACING)
                    .show(ui, |ui| {
                        egui::Grid::new("color_grid")
                            .num_columns(3)
                            .spacing(GRID_SPACING)
                            .show(ui, |ui| {
                                let hex = self.to_hex();
                                ui.label("HEX:");
                                ui.label(RichText::new(&hex).monospace());
                                if ui.small_button("Copy").clicked() {
                                    ui.output_mut(|o| o.copied_text = hex);
                                }
                                ui.end_row();

                                let rgb = self.to_rgb();
                                ui.label("RGB:");
                                ui.label(RichText::new(&rgb).monospace());
                                if ui.small_button("Copy").clicked() {
                                    ui.output_mut(|o| o.copied_text = rgb);
                                }
                                ui.end_row();

                                let hsl = self.to_hsl();
                                ui.label("HSL:");
                                ui.label(RichText::new(&hsl).monospace());
                                if ui.small_button("Copy").clicked() {
                                    ui.output_mut(|o| o.copied_text = hsl);
                                }
                                ui.end_row();
                            });
                    });

                ui.add_space(SECTION_SPACING);

                egui::Frame::group(ui.style())
                    .inner_margin(ITEM_SPACING)
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label("Enter HEX:");
                            ui.text_edit_singleline(&mut self.hex_input);
                            if ui.button("Apply").clicked() {
                                let hex = self.hex_input.trim_start_matches('#');
                                if hex.len() == 6 {
                                    if let (Ok(r), Ok(g), Ok(b)) = (
                                        u8::from_str_radix(&hex[0..2], 16),
                                        u8::from_str_radix(&hex[2..4], 16),
                                        u8::from_str_radix(&hex[4..6], 16),
                                    ) {
                                        self.color =
                                            [r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0];
                                    }
                                }
                            }
                        });
                    });
            });
        });
    }
}

// ============================================================================
// ENCODERS/DECODERS
// ============================================================================

fn cmd_hash() -> Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([500.0, 450.0]),
        ..Default::default()
    };

    eframe::run_native(
        "dx egui hash",
        options,
        Box::new(|_cc| Ok(Box::new(HashApp::default()))),
    )
    .map_err(|e| anyhow::anyhow!("Failed to run: {}", e))
}

#[derive(Default)]
struct HashApp {
    input: String,
    compare_mode: bool,
    expected_hash: String,
}

impl HashApp {
    fn md5(&self) -> String {
        use md5::{Digest, Md5};
        let mut hasher = Md5::new();
        hasher.update(self.input.as_bytes());
        hex::encode(hasher.finalize())
    }

    fn sha256(&self) -> String {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(self.input.as_bytes());
        hex::encode(hasher.finalize())
    }

    fn sha512(&self) -> String {
        use sha2::{Digest, Sha512};
        let mut hasher = Sha512::new();
        hasher.update(self.input.as_bytes());
        hex::encode(hasher.finalize())
    }

    fn matches(&self, hash: &str) -> Option<bool> {
        if !self.compare_mode || self.expected_hash.is_empty() {
            return None;
        }
        Some(hash.to_lowercase() == self.expected_hash.to_lowercase().trim())
    }
}

impl eframe::App for HashApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Frame::none().inner_margin(MARGIN).show(ui, |ui| {
                ui.heading("Hash Calculator");
                ui.add_space(SECTION_SPACING);

                egui::Frame::group(ui.style())
                    .inner_margin(ITEM_SPACING)
                    .show(ui, |ui| {
                        ui.label("Input text:");
                        ui.add(
                            egui::TextEdit::multiline(&mut self.input)
                                .desired_width(f32::INFINITY)
                                .desired_rows(4),
                        );

                        ui.add_space(ITEM_SPACING);

                        ui.checkbox(&mut self.compare_mode, "Compare mode");
                        if self.compare_mode {
                            ui.horizontal(|ui| {
                                ui.label("Expected hash:");
                                ui.text_edit_singleline(&mut self.expected_hash);
                            });
                        }
                    });

                ui.add_space(SECTION_SPACING);

                egui::Frame::group(ui.style())
                    .inner_margin(ITEM_SPACING)
                    .show(ui, |ui| {
                        egui::Grid::new("hash_grid")
                            .num_columns(3)
                            .spacing(GRID_SPACING)
                            .show(ui, |ui| {
                                let md5 = self.md5();
                                let md5_match = self.matches(&md5);
                                ui.label("MD5:");
                                let text = RichText::new(&md5).monospace();
                                let text = match md5_match {
                                    Some(true) => text.color(Color32::GREEN),
                                    Some(false) => text.color(Color32::RED),
                                    None => text,
                                };
                                ui.label(text);
                                if ui.small_button("Copy").clicked() {
                                    ui.output_mut(|o| o.copied_text = md5);
                                }
                                ui.end_row();

                                let sha256 = self.sha256();
                                let sha256_match = self.matches(&sha256);
                                ui.label("SHA-256:");
                                let text = RichText::new(&sha256).monospace().size(11.0);
                                let text = match sha256_match {
                                    Some(true) => text.color(Color32::GREEN),
                                    Some(false) => text.color(Color32::RED),
                                    None => text,
                                };
                                ui.label(text);
                                if ui.small_button("Copy").clicked() {
                                    ui.output_mut(|o| o.copied_text = sha256);
                                }
                                ui.end_row();

                                let sha512 = self.sha512();
                                let sha512_match = self.matches(&sha512);
                                ui.label("SHA-512:");
                                let text = RichText::new(&sha512[..32]).monospace().size(10.0);
                                let text = match sha512_match {
                                    Some(true) => text.color(Color32::GREEN),
                                    Some(false) => text.color(Color32::RED),
                                    None => text,
                                };
                                ui.label(text);
                                if ui.small_button("Copy").clicked() {
                                    ui.output_mut(|o| o.copied_text = sha512);
                                }
                                ui.end_row();
                            });
                    });
            });
        });
    }
}

fn cmd_base64() -> Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([650.0, 450.0]),
        ..Default::default()
    };

    eframe::run_native(
        "dx egui base64",
        options,
        Box::new(|_cc| Ok(Box::new(Base64App::default()))),
    )
    .map_err(|e| anyhow::anyhow!("Failed to run: {}", e))
}

#[derive(Default)]
struct Base64App {
    plain: String,
    encoded: String,
    url_safe: bool,
    error: Option<String>,
}

impl Base64App {
    fn encode(&mut self) {
        use base64::Engine;
        let engine = if self.url_safe {
            base64::engine::general_purpose::URL_SAFE
        } else {
            base64::engine::general_purpose::STANDARD
        };
        self.encoded = engine.encode(self.plain.as_bytes());
        self.error = None;
    }

    fn decode(&mut self) {
        use base64::Engine;
        let engine = if self.url_safe {
            base64::engine::general_purpose::URL_SAFE
        } else {
            base64::engine::general_purpose::STANDARD
        };
        match engine.decode(self.encoded.trim()) {
            Ok(bytes) => {
                self.plain = String::from_utf8_lossy(&bytes).to_string();
                self.error = None;
            }
            Err(e) => {
                self.error = Some(format!("Decode error: {}", e));
            }
        }
    }
}

impl eframe::App for Base64App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Base64 Encoder/Decoder");
            ui.separator();

            ui.checkbox(&mut self.url_safe, "URL-safe encoding");

            ui.separator();

            ui.columns(2, |columns| {
                columns[0].label("Plain text:");
                columns[0].add(
                    egui::TextEdit::multiline(&mut self.plain)
                        .desired_width(f32::INFINITY)
                        .desired_rows(10),
                );
                if columns[0].button("Encode ->").clicked() {
                    self.encode();
                }

                columns[1].label("Base64 encoded:");
                columns[1].add(
                    egui::TextEdit::multiline(&mut self.encoded)
                        .desired_width(f32::INFINITY)
                        .desired_rows(10),
                );
                if columns[1].button("<- Decode").clicked() {
                    self.decode();
                }
            });

            if let Some(err) = &self.error {
                ui.separator();
                ui.label(RichText::new(err).color(Color32::RED));
            }
        });
    }
}

fn cmd_hex() -> Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 450.0]),
        ..Default::default()
    };

    eframe::run_native(
        "dx egui hex",
        options,
        Box::new(|_cc| Ok(Box::new(HexApp::default()))),
    )
    .map_err(|e| anyhow::anyhow!("Failed to run: {}", e))
}

struct HexApp {
    input: String,
    hex_input: String,
}

impl Default for HexApp {
    fn default() -> Self {
        Self {
            input: "Hello, World!".to_string(),
            hex_input: String::new(),
        }
    }
}

impl eframe::App for HexApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hex Encoder/Decoder");
            ui.separator();

            ui.label("Text input:");
            ui.add(
                egui::TextEdit::multiline(&mut self.input)
                    .desired_width(f32::INFINITY)
                    .desired_rows(3),
            );

            ui.separator();

            let hex = hex::encode(self.input.as_bytes());
            ui.horizontal(|ui| {
                ui.label("Hex output:");
                if ui.small_button("Copy").clicked() {
                    ui.output_mut(|o| o.copied_text = hex.clone());
                }
            });
            ui.label(RichText::new(&hex).monospace());

            ui.separator();

            ui.label("Byte view:");
            egui::ScrollArea::vertical()
                .max_height(100.0)
                .show(ui, |ui| {
                    egui::Grid::new("hex_grid")
                        .num_columns(17)
                        .spacing([4.0, 2.0])
                        .show(ui, |ui| {
                            let bytes = self.input.as_bytes();
                            for (i, chunk) in bytes.chunks(16).enumerate() {
                                ui.label(
                                    RichText::new(format!("{:04X}:", i * 16)).monospace().weak(),
                                );
                                for byte in chunk {
                                    ui.label(RichText::new(format!("{:02X}", byte)).monospace());
                                }
                                for _ in chunk.len()..16 {
                                    ui.label("  ");
                                }
                                let ascii: String = chunk
                                    .iter()
                                    .map(|&b| {
                                        if b.is_ascii_graphic() || b == b' ' {
                                            b as char
                                        } else {
                                            '.'
                                        }
                                    })
                                    .collect();
                                ui.label(RichText::new(ascii).monospace());
                                ui.end_row();
                            }
                        });
                });

            ui.separator();

            ui.label("Hex to decode:");
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.hex_input);
                if ui.button("Decode").clicked() {
                    if let Ok(bytes) = hex::decode(self.hex_input.replace(' ', "")) {
                        self.input = String::from_utf8_lossy(&bytes).to_string();
                    }
                }
            });
        });
    }
}

fn cmd_url() -> Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([550.0, 500.0]),
        ..Default::default()
    };

    eframe::run_native(
        "dx egui url",
        options,
        Box::new(|_cc| Ok(Box::new(UrlApp::default()))),
    )
    .map_err(|e| anyhow::anyhow!("Failed to run: {}", e))
}

struct UrlApp {
    input: String,
    encoded: String,
}

impl Default for UrlApp {
    fn default() -> Self {
        Self {
            input: "https://example.com/path?name=John Doe&city=New York".to_string(),
            encoded: String::new(),
        }
    }
}

impl eframe::App for UrlApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("URL Encoder/Decoder");
            ui.separator();

            ui.label("URL:");
            ui.add(egui::TextEdit::singleline(&mut self.input).desired_width(f32::INFINITY));

            ui.horizontal(|ui| {
                if ui.button("Encode").clicked() {
                    // Only encode query parameters
                    if let Ok(url) = url::Url::parse(&self.input) {
                        self.encoded = url.to_string();
                    } else {
                        // Fallback: encode the whole string
                        self.encoded = urlencoding::encode(&self.input).to_string();
                    }
                }
                if ui.button("Decode").clicked() {
                    self.input = urlencoding::decode(&self.input)
                        .unwrap_or_else(|_| self.input.clone().into())
                        .to_string();
                }
                if !self.input.is_empty() {
                    if let Ok(url) = url::Url::parse(&self.input) {
                        ui.hyperlink_to("Open", url.to_string());
                    }
                }
            });

            ui.separator();

            if let Ok(url) = url::Url::parse(&self.input) {
                egui::CollapsingHeader::new("URL Breakdown")
                    .default_open(true)
                    .show(ui, |ui| {
                        egui::Grid::new("url_breakdown")
                            .num_columns(2)
                            .spacing([10.0, 4.0])
                            .show(ui, |ui| {
                                ui.label("Scheme:");
                                ui.label(RichText::new(url.scheme()).monospace());
                                ui.end_row();

                                if let Some(host) = url.host_str() {
                                    ui.label("Host:");
                                    ui.label(RichText::new(host).monospace());
                                    ui.end_row();
                                }

                                if let Some(port) = url.port() {
                                    ui.label("Port:");
                                    ui.label(RichText::new(port.to_string()).monospace());
                                    ui.end_row();
                                }

                                ui.label("Path:");
                                ui.label(RichText::new(url.path()).monospace());
                                ui.end_row();
                            });
                    });

                if url.query().is_some() {
                    egui::CollapsingHeader::new("Query Parameters")
                        .default_open(true)
                        .show(ui, |ui| {
                            egui::Grid::new("query_params")
                                .num_columns(2)
                                .spacing([10.0, 4.0])
                                .show(ui, |ui| {
                                    for (key, value) in url.query_pairs() {
                                        ui.label(RichText::new(key.as_ref()).strong());
                                        ui.label(RichText::new(value.as_ref()).monospace());
                                        ui.end_row();
                                    }
                                });
                        });
                }
            }
        });
    }
}

// ============================================================================
// CONVERTERS
// ============================================================================

fn cmd_timestamp() -> Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([500.0, 500.0]),
        ..Default::default()
    };

    eframe::run_native(
        "dx egui timestamp",
        options,
        Box::new(|_cc| Ok(Box::new(TimestampApp::default()))),
    )
    .map_err(|e| anyhow::anyhow!("Failed to run: {}", e))
}

struct TimestampApp {
    timestamp: i64,
    use_millis: bool,
}

impl Default for TimestampApp {
    fn default() -> Self {
        Self {
            timestamp: chrono::Utc::now().timestamp(),
            use_millis: false,
        }
    }
}

impl eframe::App for TimestampApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Timestamp Converter");
            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Unix timestamp:");
                ui.add(egui::DragValue::new(&mut self.timestamp));
                if ui.button("Now").clicked() {
                    self.timestamp = if self.use_millis {
                        chrono::Utc::now().timestamp_millis()
                    } else {
                        chrono::Utc::now().timestamp()
                    };
                }
            });

            ui.checkbox(&mut self.use_millis, "Milliseconds");

            ui.separator();

            let ts = if self.use_millis {
                chrono::DateTime::from_timestamp_millis(self.timestamp)
            } else {
                chrono::DateTime::from_timestamp(self.timestamp, 0)
            };

            if let Some(dt) = ts {
                let local = dt.with_timezone(&chrono::Local);

                egui::Grid::new("timestamp_grid")
                    .num_columns(2)
                    .spacing([20.0, 8.0])
                    .show(ui, |ui| {
                        ui.label("UTC:");
                        ui.label(RichText::new(dt.to_rfc3339()).monospace());
                        ui.end_row();

                        ui.label("Local:");
                        ui.label(RichText::new(local.to_rfc3339()).monospace());
                        ui.end_row();

                        ui.label("ISO 8601:");
                        ui.label(
                            RichText::new(dt.format("%Y-%m-%dT%H:%M:%SZ").to_string()).monospace(),
                        );
                        ui.end_row();

                        ui.label("RFC 2822:");
                        ui.label(RichText::new(dt.to_rfc2822()).monospace());
                        ui.end_row();

                        ui.label("Date:");
                        ui.label(RichText::new(dt.format("%B %d, %Y").to_string()).monospace());
                        ui.end_row();

                        ui.label("Time:");
                        ui.label(RichText::new(dt.format("%H:%M:%S").to_string()).monospace());
                        ui.end_row();

                        ui.label("Day of week:");
                        ui.label(RichText::new(dt.format("%A").to_string()).monospace());
                        ui.end_row();

                        // Relative time
                        let now = chrono::Utc::now();
                        let duration = now.signed_duration_since(dt);
                        let relative = if duration.num_seconds().abs() < 60 {
                            format!("{} seconds ago", duration.num_seconds())
                        } else if duration.num_minutes().abs() < 60 {
                            format!("{} minutes ago", duration.num_minutes())
                        } else if duration.num_hours().abs() < 24 {
                            format!("{} hours ago", duration.num_hours())
                        } else {
                            format!("{} days ago", duration.num_days())
                        };
                        ui.label("Relative:");
                        ui.label(RichText::new(relative).monospace());
                        ui.end_row();
                    });
            } else {
                ui.label(RichText::new("Invalid timestamp").color(Color32::RED));
            }
        });
    }
}

fn cmd_units() -> Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([450.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native(
        "dx egui units",
        options,
        Box::new(|_cc| Ok(Box::new(UnitsApp::default()))),
    )
    .map_err(|e| anyhow::anyhow!("Failed to run: {}", e))
}

#[derive(PartialEq, Clone, Copy)]
enum UnitCategory {
    Bytes,
    Time,
}

struct UnitsApp {
    category: UnitCategory,
    value: f64,
}

impl Default for UnitsApp {
    fn default() -> Self {
        Self {
            category: UnitCategory::Bytes,
            value: 1024.0,
        }
    }
}

impl eframe::App for UnitsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Unit Converter");
            ui.separator();

            ui.horizontal(|ui| {
                ui.radio_value(&mut self.category, UnitCategory::Bytes, "Bytes");
                ui.radio_value(&mut self.category, UnitCategory::Time, "Time");
            });

            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Value:");
                ui.add(egui::DragValue::new(&mut self.value).speed(1.0));
            });

            ui.separator();

            match self.category {
                UnitCategory::Bytes => {
                    let bytes = self.value;
                    egui::Grid::new("bytes_grid")
                        .num_columns(2)
                        .spacing([20.0, 4.0])
                        .show(ui, |ui| {
                            ui.label("Bytes:");
                            ui.label(RichText::new(format!("{:.0} B", bytes)).monospace());
                            ui.end_row();

                            ui.label("Kilobytes:");
                            ui.label(
                                RichText::new(format!("{:.2} KB", bytes / 1024.0)).monospace(),
                            );
                            ui.end_row();

                            ui.label("Megabytes:");
                            ui.label(
                                RichText::new(format!("{:.2} MB", bytes / 1024.0_f64.powi(2)))
                                    .monospace(),
                            );
                            ui.end_row();

                            ui.label("Gigabytes:");
                            ui.label(
                                RichText::new(format!("{:.2} GB", bytes / 1024.0_f64.powi(3)))
                                    .monospace(),
                            );
                            ui.end_row();

                            ui.label("Terabytes:");
                            ui.label(
                                RichText::new(format!("{:.4} TB", bytes / 1024.0_f64.powi(4)))
                                    .monospace(),
                            );
                            ui.end_row();

                            ui.label("Human:");
                            ui.label(
                                RichText::new(bytesize::ByteSize::b(bytes as u64).to_string())
                                    .monospace(),
                            );
                            ui.end_row();
                        });
                }
                UnitCategory::Time => {
                    let secs = self.value;
                    egui::Grid::new("time_grid")
                        .num_columns(2)
                        .spacing([20.0, 4.0])
                        .show(ui, |ui| {
                            ui.label("Seconds:");
                            ui.label(RichText::new(format!("{:.2} s", secs)).monospace());
                            ui.end_row();

                            ui.label("Minutes:");
                            ui.label(RichText::new(format!("{:.2} min", secs / 60.0)).monospace());
                            ui.end_row();

                            ui.label("Hours:");
                            ui.label(RichText::new(format!("{:.2} h", secs / 3600.0)).monospace());
                            ui.end_row();

                            ui.label("Days:");
                            ui.label(RichText::new(format!("{:.4} d", secs / 86400.0)).monospace());
                            ui.end_row();

                            ui.label("Weeks:");
                            ui.label(
                                RichText::new(format!("{:.4} w", secs / 604800.0)).monospace(),
                            );
                            ui.end_row();

                            ui.label("Human:");
                            ui.label(
                                RichText::new(
                                    humantime::format_duration(std::time::Duration::from_secs_f64(
                                        secs,
                                    ))
                                    .to_string(),
                                )
                                .monospace(),
                            );
                            ui.end_row();
                        });
                }
            }
        });
    }
}

fn cmd_base() -> Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([500.0, 450.0]),
        ..Default::default()
    };

    eframe::run_native(
        "dx egui base",
        options,
        Box::new(|_cc| Ok(Box::new(BaseApp::default()))),
    )
    .map_err(|e| anyhow::anyhow!("Failed to run: {}", e))
}

struct BaseApp {
    decimal: String,
    binary: String,
    octal: String,
    hex: String,
    last_edited: u8, // 0=dec, 1=bin, 2=oct, 3=hex
}

impl Default for BaseApp {
    fn default() -> Self {
        Self {
            decimal: "255".to_string(),
            binary: "11111111".to_string(),
            octal: "377".to_string(),
            hex: "FF".to_string(),
            last_edited: 0,
        }
    }
}

impl BaseApp {
    fn update_from_decimal(&mut self) {
        if let Ok(n) = self.decimal.parse::<u64>() {
            self.binary = format!("{:b}", n);
            self.octal = format!("{:o}", n);
            self.hex = format!("{:X}", n);
        }
    }

    fn update_from_binary(&mut self) {
        if let Ok(n) = u64::from_str_radix(&self.binary, 2) {
            self.decimal = n.to_string();
            self.octal = format!("{:o}", n);
            self.hex = format!("{:X}", n);
        }
    }

    fn update_from_octal(&mut self) {
        if let Ok(n) = u64::from_str_radix(&self.octal, 8) {
            self.decimal = n.to_string();
            self.binary = format!("{:b}", n);
            self.hex = format!("{:X}", n);
        }
    }

    fn update_from_hex(&mut self) {
        if let Ok(n) = u64::from_str_radix(&self.hex, 16) {
            self.decimal = n.to_string();
            self.binary = format!("{:b}", n);
            self.octal = format!("{:o}", n);
        }
    }
}

impl eframe::App for BaseApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Number Base Converter");
            ui.separator();

            egui::Grid::new("base_grid")
                .num_columns(2)
                .spacing([20.0, 10.0])
                .show(ui, |ui| {
                    ui.label("Decimal (base 10):");
                    let resp = ui.add(
                        egui::TextEdit::singleline(&mut self.decimal)
                            .font(egui::TextStyle::Monospace),
                    );
                    if resp.changed() {
                        self.last_edited = 0;
                        self.update_from_decimal();
                    }
                    ui.end_row();

                    ui.label("Binary (base 2):");
                    let resp = ui.add(
                        egui::TextEdit::singleline(&mut self.binary)
                            .font(egui::TextStyle::Monospace),
                    );
                    if resp.changed() {
                        self.last_edited = 1;
                        self.update_from_binary();
                    }
                    ui.end_row();

                    ui.label("Octal (base 8):");
                    let resp = ui.add(
                        egui::TextEdit::singleline(&mut self.octal)
                            .font(egui::TextStyle::Monospace),
                    );
                    if resp.changed() {
                        self.last_edited = 2;
                        self.update_from_octal();
                    }
                    ui.end_row();

                    ui.label("Hexadecimal (base 16):");
                    let resp = ui.add(
                        egui::TextEdit::singleline(&mut self.hex).font(egui::TextStyle::Monospace),
                    );
                    if resp.changed() {
                        self.last_edited = 3;
                        self.update_from_hex();
                    }
                    ui.end_row();
                });

            ui.separator();

            // Bit visualization
            if let Ok(n) = self.decimal.parse::<u64>() {
                ui.label("Bit visualization:");
                ui.horizontal_wrapped(|ui| {
                    let bits = format!("{:064b}", n);
                    let bits = bits.trim_start_matches('0');
                    let bits = if bits.is_empty() { "0" } else { bits };
                    for (i, c) in bits.chars().enumerate() {
                        let color = if c == '1' {
                            Color32::GREEN
                        } else {
                            Color32::GRAY
                        };
                        ui.label(RichText::new(c.to_string()).monospace().color(color));
                        if (bits.len() - i - 1) % 4 == 0 && i < bits.len() - 1 {
                            ui.label(" ");
                        }
                    }
                });
            }
        });
    }
}

fn cmd_json() -> Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([750.0, 550.0]),
        ..Default::default()
    };

    eframe::run_native(
        "dx egui json",
        options,
        Box::new(|_cc| Ok(Box::new(JsonApp::default()))),
    )
    .map_err(|e| anyhow::anyhow!("Failed to run: {}", e))
}

struct JsonApp {
    input: String,
    output: String,
    indent: usize,
    sort_keys: bool,
    error: Option<String>,
}

impl Default for JsonApp {
    fn default() -> Self {
        Self {
            input: r#"{"name":"John","age":30,"city":"New York"}"#.to_string(),
            output: String::new(),
            indent: 2,
            sort_keys: false,
            error: None,
        }
    }
}

impl JsonApp {
    fn format(&mut self) {
        match serde_json::from_str::<serde_json::Value>(&self.input) {
            Ok(mut value) => {
                if self.sort_keys {
                    value = sort_json_keys(value);
                }
                match serde_json::to_string_pretty(&value) {
                    Ok(s) => {
                        // Apply custom indent
                        let indent_str = " ".repeat(self.indent);
                        self.output = s
                            .lines()
                            .map(|line| {
                                let spaces = line.len() - line.trim_start().len();
                                format!("{}{}", indent_str.repeat(spaces / 2), line.trim_start())
                            })
                            .collect::<Vec<_>>()
                            .join("\n");
                        self.error = None;
                    }
                    Err(e) => self.error = Some(e.to_string()),
                }
            }
            Err(e) => {
                self.error = Some(format!("Parse error: {}", e));
            }
        }
    }

    fn minify(&mut self) {
        match serde_json::from_str::<serde_json::Value>(&self.input) {
            Ok(value) => {
                self.output = serde_json::to_string(&value).unwrap_or_default();
                self.error = None;
            }
            Err(e) => {
                self.error = Some(format!("Parse error: {}", e));
            }
        }
    }
}

fn sort_json_keys(value: serde_json::Value) -> serde_json::Value {
    match value {
        serde_json::Value::Object(map) => {
            let mut sorted: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
            let mut keys: Vec<_> = map.keys().cloned().collect();
            keys.sort();
            for key in keys {
                if let Some(v) = map.get(&key) {
                    sorted.insert(key, sort_json_keys(v.clone()));
                }
            }
            serde_json::Value::Object(sorted)
        }
        serde_json::Value::Array(arr) => {
            serde_json::Value::Array(arr.into_iter().map(sort_json_keys).collect())
        }
        other => other,
    }
}

impl eframe::App for JsonApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("json_toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Format").clicked() {
                    self.format();
                }
                if ui.button("Minify").clicked() {
                    self.minify();
                }
                ui.separator();
                ui.label("Indent:");
                ui.add(egui::Slider::new(&mut self.indent, 1..=8));
                ui.checkbox(&mut self.sort_keys, "Sort keys");
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(err) = &self.error {
                ui.label(RichText::new(err).color(Color32::RED));
                ui.separator();
            }

            ui.columns(2, |columns| {
                columns[0].label("Input:");
                egui::ScrollArea::vertical()
                    .id_salt("json_input")
                    .show(&mut columns[0], |ui| {
                        ui.add(
                            egui::TextEdit::multiline(&mut self.input)
                                .font(egui::TextStyle::Monospace)
                                .desired_width(f32::INFINITY),
                        );
                    });

                columns[1].label("Output:");
                egui::ScrollArea::vertical()
                    .id_salt("json_output")
                    .show(&mut columns[1], |ui| {
                        ui.add(
                            egui::TextEdit::multiline(&mut self.output.as_str())
                                .font(egui::TextStyle::Monospace)
                                .desired_width(f32::INFINITY),
                        );
                    });
            });
        });
    }
}

// ============================================================================
// UTILITIES
// ============================================================================

fn cmd_regex() -> Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([650.0, 550.0]),
        ..Default::default()
    };

    eframe::run_native(
        "dx egui regex",
        options,
        Box::new(|_cc| Ok(Box::new(RegexApp::default()))),
    )
    .map_err(|e| anyhow::anyhow!("Failed to run: {}", e))
}

struct RegexApp {
    pattern: String,
    test_string: String,
    case_insensitive: bool,
    multiline: bool,
    selected_preset: usize,
}

impl Default for RegexApp {
    fn default() -> Self {
        Self {
            pattern: r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b".to_string(),
            test_string: "Contact us at test@example.com or support@company.org for help."
                .to_string(),
            case_insensitive: true,
            multiline: false,
            selected_preset: 0,
        }
    }
}

const REGEX_PRESETS: &[(&str, &str)] = &[
    (
        "Email",
        r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b",
    ),
    ("URL", r"https?://[^\s]+"),
    ("Phone", r"\b\d{3}[-.]?\d{3}[-.]?\d{4}\b"),
    ("IPv4", r"\b\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}\b"),
    ("Date", r"\b\d{4}-\d{2}-\d{2}\b"),
];

impl eframe::App for RegexApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Regex Tester");
            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Preset:");
                egui::ComboBox::from_id_salt("regex_preset")
                    .selected_text(REGEX_PRESETS[self.selected_preset].0)
                    .show_ui(ui, |ui| {
                        for (i, (name, _)) in REGEX_PRESETS.iter().enumerate() {
                            if ui
                                .selectable_value(&mut self.selected_preset, i, *name)
                                .clicked()
                            {
                                self.pattern = REGEX_PRESETS[i].1.to_string();
                            }
                        }
                    });
            });

            ui.horizontal(|ui| {
                ui.label("Pattern:");
                ui.add(
                    egui::TextEdit::singleline(&mut self.pattern)
                        .font(egui::TextStyle::Monospace)
                        .desired_width(f32::INFINITY),
                );
            });

            ui.horizontal(|ui| {
                ui.checkbox(&mut self.case_insensitive, "Case insensitive");
                ui.checkbox(&mut self.multiline, "Multiline");
            });

            ui.separator();

            ui.label("Test string:");
            ui.add(
                egui::TextEdit::multiline(&mut self.test_string)
                    .font(egui::TextStyle::Monospace)
                    .desired_width(f32::INFINITY)
                    .desired_rows(5),
            );

            ui.separator();

            // Compile and run regex
            let pattern = if self.case_insensitive {
                format!("(?i){}", self.pattern)
            } else {
                self.pattern.clone()
            };

            match regex::Regex::new(&pattern) {
                Ok(re) => {
                    let matches: Vec<_> = re.find_iter(&self.test_string).collect();
                    ui.label(format!("Matches found: {}", matches.len()));

                    if !matches.is_empty() {
                        ui.separator();
                        egui::ScrollArea::vertical()
                            .max_height(150.0)
                            .show(ui, |ui| {
                                egui::Grid::new("regex_matches")
                                    .num_columns(3)
                                    .spacing([10.0, 4.0])
                                    .show(ui, |ui| {
                                        ui.label(RichText::new("#").strong());
                                        ui.label(RichText::new("Match").strong());
                                        ui.label(RichText::new("Position").strong());
                                        ui.end_row();

                                        for (i, m) in matches.iter().enumerate() {
                                            ui.label(format!("{}", i + 1));
                                            ui.label(
                                                RichText::new(m.as_str())
                                                    .monospace()
                                                    .color(Color32::GREEN),
                                            );
                                            ui.label(format!("{}..{}", m.start(), m.end()));
                                            ui.end_row();
                                        }
                                    });
                            });
                    }

                    // Capture groups
                    if let Some(caps) = re.captures(&self.test_string) {
                        if caps.len() > 1 {
                            ui.separator();
                            ui.label("Capture groups:");
                            for (i, cap) in caps.iter().enumerate().skip(1) {
                                if let Some(c) = cap {
                                    ui.label(format!("  Group {}: {}", i, c.as_str()));
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    ui.label(RichText::new(format!("Invalid regex: {}", e)).color(Color32::RED));
                }
            }
        });
    }
}

fn cmd_diff() -> Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([850.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "dx egui diff",
        options,
        Box::new(|_cc| Ok(Box::new(DiffApp::default()))),
    )
    .map_err(|e| anyhow::anyhow!("Failed to run: {}", e))
}

struct DiffApp {
    left: String,
    right: String,
}

impl Default for DiffApp {
    fn default() -> Self {
        Self {
            left: "Hello World\nThis is a test\nLine three\nLine four".to_string(),
            right: "Hello World\nThis is a demo\nLine three\nLine five\nLine six".to_string(),
        }
    }
}

impl eframe::App for DiffApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Text Diff Viewer");
            ui.separator();

            ui.columns(2, |columns| {
                columns[0].label("Original:");
                columns[0].add(
                    egui::TextEdit::multiline(&mut self.left)
                        .font(egui::TextStyle::Monospace)
                        .desired_width(f32::INFINITY)
                        .desired_rows(8),
                );

                columns[1].label("Modified:");
                columns[1].add(
                    egui::TextEdit::multiline(&mut self.right)
                        .font(egui::TextStyle::Monospace)
                        .desired_width(f32::INFINITY)
                        .desired_rows(8),
                );
            });

            ui.separator();
            ui.label("Diff output:");

            egui::ScrollArea::vertical()
                .max_height(200.0)
                .show(ui, |ui| {
                    let left_lines: Vec<&str> = self.left.lines().collect();
                    let right_lines: Vec<&str> = self.right.lines().collect();

                    let max_len = left_lines.len().max(right_lines.len());

                    for i in 0..max_len {
                        let l = left_lines.get(i).copied();
                        let r = right_lines.get(i).copied();

                        match (l, r) {
                            (Some(left), Some(right)) if left == right => {
                                ui.label(RichText::new(format!("  {}", left)).monospace());
                            }
                            (Some(left), Some(right)) => {
                                ui.label(
                                    RichText::new(format!("- {}", left))
                                        .monospace()
                                        .color(Color32::RED),
                                );
                                ui.label(
                                    RichText::new(format!("+ {}", right))
                                        .monospace()
                                        .color(Color32::GREEN),
                                );
                            }
                            (Some(left), None) => {
                                ui.label(
                                    RichText::new(format!("- {}", left))
                                        .monospace()
                                        .color(Color32::RED),
                                );
                            }
                            (None, Some(right)) => {
                                ui.label(
                                    RichText::new(format!("+ {}", right))
                                        .monospace()
                                        .color(Color32::GREEN),
                                );
                            }
                            (None, None) => {}
                        }
                    }
                });
        });
    }
}

fn cmd_stopwatch() -> Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 450.0]),
        ..Default::default()
    };

    eframe::run_native(
        "dx egui stopwatch",
        options,
        Box::new(|_cc| Ok(Box::new(StopwatchApp::default()))),
    )
    .map_err(|e| anyhow::anyhow!("Failed to run: {}", e))
}

struct StopwatchApp {
    running: bool,
    start_time: Option<std::time::Instant>,
    elapsed: std::time::Duration,
    laps: Vec<std::time::Duration>,
}

impl Default for StopwatchApp {
    fn default() -> Self {
        Self {
            running: false,
            start_time: None,
            elapsed: std::time::Duration::ZERO,
            laps: Vec::new(),
        }
    }
}

impl StopwatchApp {
    fn current_elapsed(&self) -> std::time::Duration {
        if let Some(start) = self.start_time {
            self.elapsed + start.elapsed()
        } else {
            self.elapsed
        }
    }

    fn format_duration(d: std::time::Duration) -> String {
        let total_secs = d.as_secs();
        let millis = d.subsec_millis();
        let hours = total_secs / 3600;
        let mins = (total_secs % 3600) / 60;
        let secs = total_secs % 60;
        format!("{:02}:{:02}:{:02}.{:03}", hours, mins, secs, millis)
    }
}

impl eframe::App for StopwatchApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Stopwatch");
            ui.separator();

            let elapsed = self.current_elapsed();
            ui.vertical_centered(|ui| {
                ui.label(
                    RichText::new(Self::format_duration(elapsed))
                        .size(48.0)
                        .monospace(),
                );
            });

            ui.separator();

            ui.horizontal(|ui| {
                if self.running {
                    if ui.button("Stop").clicked() {
                        self.elapsed = self.current_elapsed();
                        self.start_time = None;
                        self.running = false;
                    }
                } else if ui.button("Start").clicked() {
                    self.start_time = Some(std::time::Instant::now());
                    self.running = true;
                }

                if ui.button("Lap").clicked() && self.running {
                    self.laps.push(self.current_elapsed());
                }

                if ui.button("Reset").clicked() {
                    self.running = false;
                    self.start_time = None;
                    self.elapsed = std::time::Duration::ZERO;
                    self.laps.clear();
                }
            });

            if !self.laps.is_empty() {
                ui.separator();
                ui.label("Laps:");
                egui::ScrollArea::vertical()
                    .max_height(150.0)
                    .show(ui, |ui| {
                        for (i, lap) in self.laps.iter().enumerate() {
                            ui.horizontal(|ui| {
                                ui.label(format!("Lap {}:", i + 1));
                                ui.label(RichText::new(Self::format_duration(*lap)).monospace());
                            });
                        }
                    });
            }
        });

        if self.running {
            ctx.request_repaint();
        }
    }
}

fn cmd_calculator() -> Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([450.0, 550.0]),
        ..Default::default()
    };

    eframe::run_native(
        "dx egui calculator",
        options,
        Box::new(|_cc| Ok(Box::new(CalculatorApp::default()))),
    )
    .map_err(|e| anyhow::anyhow!("Failed to run: {}", e))
}

#[derive(Default)]
struct CalculatorApp {
    expression: String,
    result: String,
    history: Vec<(String, String)>,
    error: Option<String>,
}

impl CalculatorApp {
    fn evaluate(&mut self) {
        use crate::expr;
        match expr::eval(&self.expression) {
            Ok(value) => {
                self.result = format!("{}", value);
                self.history
                    .push((self.expression.clone(), self.result.clone()));
                self.error = None;
            }
            Err(e) => {
                self.error = Some(e.to_string());
            }
        }
    }

    fn append(&mut self, s: &str) {
        self.expression.push_str(s);
    }
}

impl eframe::App for CalculatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Expression Calculator");
            ui.separator();

            // Expression input
            ui.horizontal(|ui| {
                ui.add(
                    egui::TextEdit::singleline(&mut self.expression)
                        .font(egui::TextStyle::Monospace)
                        .desired_width(f32::INFINITY),
                );
                if ui.button("=").clicked() {
                    self.evaluate();
                }
            });

            // Result display
            if !self.result.is_empty() {
                ui.label(
                    RichText::new(format!("= {}", self.result))
                        .size(24.0)
                        .monospace(),
                );
            }

            if let Some(err) = &self.error {
                ui.label(RichText::new(err).color(Color32::RED));
            }

            ui.separator();

            // Calculator buttons
            egui::Grid::new("calc_buttons")
                .num_columns(4)
                .spacing([8.0, 8.0])
                .show(ui, |ui| {
                    let buttons = [
                        ["C", "(", ")", "/"],
                        ["7", "8", "9", "*"],
                        ["4", "5", "6", "-"],
                        ["1", "2", "3", "+"],
                        ["0", ".", "^", "="],
                    ];

                    for row in buttons {
                        for btn in row {
                            let button = egui::Button::new(RichText::new(btn).size(20.0))
                                .min_size(egui::vec2(50.0, 40.0));
                            if ui.add(button).clicked() {
                                match btn {
                                    "C" => {
                                        self.expression.clear();
                                        self.result.clear();
                                        self.error = None;
                                    }
                                    "=" => self.evaluate(),
                                    _ => self.append(btn),
                                }
                            }
                        }
                        ui.end_row();
                    }
                });

            // Functions reference
            egui::CollapsingHeader::new("Functions")
                .default_open(false)
                .show(ui, |ui| {
                    ui.label("sin, cos, tan, asin, acos, atan");
                    ui.label("sqrt, cbrt, abs, floor, ceil, round");
                    ui.label("ln, log10, log2, exp");
                    ui.label("min, max, pow");
                    ui.label("Constants: pi, e, tau");
                });

            // History
            if !self.history.is_empty() {
                egui::CollapsingHeader::new("History")
                    .default_open(true)
                    .show(ui, |ui| {
                        egui::ScrollArea::vertical()
                            .max_height(100.0)
                            .show(ui, |ui| {
                                for (expr, res) in self.history.iter().rev().take(10) {
                                    ui.horizontal(|ui| {
                                        ui.label(RichText::new(expr).monospace().weak());
                                        ui.label("=");
                                        ui.label(RichText::new(res).monospace());
                                    });
                                }
                            });
                    });
            }
        });
    }
}

// ============================================================================
// TEXT TOOLS
// ============================================================================

fn cmd_case() -> Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([550.0, 500.0]),
        ..Default::default()
    };

    eframe::run_native(
        "dx egui case",
        options,
        Box::new(|_cc| Ok(Box::new(CaseApp::default()))),
    )
    .map_err(|e| anyhow::anyhow!("Failed to run: {}", e))
}

struct CaseApp {
    input: String,
}

impl Default for CaseApp {
    fn default() -> Self {
        Self {
            input: "hello world example".to_string(),
        }
    }
}

impl eframe::App for CaseApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        use heck::*;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Case Converter");
            ui.separator();

            ui.label("Input text:");
            ui.add(egui::TextEdit::singleline(&mut self.input).desired_width(f32::INFINITY));

            ui.separator();

            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::Grid::new("case_grid")
                    .num_columns(3)
                    .spacing([10.0, 8.0])
                    .show(ui, |ui| {
                        let cases: Vec<(&str, String)> = vec![
                            ("UPPERCASE", self.input.to_uppercase()),
                            ("lowercase", self.input.to_lowercase()),
                            ("Title Case", self.input.to_title_case()),
                            ("camelCase", self.input.to_lower_camel_case()),
                            ("PascalCase", self.input.to_upper_camel_case()),
                            ("snake_case", self.input.to_snake_case()),
                            ("SCREAMING_SNAKE", self.input.to_shouty_snake_case()),
                            ("kebab-case", self.input.to_kebab_case()),
                            ("SCREAMING-KEBAB", self.input.to_shouty_kebab_case()),
                            ("Train-Case", self.input.to_train_case()),
                        ];

                        for (name, value) in cases {
                            ui.label(RichText::new(name).strong());
                            ui.label(RichText::new(&value).monospace());
                            if ui.small_button("Copy").clicked() {
                                ui.output_mut(|o| o.copied_text = value);
                            }
                            ui.end_row();
                        }
                    });
            });
        });
    }
}

fn cmd_text_stats() -> Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([550.0, 500.0]),
        ..Default::default()
    };

    eframe::run_native(
        "dx egui text-stats",
        options,
        Box::new(|_cc| Ok(Box::new(TextStatsApp::default()))),
    )
    .map_err(|e| anyhow::anyhow!("Failed to run: {}", e))
}

struct TextStatsApp {
    input: String,
}

impl Default for TextStatsApp {
    fn default() -> Self {
        Self {
            input: "The quick brown fox jumps over the lazy dog.\n\nThis is a sample paragraph to demonstrate text statistics. It includes multiple sentences and words.".to_string(),
        }
    }
}

impl eframe::App for TextStatsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Text Statistics");
            ui.separator();

            ui.label("Input text:");
            ui.add(
                egui::TextEdit::multiline(&mut self.input)
                    .desired_width(f32::INFINITY)
                    .desired_rows(8),
            );

            ui.separator();

            let chars = self.input.chars().count();
            let chars_no_spaces = self.input.chars().filter(|c| !c.is_whitespace()).count();
            let words = self.input.split_whitespace().count();
            let lines = self.input.lines().count();
            let paragraphs = self.input.split("\n\n").filter(|s| !s.is_empty()).count();
            let sentences = self.input.matches(['.', '!', '?']).count();

            // Reading time (average 200 words per minute)
            let reading_mins = words as f32 / 200.0;

            egui::Grid::new("stats_grid")
                .num_columns(2)
                .spacing([20.0, 8.0])
                .show(ui, |ui| {
                    ui.label("Characters:");
                    ui.label(RichText::new(format!("{}", chars)).monospace());
                    ui.end_row();

                    ui.label("Characters (no spaces):");
                    ui.label(RichText::new(format!("{}", chars_no_spaces)).monospace());
                    ui.end_row();

                    ui.label("Words:");
                    ui.label(RichText::new(format!("{}", words)).monospace());
                    ui.end_row();

                    ui.label("Sentences:");
                    ui.label(RichText::new(format!("{}", sentences)).monospace());
                    ui.end_row();

                    ui.label("Lines:");
                    ui.label(RichText::new(format!("{}", lines)).monospace());
                    ui.end_row();

                    ui.label("Paragraphs:");
                    ui.label(RichText::new(format!("{}", paragraphs)).monospace());
                    ui.end_row();

                    ui.label("Reading time:");
                    ui.label(RichText::new(format!("{:.1} min", reading_mins)).monospace());
                    ui.end_row();
                });

            ui.separator();

            // Word frequency
            if words > 0 {
                ui.label("Top words:");
                let mut word_freq: std::collections::HashMap<String, usize> =
                    std::collections::HashMap::new();
                for word in self.input.split_whitespace() {
                    let word = word
                        .to_lowercase()
                        .trim_matches(|c: char| !c.is_alphabetic())
                        .to_string();
                    if !word.is_empty() && word.len() > 2 {
                        *word_freq.entry(word).or_insert(0) += 1;
                    }
                }
                let mut freq_vec: Vec<_> = word_freq.into_iter().collect();
                freq_vec.sort_by(|a, b| b.1.cmp(&a.1));

                egui::ScrollArea::vertical()
                    .max_height(100.0)
                    .show(ui, |ui| {
                        for (word, count) in freq_vec.iter().take(10) {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new(word).monospace());
                                ui.label(format!("({})", count));
                            });
                        }
                    });
            }
        });
    }
}

fn cmd_markdown() -> Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([850.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "dx egui markdown",
        options,
        Box::new(|_cc| Ok(Box::new(MarkdownApp::default()))),
    )
    .map_err(|e| anyhow::anyhow!("Failed to run: {}", e))
}

struct MarkdownApp {
    source: String,
}

impl Default for MarkdownApp {
    fn default() -> Self {
        Self {
            source: "# Welcome to Markdown Preview\n\nThis is a **bold** and *italic* text demo.\n\n## Features\n\n- Item one\n- Item two\n- Item three\n\n### Code\n\n`inline code` example\n\n> This is a blockquote\n\n---\n\nVisit [egui](https://github.com/emilk/egui) for more!".to_string(),
        }
    }
}

impl MarkdownApp {
    fn render_markdown(&self, ui: &mut egui::Ui) {
        for line in self.source.lines() {
            let trimmed = line.trim();

            if let Some(heading) = trimmed.strip_prefix("# ") {
                ui.label(RichText::new(heading).heading().size(28.0));
            } else if let Some(heading) = trimmed.strip_prefix("## ") {
                ui.label(RichText::new(heading).heading().size(24.0));
            } else if let Some(heading) = trimmed.strip_prefix("### ") {
                ui.label(RichText::new(heading).heading().size(20.0));
            } else if let Some(item) = trimmed
                .strip_prefix("- ")
                .or_else(|| trimmed.strip_prefix("* "))
            {
                ui.horizontal(|ui| {
                    ui.label("•");
                    self.render_inline(ui, item);
                });
            } else if let Some(quote) = trimmed.strip_prefix("> ") {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("│").color(Color32::GRAY));
                    ui.label(RichText::new(quote).italics().color(Color32::GRAY));
                });
            } else if trimmed == "---" {
                ui.separator();
            } else if trimmed.is_empty() {
                ui.add_space(8.0);
            } else {
                self.render_inline(ui, trimmed);
            }
        }
    }

    fn render_inline(&self, ui: &mut egui::Ui, text: &str) {
        // Simple inline parsing for bold, italic, code, and links
        let mut remaining = text;
        let mut parts: Vec<(String, egui::RichText)> = Vec::new();

        while !remaining.is_empty() {
            if let Some(start) = remaining.find("**") {
                if let Some(end) = remaining[start + 2..].find("**") {
                    if start > 0 {
                        parts.push((
                            remaining[..start].to_string(),
                            RichText::new(&remaining[..start]),
                        ));
                    }
                    let bold_text = &remaining[start + 2..start + 2 + end];
                    parts.push((bold_text.to_string(), RichText::new(bold_text).strong()));
                    remaining = &remaining[start + 4 + end..];
                    continue;
                }
            }
            if let Some(start) = remaining.find('*') {
                if !remaining[start..].starts_with("**") {
                    if let Some(end) = remaining[start + 1..].find('*') {
                        if start > 0 {
                            parts.push((
                                remaining[..start].to_string(),
                                RichText::new(&remaining[..start]),
                            ));
                        }
                        let italic_text = &remaining[start + 1..start + 1 + end];
                        parts.push((
                            italic_text.to_string(),
                            RichText::new(italic_text).italics(),
                        ));
                        remaining = &remaining[start + 2 + end..];
                        continue;
                    }
                }
            }
            if let Some(start) = remaining.find('`') {
                if let Some(end) = remaining[start + 1..].find('`') {
                    if start > 0 {
                        parts.push((
                            remaining[..start].to_string(),
                            RichText::new(&remaining[..start]),
                        ));
                    }
                    let code_text = &remaining[start + 1..start + 1 + end];
                    parts.push((
                        code_text.to_string(),
                        RichText::new(code_text)
                            .monospace()
                            .background_color(Color32::from_gray(40)),
                    ));
                    remaining = &remaining[start + 2 + end..];
                    continue;
                }
            }
            // No more special formatting
            parts.push((remaining.to_string(), RichText::new(remaining)));
            break;
        }

        ui.horizontal_wrapped(|ui| {
            for (_, rich) in parts {
                ui.label(rich);
            }
        });
    }
}

impl eframe::App for MarkdownApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("md_toolbar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Bold").clicked() {
                    self.source.push_str("**bold**");
                }
                if ui.button("Italic").clicked() {
                    self.source.push_str("*italic*");
                }
                if ui.button("Code").clicked() {
                    self.source.push_str("`code`");
                }
                if ui.button("H1").clicked() {
                    self.source.push_str("\n# Heading\n");
                }
                if ui.button("H2").clicked() {
                    self.source.push_str("\n## Heading\n");
                }
                if ui.button("List").clicked() {
                    self.source.push_str("\n- Item\n");
                }
                if ui.button("Quote").clicked() {
                    self.source.push_str("\n> Quote\n");
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.columns(2, |columns| {
                columns[0].label("Source:");
                egui::ScrollArea::vertical()
                    .id_salt("md_source")
                    .show(&mut columns[0], |ui| {
                        ui.add(
                            egui::TextEdit::multiline(&mut self.source)
                                .font(egui::TextStyle::Monospace)
                                .desired_width(f32::INFINITY),
                        );
                    });

                columns[1].label("Preview:");
                egui::ScrollArea::vertical()
                    .id_salt("md_preview")
                    .show(&mut columns[1], |ui| {
                        self.render_markdown(ui);
                    });
            });
        });
    }
}

fn cmd_timer() -> Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native(
        "dx egui timer",
        options,
        Box::new(|_cc| Ok(Box::new(TimerApp::default()))),
    )
    .map_err(|e| anyhow::anyhow!("Failed to run: {}", e))
}

struct TimerApp {
    work_mins: u32,
    break_mins: u32,
    running: bool,
    is_break: bool,
    remaining: std::time::Duration,
    last_tick: Option<std::time::Instant>,
    sessions: u32,
    auto_start_break: bool,
}

impl Default for TimerApp {
    fn default() -> Self {
        Self {
            work_mins: 25,
            break_mins: 5,
            running: false,
            is_break: false,
            remaining: std::time::Duration::from_secs(25 * 60),
            last_tick: None,
            sessions: 0,
            auto_start_break: true,
        }
    }
}

impl TimerApp {
    fn format_time(&self) -> String {
        let secs = self.remaining.as_secs();
        format!("{:02}:{:02}", secs / 60, secs % 60)
    }

    fn reset_work(&mut self) {
        self.remaining = std::time::Duration::from_secs((self.work_mins * 60) as u64);
        self.is_break = false;
    }

    fn reset_break(&mut self) {
        self.remaining = std::time::Duration::from_secs((self.break_mins * 60) as u64);
        self.is_break = true;
    }
}

impl eframe::App for TimerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Update timer
        if self.running {
            if let Some(last) = self.last_tick {
                let elapsed = last.elapsed();
                if elapsed >= self.remaining {
                    self.remaining = std::time::Duration::ZERO;
                    self.running = false;
                    if !self.is_break {
                        self.sessions += 1;
                        if self.auto_start_break {
                            self.reset_break();
                            self.running = true;
                        }
                    } else {
                        self.reset_work();
                    }
                } else {
                    self.remaining -= elapsed;
                }
            }
            self.last_tick = Some(std::time::Instant::now());
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Pomodoro Timer");
            ui.separator();

            let phase = if self.is_break { "Break" } else { "Work" };
            let color = if self.is_break {
                Color32::GREEN
            } else {
                Color32::from_rgb(255, 100, 100)
            };

            ui.vertical_centered(|ui| {
                ui.label(RichText::new(phase).size(24.0).color(color));
                ui.label(RichText::new(self.format_time()).size(64.0).monospace());

                // Progress bar
                let total = if self.is_break {
                    (self.break_mins * 60) as f32
                } else {
                    (self.work_mins * 60) as f32
                };
                let progress = 1.0 - (self.remaining.as_secs_f32() / total);
                ui.add(egui::ProgressBar::new(progress).fill(color));
            });

            ui.separator();

            ui.horizontal(|ui| {
                if self.running {
                    if ui.button("Pause").clicked() {
                        self.running = false;
                    }
                } else if ui.button("Start").clicked() {
                    self.running = true;
                    self.last_tick = Some(std::time::Instant::now());
                }

                if ui.button("Reset").clicked() {
                    self.running = false;
                    self.reset_work();
                }

                if ui.button("Skip").clicked() {
                    if self.is_break {
                        self.reset_work();
                    } else {
                        self.sessions += 1;
                        self.reset_break();
                    }
                }
            });

            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Work:");
                ui.add(
                    egui::DragValue::new(&mut self.work_mins)
                        .range(1..=60)
                        .suffix(" min"),
                );
                ui.label("Break:");
                ui.add(
                    egui::DragValue::new(&mut self.break_mins)
                        .range(1..=30)
                        .suffix(" min"),
                );
            });

            ui.checkbox(&mut self.auto_start_break, "Auto-start break");

            ui.separator();

            ui.label(format!("Sessions completed: {}", self.sessions));
        });

        if self.running {
            ctx.request_repaint();
        }
    }
}
