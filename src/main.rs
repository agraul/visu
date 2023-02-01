use eframe::egui;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

mod algos;
mod datatypes;

struct VerticalBarWidget {
    height: u8,
    width: u8,
    color: egui::Color32,
}

impl VerticalBarWidget {
    fn new(height: u8, color: egui::Color32) -> Self {
        Self {
            height: height * 10,
            width: 15,
            color,
        }
    }
}

impl egui::Widget for &mut VerticalBarWidget {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let size = egui::vec2(self.width as f32, self.height as f32);
        let (rect, response) = ui.allocate_at_least(
            size,
            egui::Sense {
                click: false,
                drag: false,
                focusable: false,
            },
        );
        let painter = ui.painter();
        painter.rect_filled(rect, egui::Rounding::none(), self.color);
        response
    }
}

struct VisuApp {
    numbers: Arc<Mutex<datatypes::NumberVec>>,
    animation_delay_ms: Arc<AtomicU8>,
}

impl VisuApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let vals = (1..=25).rev().collect();
        Self {
            numbers: Arc::new(Mutex::new(datatypes::NumberVec::new(vals))),
            animation_delay_ms: Arc::new(AtomicU8::new(10)),
        }
    }
}

fn delay_to_speed(dt: &u8) -> u8 {
    11 - dt / 10
}

fn speed_to_delay(v: &u8) -> u8 {
    110 - 10 * v
}

impl eframe::App for VisuApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("control_panel").show(ctx, |ui| {
            ui.heading("Welcome to visu");
            if ui.add(egui::Button::new("Shuffle numbers")).clicked() {
                let nums = Arc::clone(&self.numbers);
                thread::spawn(move || algos::shuffle(nums));
            }
            if ui.add(egui::Button::new("Start Bubble Sort")).clicked() {
                let nums = Arc::clone(&self.numbers);
                let delay = Arc::clone(&self.animation_delay_ms);
                let context = ctx.clone();
                thread::spawn(move || algos::bubblesort(nums, delay, &context));
            }
            // Animation speed slider
            let animation_delay = Arc::clone(&self.animation_delay_ms);
            let mut speed = delay_to_speed(&animation_delay.load(Ordering::Acquire));
            ui.add(egui::Slider::new(&mut speed, 1..=10).text("Animation speed"));
            animation_delay.store(speed_to_delay(&speed), Ordering::Release);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::BOTTOM), |ui| {
                let nums_arc = Arc::clone(&self.numbers);
                // .len() above automatically dereferences, it does not exist on MutexGuard
                // That does not happen automatically with `for` -> `.iter()` is needed
                let mut nums = nums_arc.lock().unwrap();
                let highlight_at = nums.highlight_at;
                for (i, n) in nums.values.iter_mut().enumerate() {
                    if let Some(h) = highlight_at {
                        if h == i {
                            n.color(egui::Color32::YELLOW);
                        } else {
                            n.color(datatypes::Number::default().color);
                        }
                    } else {
                        n.color(datatypes::Number::default().color);
                    }
                    ui.add(&mut VerticalBarWidget::new(n.value, n.color));
                }
            });
        });
    }
}

fn main() {
    let native_options = eframe::NativeOptions {
        default_theme: eframe::Theme::Light,
        ..Default::default()
    };
    eframe::run_native(
        "visu",
        native_options,
        Box::new(|cc| Box::new(VisuApp::new(cc))),
    );
}
