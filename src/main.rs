use eframe::egui;
use std::fs;
use std::path::PathBuf;
use std::time::{Instant, Duration};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
        .with_inner_size([1000.0, 700.0])
        .with_title("rust-text"), // Titel ohne "Pro"
        ..Default::default()
    };

    eframe::run_native(
        "rust-text",
        options,
        Box::new(|_cc| Box::new(RustTextEditor::default())),
    )
}

struct RustTextEditor {
    content: String,
    current_file: Option<PathBuf>,
    font_size: f32,
    last_save: Instant,
}

impl Default for RustTextEditor {
    fn default() -> Self {
        Self {
            content: String::new(),
            current_file: None,
            font_size: 14.0,
            last_save: Instant::now(),
        }
    }
}

impl eframe::App for RustTextEditor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // --- AUTO SAVE ---
        if self.last_save.elapsed() > Duration::from_secs(30) {
            if self.current_file.is_some() {
                self.save_file();
            }
            self.last_save = Instant::now();
        }

        // --- KEYBOARD SHORTCUTS ---
        if ctx.input_mut(|i| i.consume_shortcut(&egui::KeyboardShortcut::new(egui::Modifiers::COMMAND, egui::Key::N))) {
            self.new_file();
        }
        if ctx.input_mut(|i| i.consume_shortcut(&egui::KeyboardShortcut::new(egui::Modifiers::COMMAND, egui::Key::O))) {
            self.open_file();
        }
        if ctx.input_mut(|i| i.consume_shortcut(&egui::KeyboardShortcut::new(egui::Modifiers::COMMAND, egui::Key::S))) {
            self.save_file();
        }
        if ctx.input_mut(|i| i.consume_shortcut(&egui::KeyboardShortcut::new(egui::Modifiers::COMMAND | egui::Modifiers::SHIFT, egui::Key::S))) {
            self.save_as();
        }

        // --- TOP PANEL: Menu Bar ---
        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New (Ctrl+N)").clicked() { self.new_file(); ui.close_menu(); }
                    if ui.button("Open... (Ctrl+O)").clicked() { self.open_file(); ui.close_menu(); }
                    ui.separator();
                    if ui.button("Save (Ctrl+S)").clicked() { self.save_file(); ui.close_menu(); }
                    if ui.button("Save As... (Ctrl+Shift+S)").clicked() { self.save_as(); ui.close_menu(); }
                    ui.separator();
                    if ui.button("Exit").clicked() { ctx.send_viewport_cmd(egui::ViewportCommand::Close); }
                });

                ui.menu_button("View", |ui| {
                    ui.label("Theme:");
                    egui::widgets::global_dark_light_mode_buttons(ui);
                    ui.separator();
                    ui.label("Font Size:");
                    ui.add(egui::Slider::new(&mut self.font_size, 8.0..=40.0));
                    if ui.button("Reset Font Size").clicked() {
                        self.font_size = 14.0;
                    }
                });
            });
        });

        // --- BOTTOM PANEL: Status Bar ---
        egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("Lines: {}", self.content.lines().count()));
                ui.label(format!("Chars: {}", self.content.len()));

                if let Some(path) = &self.current_file {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(format!("{}", path.display()));
                    });
                }
            });
        });

        // --- CENTRAL PANEL: Editor ---
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut style = (*ctx.style()).clone();
            style.text_styles.get_mut(&egui::TextStyle::Monospace).unwrap().size = self.font_size;
            ctx.set_style(style);

            egui::ScrollArea::vertical()
            .stick_to_bottom(false)
            .show(ui, |ui| {
                ui.add_sized(
                    ui.available_size(),
                             egui::TextEdit::multiline(&mut self.content)
                             .font(egui::TextStyle::Monospace)
                             .code_editor()
                             .lock_focus(true)
                             .desired_width(f32::INFINITY),
                );
            });
        });
    }
}

impl RustTextEditor {
    fn new_file(&mut self) {
        self.content.clear();
        self.current_file = None;
    }

    fn open_file(&mut self) {
        if let Some(path) = rfd::FileDialog::new().pick_file() {
            if let Ok(data) = fs::read_to_string(&path) {
                self.content = data;
                self.current_file = Some(path);
            }
        }
    }

    fn save_file(&mut self) {
        if let Some(path) = &self.current_file {
            let _ = fs::write(path, &self.content);
        } else {
            self.save_as();
        }
    }

    fn save_as(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .set_file_name("file.txt")
            .save_file()
            {
                if let Ok(_) = fs::write(&path, &self.content) {
                    self.current_file = Some(path);
                }
            }
    }
}
