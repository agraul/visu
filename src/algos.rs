use eframe::egui;
use rand::prelude::*;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::{Arc, Mutex};
use std::{thread, time};

use crate::datatypes;

pub fn bubblesort(
    numbers: Arc<Mutex<datatypes::NumberVec>>,
    animation_delay: Arc<AtomicU8>,
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
            thread::sleep(time::Duration::from_millis(
                animation_delay.load(Ordering::Acquire).into(),
            ));
        }
    }
    let mut nums = numbers.lock().unwrap();
    nums.highlight_at = None;
}

pub fn shuffle(numbers: Arc<Mutex<datatypes::NumberVec>>) {
    let mut rng = thread_rng();
    let mut nums = numbers.lock().unwrap();
    nums.highlight_at = None;
    nums.values.shuffle(&mut rng);
    for (i, n) in nums.values.iter_mut().enumerate() {
        n.color(i as u8);
    }
}
