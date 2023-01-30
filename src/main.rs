use eframe::egui;
use rand::prelude::*;
use std::sync::mpsc;
use std::{thread, time};

struct VerticalBarWidget {
    height: u8,
    width: u8,
}

impl VerticalBarWidget {
    fn new(height: u8) -> Self {
        Self {
            height: height * 10,
            width: 15,
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
        painter.rect_filled(rect, egui::Rounding::none(), egui::Color32::RED);
        response
    }
}

struct VisuApp {
    numbers: Vec<u8>,
    rx: mpsc::Receiver<Vec<u8>>,
    tx: mpsc::Sender<Vec<u8>>,
}
impl VisuApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let (tx, rx) = mpsc::channel();
        Self {
            numbers: (1..=25).rev().collect(),
            tx,
            rx,
        }
    }
}

fn bubble_sort(mut numbers: Vec<u8>, delay: time::Duration, tx: mpsc::Sender<Vec<u8>>) {
    for n in 0..numbers.len() {
        for i in 0..numbers.len() - n - 1 {
            let j = i + 1;
            if numbers[i] > numbers[j] {
                numbers.swap(i, j);
                tx.send(numbers.clone()).unwrap();
                // thread::sleep(delay);
            }
        }
    }
}

impl eframe::App for VisuApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Ok(numbers) = self.rx.try_recv() {
            self.numbers = numbers;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Welcome to visu");
            if ui.add(egui::Button::new("Shuffle numbers")).clicked() {
                let nums = &mut self.numbers;
                let mut rng = thread_rng();
                nums.shuffle(&mut rng);
            }
            if ui.add(egui::Button::new("Start Bubble Sort")).clicked() {
                let numbers = self.numbers.clone();
                let sender = self.tx.clone();
                thread::spawn(move || bubble_sort(numbers, time::Duration::from_millis(50), sender));
            }
            ui.with_layout(egui::Layout::left_to_right(egui::Align::BOTTOM), |ui| {
                for n in &self.numbers {
                    ui.add(&mut VerticalBarWidget::new(*n));
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
