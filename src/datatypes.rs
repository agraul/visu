use eframe::egui;

#[derive(Debug)]
pub enum Highlight {
    None,
    Primary,
    Secondary,
}

#[derive(Debug)]
pub struct Number {
    pub value: u8,
    pub color: egui::Color32,
    pub highlight: Highlight,
}

impl Number {
    pub fn new(value: u8, color_multiplier: u8) -> Self {
        Self {
            value,
            color: Number::calculate_color(color_multiplier),
            highlight: Highlight::None,
        }
    }
    pub fn color(&mut self, color_multiplier: u8) {
        self.color = Number::calculate_color(color_multiplier);
    }

    fn calculate_color(color_multiplier: u8) -> egui::Color32 {
        let mut red_value = 255 - 5 * color_multiplier as i16;
        if red_value < 0 {
            red_value = 0
        }
        egui::Color32::from_rgb(red_value as u8, 100, 100)
    }
}

impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}
impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

pub struct NumberVec {
    pub values: Vec<Number>,
}

impl NumberVec {
    pub fn new(values: Vec<u8>) -> Self {
        Self {
            values: values
                .iter()
                .enumerate()
                .map(|(i, v)| Number::new(*v, i as u8))
                .collect(),
        }
    }
    pub fn is_sorted(&self) -> bool {
        for window in self.values.windows(2) {
            if window[0] > window[1] {
                return false;
            }
        }
        true
    }
    pub fn remove_all_highlights(&mut self) {
        for mut num in self.values.iter_mut() {
            num.highlight = Highlight::None;
        }
    }
    pub fn add_highlight(
        &mut self,
        idx: usize,
        highlight: Highlight,
    ) -> Result<(), IndexOutOfBoundsError> {
        if idx >= self.values.len() {
            return Err(IndexOutOfBoundsError {
                requested: idx,
                maximum: self.values.len() - 1,
            });
        }
        self.values[idx].highlight = highlight;
        Ok(())
    }
}

#[derive(Debug)]
pub struct IndexOutOfBoundsError {
    requested: usize,
    maximum: usize,
}
impl std::fmt::Display for IndexOutOfBoundsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Index {} higher than {}", self.requested, self.maximum)
    }
}
impl std::error::Error for IndexOutOfBoundsError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numbervec_is_sorted() {
        let ascending = NumberVec::new((1..=100).collect());
        assert!(ascending.is_sorted());

        let descending = NumberVec::new((1..=100).rev().collect());
        assert!(!descending.is_sorted());
    }
}
