//! Signal Processing: 15 calculations through the Rust library (the same engine as the desktop app, the
//! command line and the browser). Run from the rust/ folder:
//!     cargo run --release --example readme -p signal-processing-ai999x
//! Inputs are text, read like a form field ("2 m", "0.5", "x^3 - 3x"); a JSON list stays a list.

use serde_json::{json, Value};

/// One calculation: the request in, its outcome and every value with its unit out.
fn show(request: Value) {
    let answer: Value = serde_json::from_str(&signal_processing_ai999x::solve_json(&request.to_string())).unwrap();
    println!("{}", answer["outcome"].as_str().unwrap_or(""));
    for v in answer["values"].as_array().into_iter().flatten() {
        let unit = v[2].as_str().unwrap_or("");
        let unit = if unit.is_empty() || unit == "1" { String::new() } else { format!(" {unit}") };
        println!("{} = {}{}", v[0].as_str().unwrap_or(""), v[1], unit);
    }
}

fn main() {
    show(json!({"key": "butterworth_magnitude", "inputs": {"N": "4", "fc": "1 kHz", "f": "2 kHz"}}));
    println!();
    show(json!({"key": "dish_antenna_gain", "inputs": {"D": "3 m", "f": "10 GHz", "eta": "0.55"}}));
    println!();
    show(json!({"key": "shannon_min_snr", "inputs": {"B": "2 MHz", "C": "10000000"}}));
    println!();
    show(json!({"key": "circular_convolution", "inputs": {"x": [1, 2, 3, 4], "h": [1, 0, 0, 1]}}));
    println!();
    show(json!({"key": "dft_spectrum", "inputs": {"x": [1, 2, 3, 4]}}));
    println!();
    show(json!({"key": "amplitude_ratio_to_db", "inputs": {"ratio": "50"}}));
    println!();
    show(json!({"key": "qpsk_error_rates", "inputs": {"ebn0_db": "6"}}));
    println!();
    show(json!({"key": "bilinear_rc_lowpass", "inputs": {"fs": "8 kHz", "fc": "1 kHz"}}));
    println!();
    show(json!({"key": "butterworth_design", "inputs": {"btype": "lowpass", "N": "2", "fs": "8 kHz", "fc": "1 kHz"}}));
    println!();
    show(json!({"key": "analytic_envelope", "inputs": {"x": [1, 0, -1, 0]}}));
    println!();
    show(json!({"key": "am_modulation_index", "inputs": {"Vmax": "10 V", "Vmin": "2 V"}}));
    println!();
    show(json!({"key": "free_space_path_loss", "inputs": {"d": "100 m", "f": "2.4 GHz"}}));
    println!();
    show(json!({"key": "radar_doppler", "inputs": {"v": "30 m/s", "f": "10 GHz"}}));
    println!();
    show(json!({"key": "alias_frequency", "inputs": {"fs": "10 kHz", "f": "7 kHz"}}));
    println!();
    show(json!({"key": "coax_impedance", "inputs": {"D": "3.5 mm", "d": "1 mm", "er": "2.25"}}));
}
