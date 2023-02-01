use eframe::egui;
use rand::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::sync::{Arc, Mutex};
use std::{thread, time};

use crate::datatypes;

/// Sort `numbers` using bubble sort.
///
/// # Arguments
///
/// - numbers: `datatypes::NumberVec` to sort
/// - animation_delay: time to sleep in ms after each comparison
/// - ctx: egui::Context to request repainting after each comparison
/// - stop_flag: Set to `true` from another thread to abort
pub fn bubblesort(
    numbers: Arc<Mutex<datatypes::NumberVec>>,
    animation_delay: Arc<AtomicU8>,
    ctx: &egui::Context,
    stop_flag: Arc<AtomicBool>,
) {
    let nums = numbers.lock().unwrap();
    let length = nums.values.len();
    drop(nums);
    for n in 0..length {
        for i in 0..length - n - 1 {
            if stop_flag.load(Ordering::Relaxed) {
                return;
            }
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
                animation_delay.load(Ordering::Relaxed).into(),
            ));
        }
    }
    let mut nums = numbers.lock().unwrap();
    nums.highlight_at = None;
    ctx.request_repaint();
}

pub fn quicksort(
    numbers: Arc<Mutex<datatypes::NumberVec>>,
    low_idx: usize,
    high_idx: usize,
    animation_delay: &Arc<AtomicU8>,
    ctx: &egui::Context,
    stop_flag: &Arc<AtomicBool>,
) {
    if low_idx > high_idx || stop_flag.load(Ordering::Relaxed) {
        return;
    }

    let pivot_idx = qs_partition(
        Arc::clone(&numbers),
        low_idx,
        high_idx,
        animation_delay,
        ctx,
        stop_flag,
    );
    // highlight pivot?
    println!("{pivot_idx:?}");

    if pivot_idx == 0 {
        quicksort(
            Arc::clone(&numbers),
            low_idx,
            0,
            animation_delay,
            ctx,
            stop_flag,
        );
        quicksort(
            Arc::clone(&numbers),
            pivot_idx + 1,
            high_idx,
            animation_delay,
            ctx,
            stop_flag,
        );
    } else {
        quicksort(
            Arc::clone(&numbers),
            low_idx,
            pivot_idx - 1,
            animation_delay,
            ctx,
            stop_flag,
        );
        quicksort(
            Arc::clone(&numbers),
            pivot_idx + 1,
            high_idx,
            animation_delay,
            ctx,
            stop_flag,
        );
    }
}

fn qs_partition(
    numbers: Arc<Mutex<datatypes::NumberVec>>,
    low_idx: usize,
    high_idx: usize,
    animation_delay: &Arc<AtomicU8>,
    ctx: &egui::Context,
    stop_flag: &Arc<AtomicBool>,
) -> usize {
    let mut nums = numbers.lock().unwrap();
    let pivot_value = nums.values[high_idx].value;
    println!("pivot value {pivot_value:?} (index {high_idx:?})");
    // drop(nums);

    // when low_idx == 0, i becomes -1 temporarily
    let mut i = low_idx as i64 - 1;

    for j in low_idx..high_idx {
        // if stop_flag.load(Ordering::Relaxed) {
        //     return high_idx;
        // }

        // let mut nums = numbers.lock().unwrap();
        let looking_at = nums.values[j].value;
        println!("comparing {looking_at:?} to pivot={pivot_value:?}");
        if nums.values[j].value <= pivot_value {
            i += 1;
            // i should never be negative at this point
            nums.values.swap(i as usize, j)
        }
        // drop(nums);
        ctx.request_repaint();
        thread::sleep(time::Duration::from_millis(
            animation_delay.load(Ordering::Relaxed).into(),
        ));
    }

    // let mut nums = numbers.lock().unwrap();
    i += 1;
    // i should never be negative at this point
    nums.values.swap(i as usize, high_idx);
    ctx.request_repaint();
    // drop(nums);
    i as usize
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

/// Only exists to test the implementation easily
fn _basic_quicksort(numbers: &mut datatypes::NumberVec, lo: usize, hi: usize) {
    if lo >= hi {
        return;
    }

    let p = _basic_qs_partition(numbers, lo, hi);
    if p == 0 {
        _basic_quicksort(numbers, lo, p);
        _basic_quicksort(numbers, p + 1, hi);
    } else {
        _basic_quicksort(numbers, lo, p - 1);
        _basic_quicksort(numbers, p + 1, hi);
    }
}

/// Only exists to test the implementation easily
fn _basic_qs_partition(numbers: &mut datatypes::NumberVec, lo: usize, hi: usize) -> usize {
    let pivot = numbers.values[hi].value;
    println!("pivot value {pivot:?} (index {hi:?})");
    // when lo == 0, i becomes -1 temporarily
    let mut i = lo as i64 - 1;
    for j in lo..hi {
        let looking_at = numbers.values[j].value;
        if numbers.values[j].value <= pivot {
            println!("Incrementing i from {i:?}, {looking_at:?} <= {pivot:?}");
            i += 1;
            // i should never be negative at this point
            numbers.values.swap(i as usize, j)
        }
    }
    i += 1;
    // i should never be negative at this point
    numbers.values.swap(i as usize, hi);
    i as usize
}

// Tests below

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quicksort_shuffled_input() {
        let mut numbers = datatypes::NumberVec::new((1..=100).collect());
        let mut rng = thread_rng();
        numbers.values.shuffle(&mut rng);

        let length = numbers.values.len();
        _basic_quicksort(&mut numbers, 0, length - 1);
        assert!(numbers.is_sorted());
    }
    #[test]
    fn quicksort_sorted_input() {
        let mut numbers = datatypes::NumberVec::new((1..=100).collect());

        let length = numbers.values.len();
        _basic_quicksort(&mut numbers, 0, length - 1);
        assert!(numbers.is_sorted());
    }
    #[test]
    fn quicksort_sorted_reversed_input() {
        let mut numbers = datatypes::NumberVec::new((1..=100).rev().collect());

        let length = numbers.values.len();
        _basic_quicksort(&mut numbers, 0, length - 1);
        println!(
            "{:?}",
            numbers.values.iter().map(|n| n.value).collect::<Vec<u8>>()
        );
        assert!(numbers.is_sorted());
    }
}
