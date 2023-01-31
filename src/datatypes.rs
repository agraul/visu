use eframe::egui;
pub struct Number {
    pub value: u8,
    pub color: egui::Color32,
}

impl Number {
    pub fn new(value: u8) -> Self {
        Self {
            value,
            ..Default::default()
        }
    }
    pub fn color(&mut self, color: egui::Color32) {
        self.color = color;
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
            values: values.iter().map(|v| Number::new(*v)).collect(),
            highlight_at: None,
        }
    }
}
