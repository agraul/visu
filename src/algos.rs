use eframe::egui;
use rand::prelude::*;
use std::sync::{Arc, Mutex};
use std::{thread, time};

use crate::datatypes;

pub fn bubble(
    numbers: Arc<Mutex<datatypes::NumberVec>>,
    sleep_for: time::Duration,
    ctx: &egui::Context,
) {
    let nums = numbers.lock().unwrap();
    let length = nums.values.len();
    drop(nums);
    for n in 0..length {
        for i in 0..length - n - 1 {
            let j = i + 1;
            let mut nums = numbers.lock().unwrap();
            if nums.values[i].value > nums.values[j].value {
                nums.values.swap(i, j);
                nums.highlight_at = Some(j);
            } else {
                nums.highlight_at = Some(i);
            }

            drop(nums);
            ctx.request_repaint();
            thread::sleep(sleep_for);
        }
    }
}

pub fn shuffle(numbers: Arc<Mutex<datatypes::NumberVec>>) {
    let mut rng = thread_rng();
    let mut nums = numbers.lock().unwrap();
    nums.highlight_at = None;
    nums.values.shuffle(&mut rng);
}
