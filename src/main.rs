use eframe::egui;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
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

/// Owns the number vector that's manipulated by the sorting algorithm
struct AlgoVisualizer {
    numbers: Arc<Mutex<datatypes::NumberVec>>,
    stop_flag: Arc<AtomicBool>,
    thread: Option<thread::JoinHandle<()>>,
}

impl Default for AlgoVisualizer {
    fn default() -> Self {
        Self {
            numbers: Arc::new(Mutex::new(datatypes::NumberVec::new(
                (1..=25).rev().collect(),
            ))),
            stop_flag: Arc::new(AtomicBool::new(false)),
            thread: None,
        }
    }
}

struct VisuApp {
    visualizers: Vec<AlgoVisualizer>,
    animation_delay_ms: Arc<AtomicU8>,
}

impl VisuApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self {
            visualizers: vec![AlgoVisualizer::default(), AlgoVisualizer::default()],
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
        egui::TopBottomPanel::top("title_panel").show(ctx, |ui| {
            ui.heading("Welcome to VISU!");
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                // TODO: LOOP THIS!
                ui.allocate_ui_with_layout(
                    egui::vec2(350., 250.),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        let visualizer: &mut AlgoVisualizer = &mut self.visualizers[0];
                        ui.vertical(|ui| {
                            if ui.add(egui::Button::new("Shuffle numbers")).clicked() {
                                if let Some(handle) = visualizer.thread.take() {
                                    let flag = Arc::clone(&visualizer.stop_flag);
                                    flag.store(true, Ordering::Relaxed);
                                    handle.join().unwrap();
                                }
                                let numbers = Arc::clone(&visualizer.numbers);
                                visualizer.thread =
                                    Some(thread::spawn(move || algos::shuffle(numbers)));
                            } else if ui.add(egui::Button::new("Bubble Sort")).clicked() {
                                if let Some(handle) = visualizer.thread.take() {
                                    let flag = Arc::clone(&visualizer.stop_flag);
                                    flag.store(true, Ordering::Relaxed);
                                    handle.join().unwrap();
                                }
                                let (flag, numbers, delay, context) = (
                                    Arc::clone(&visualizer.stop_flag),
                                    Arc::clone(&visualizer.numbers),
                                    Arc::clone(&self.animation_delay_ms),
                                    ctx.clone(),
                                );
                                flag.store(false, Ordering::Relaxed);
                                visualizer.thread = Some(thread::spawn(move || {
                                    algos::bubblesort(numbers, delay, &context, flag)
                                }));
                            } else if ui.add(egui::Button::new("Quick Sort")).clicked() {
                                if let Some(handle) = visualizer.thread.take() {
                                    let flag = Arc::clone(&visualizer.stop_flag);
                                    flag.store(true, Ordering::Relaxed);
                                    handle.join().unwrap();
                                }
                                let (flag, numbers, highest_index, delay, context) = (
                                    Arc::clone(&visualizer.stop_flag),
                                    Arc::clone(&visualizer.numbers),
                                    Arc::clone(&visualizer.numbers).lock().unwrap().values.len()
                                        - 1,
                                    Arc::clone(&self.animation_delay_ms),
                                    ctx.clone(),
                                );
                                flag.store(false, Ordering::Relaxed);
                                visualizer.thread = Some(thread::spawn(move || {
                                    algos::quicksort(
                                        numbers,
                                        0,
                                        highest_index,
                                        &delay,
                                        &context,
                                        &flag,
                                    )
                                }));
                            }
                        });
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::BOTTOM), |ui| {
                            let numbers = Arc::clone(&visualizer.numbers);
                            for num in numbers.lock().unwrap().values.iter() {
                                let col = match num.highlight {
                                    datatypes::Highlight::None => num.color,
                                    datatypes::Highlight::Primary => egui::Color32::KHAKI,
                                    datatypes::Highlight::Secondary => {
                                        num.color.linear_multiply(0.5)
                                    }
                                };
                                ui.add(&mut VerticalBarWidget::new(num.value, col));
                            }
                        })
                    },
                );
                ui.add_space(20.);
                ui.allocate_ui_with_layout(
                    egui::vec2(350., 250.),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        let visualizer: &mut AlgoVisualizer = &mut self.visualizers[1];
                        ui.vertical(|ui| {
                            if ui.add(egui::Button::new("Shuffle numbers")).clicked() {
                                if let Some(handle) = visualizer.thread.take() {
                                    let flag = Arc::clone(&visualizer.stop_flag);
                                    flag.store(true, Ordering::Relaxed);
                                    handle.join().unwrap();
                                }
                                let numbers = Arc::clone(&visualizer.numbers);
                                visualizer.thread =
                                    Some(thread::spawn(move || algos::shuffle(numbers)));
                            } else if ui.add(egui::Button::new("Bubble Sort")).clicked() {
                                if let Some(handle) = visualizer.thread.take() {
                                    let flag = Arc::clone(&visualizer.stop_flag);
                                    flag.store(true, Ordering::Relaxed);
                                    handle.join().unwrap();
                                }
                                let (flag, numbers, delay, context) = (
                                    Arc::clone(&visualizer.stop_flag),
                                    Arc::clone(&visualizer.numbers),
                                    Arc::clone(&self.animation_delay_ms),
                                    ctx.clone(),
                                );
                                flag.store(false, Ordering::Relaxed);
                                visualizer.thread = Some(thread::spawn(move || {
                                    algos::bubblesort(numbers, delay, &context, flag)
                                }));
                            } else if ui.add(egui::Button::new("Quick Sort")).clicked() {
                                if let Some(handle) = visualizer.thread.take() {
                                    let flag = Arc::clone(&visualizer.stop_flag);
                                    flag.store(true, Ordering::Relaxed);
                                    handle.join().unwrap();
                                }
                                let (flag, numbers, highest_index, delay, context) = (
                                    Arc::clone(&visualizer.stop_flag),
                                    Arc::clone(&visualizer.numbers),
                                    Arc::clone(&visualizer.numbers).lock().unwrap().values.len()
                                        - 1,
                                    Arc::clone(&self.animation_delay_ms),
                                    ctx.clone(),
                                );
                                flag.store(false, Ordering::Relaxed);
                                visualizer.thread = Some(thread::spawn(move || {
                                    algos::quicksort(
                                        numbers,
                                        0,
                                        highest_index,
                                        &delay,
                                        &context,
                                        &flag,
                                    )
                                }));
                            }
                        });
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::BOTTOM), |ui| {
                            let numbers = Arc::clone(&visualizer.numbers);
                            for num in numbers.lock().unwrap().values.iter() {
                                let col = match num.highlight {
                                    datatypes::Highlight::None => num.color,
                                    datatypes::Highlight::Primary => egui::Color32::KHAKI,
                                    datatypes::Highlight::Secondary => {
                                        num.color.linear_multiply(0.5)
                                    }
                                };
                                ui.add(&mut VerticalBarWidget::new(num.value, col));
                            }
                        })
                    },
                );
                // Animation speed slider
                let animation_delay = Arc::clone(&self.animation_delay_ms);
                let mut speed = delay_to_speed(&animation_delay.load(Ordering::Acquire));
                ui.add(egui::Slider::new(&mut speed, 1..=10).text("Animation speed"));
                animation_delay.store(speed_to_delay(&speed), Ordering::Release);
            });
        });
    }
}

fn main() {
    let native_options = eframe::NativeOptions {
        default_theme: eframe::Theme::Light,
        initial_window_size: Some(egui::vec2(750., 600.)),
        ..Default::default()
    };
    eframe::run_native(
        "visu",
        native_options,
        Box::new(|cc| Box::new(VisuApp::new(cc))),
    );
}
