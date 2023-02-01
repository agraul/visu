use eframe::egui;
pub struct Number {
    pub value: u8,
    pub color: egui::Color32,
}

impl Number {
    pub fn new(value: u8, color_multiplier: u8) -> Self {
        Self {
            value,
            color: Number::calculate_color(color_multiplier),
        }
    }
    pub fn color(&mut self, color_multiplier: u8) {
        self.color = Number::calculate_color(color_multiplier);
    }

    fn calculate_color(color_multiplier: u8) -> egui::Color32 {
        egui::Color32::from_rgb(255 - 5* color_multiplier, 100, 100)
    }
}

impl Default for Number {
    fn default() -> Self {
        Self {
            value: 0,
            color: egui::Color32::RED,
        }
    }
}

pub struct NumberVec {
    pub values: Vec<Number>,
    pub highlight_at: Option<usize>,
}

impl NumberVec {
    pub fn new(values: Vec<u8>) -> Self {
        Self {
            values: values
                .iter()
                .enumerate()
                .map(|(i, v)| Number::new(*v, i as u8))
                .collect(),
            highlight_at: None,
        }
    }
}
