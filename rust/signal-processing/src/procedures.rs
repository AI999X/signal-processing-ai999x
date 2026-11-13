//! signal_processing: procedures translated from the package's Python by tools/py2rust.py (do not edit; regenerate).
//! Source: models/signal_processing/capabilities/procedures.py. Each keeps Python's number rules through
//! ax_engine::py and is checked against the package by tools/rust_accuracy.py before the build is listed.
#![allow(clippy::all, non_snake_case, unused_parens, unused_braces, unused_mut, unused_variables, unreachable_code)]

use ax_engine::py::{self, Py, PyErr, R};
use ax_engine::Val;
use serde_json::Value;
use std::collections::HashMap;

/// The procedures this crate implements (tools/export_rust_pack.py reads this list).
pub const NATIVE: &[&str] = &[
    "alias_frequency",
    "am_modulation_index",
    "am_power",
    "amplitude_ratio_to_db",
    "analytic_envelope",
    "bilinear_rc_lowpass",
    "bin_frequency",
    "bpsk_ber",
    "butterworth_design",
    "butterworth_magnitude",
    "butterworth_order_analog",
    "butterworth_order_digital",
    "chebyshev1_order_analog",
    "circular_convolution",
    "coax_impedance",
    "cross_correlation",
    "db_to_amplitude_ratio",
    "db_to_power_ratio",
    "dbm_power_sum",
    "dbm_to_watts",
    "dc_gain",
    "dft_bin",
    "dft_spectrum",
    "difference_equation_z",
    "dish_antenna_gain",
    "exponential_pulse_spectrum",
    "fir_filter_output",
    "fir_window_design",
    "fm_spectrum",
    "fourier_series_harmonic",
    "free_space_path_loss",
    "frequency_from_period",
    "frequency_resolution",
    "frequency_response",
    "friis_received_power",
    "group_delay",
    "haar_dwt",
    "haar_idwt",
    "iir_filter_output",
    "impulse_response",
    "input_impedance",
    "inverse_dft",
    "inverse_z_transform",
    "linear_convolution",
    "moving_average",
    "mpsk_symbol_error",
    "mqam_symbol_error",
    "noise_figure_cascade",
    "noncoherent_ber",
    "notch_filter_design",
    "nyquist_rate",
    "period_from_frequency",
    "periodogram",
    "poles_zeros",
    "power_ratio_to_db",
    "qpsk_error_rates",
    "quantization_snr",
    "quarter_wave_transformer",
    "radar_doppler",
    "radar_max_range",
    "radar_received_power",
    "rect_pulse_spectrum",
    "reflection_coefficient",
    "reflection_from_vswr",
    "second_order_poles",
    "shannon_capacity",
    "shannon_min_snr",
    "signal_statistics",
    "sine_rms",
    "snr_from_powers",
    "step_response",
    "thd",
    "vswr_from_reflection",
    "watts_to_dbm",
    "window_coefficients",
    "window_mainlobe_width",
    "z_transform_sequence",
];

type Out = Result<(Vec<(String, Value)>, f64), String>;

/// The run's result as the engine's answer: a dict of outputs and the residual of the package's own check; a
/// ValueError is the refusal's reason; any other error is an internal error (None).
fn finish(r: R, residual: impl Fn(&Py, &Py) -> f64, v: &HashMap<String, Val>) -> Option<Out> {
    let o = match r {
        Ok(o) => o,
        Err(PyErr::Value(m)) => return Some(Err(m)),
        Err(e) => {
            if std::env::var_os("AX_DEBUG").is_some() {        // the error behind an internal error, for diagnosis
                eprintln!("internal error: {e:?}");
            }
            return None;
        }
    };
    let Py::Dict(d) = &o else { return None };
    let outs: Vec<(String, Value)> = d.borrow().iter().map(|(k, x)| (py::to_str(k), py::to_json(x))).collect();
    let i = Py::dict(v.iter().map(|(k, x)| (Py::str(k), py::from_val(x))).collect());
    let res = residual(&i, &o);
    if std::env::var_os("AX_DEBUG").is_some() && !(res <= 1e-9) {
        eprintln!("check failed: residual {res}");
    }
    Some(Ok((outs, res)))
}

/// The engine's entry point (ax_engine::pack::NativeFn): None for a key that is not one of these procedures.
pub fn native(key: &str, v: &HashMap<String, Val>) -> Option<Out> {
    match key {
        "alias_frequency" => {
            let r = f_alias_frequency(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fs") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_12(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "am_modulation_index" => {
            let r = f_am_modulation_index(match v.get("Vmax") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vmin") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_55(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "am_power" => {
            let r = f_am_power(match v.get("Pc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_93(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "amplitude_ratio_to_db" => {
            let r = f_amplitude_ratio_to_db(match v.get("ratio") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_128(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "analytic_envelope" => {
            let r = f_analytic_envelope(match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_199(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bilinear_rc_lowpass" => {
            let r = f_bilinear_rc_lowpass(match v.get("fc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fs") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_236(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bin_frequency" => {
            let r = f_bin_frequency(match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("N") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_286(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bpsk_ber" => {
            let r = f_bpsk_ber(match v.get("ebn0_db") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_318(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "butterworth_design" => {
            let r = f_butterworth_design(match v.get("N") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("btype") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_390(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "butterworth_magnitude" => {
            let r = f_butterworth_magnitude(match v.get("N") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_446(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "butterworth_order_analog" => {
            let r = f_butterworth_order_analog(match v.get("fp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fstop") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("As") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_491(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "butterworth_order_digital" => {
            let r = f_butterworth_order_digital(match v.get("fp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fstop") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("As") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_539(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "chebyshev1_order_analog" => {
            let r = f_chebyshev1_order_analog(match v.get("fp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fstop") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Rp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("As") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_584(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "circular_convolution" => {
            let r = f_circular_convolution(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_631(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "coax_impedance" => {
            let r = f_coax_impedance(match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("er") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_671(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cross_correlation" => {
            let r = f_cross_correlation(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_722(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "db_to_amplitude_ratio" => {
            let r = f_db_to_amplitude_ratio(match v.get("dB") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_754(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "db_to_power_ratio" => {
            let r = f_db_to_power_ratio(match v.get("dB") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_786(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "dbm_power_sum" => {
            let r = f_dbm_power_sum(match v.get("levels") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_829(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "dbm_to_watts" => {
            let r = f_dbm_to_watts(match v.get("dBm") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_861(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "dc_gain" => {
            let r = f_dc_gain(match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_909(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "dft_bin" => {
            let r = f_dft_bin(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_976(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "dft_spectrum" => {
            let r = f_dft_spectrum(match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1047(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "difference_equation_z" => {
            let r = f_difference_equation_z(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("initial_conditions") { Some(x) => py::from_val(x), None => { return None } }, match v.get("input_amplitude") { Some(x) => py::from_val(x), None => { return None } }, match v.get("input_ratio") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(8i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_de(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "dish_antenna_gain" => {
            let r = f_dish_antenna_gain(match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eta") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1314(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "exponential_pulse_spectrum" => {
            let r = f_exponential_pulse_spectrum(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tau") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1355(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "fir_filter_output" => {
            let r = f_fir_filter_output(match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1407(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "fir_window_design" => {
            let r = f_fir_window_design(match v.get("N") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("window") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1477(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "fm_spectrum" => {
            let r = f_fm_spectrum(match v.get("df") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(1i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1533(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "fourier_series_harmonic" => {
            let r = f_fourier_series_harmonic(match v.get("waveform") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1593(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "free_space_path_loss" => {
            let r = f_free_space_path_loss(match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1631(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "frequency_from_period" => {
            let r = f_frequency_from_period(match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1666(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "frequency_resolution" => {
            let r = f_frequency_resolution(match v.get("fs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("N") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1717(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "frequency_response" => {
            let r = f_frequency_response(match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1792(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "friis_received_power" => {
            let r = f_friis_received_power(match v.get("Pt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Gt_db") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Gr_db") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1833(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "group_delay" => {
            let r = f_group_delay(match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1908(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "haar_dwt" => {
            let r = f_haar_dwt(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("levels") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(1i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1969(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "haar_idwt" => {
            let r = f_haar_idwt(match v.get("cA") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cD") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2016(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "iir_filter_output" => {
            let r = f_iir_filter_output(match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2068(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "impulse_response" => {
            let r = f_impulse_response(match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2135(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "input_impedance" => {
            let r = f_input_impedance(match v.get("RL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Z0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("length") { Some(x) => py::from_val(x), None => { return None } }, match v.get("XL") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2183(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "inverse_dft" => {
            let r = f_inverse_dft(match v.get("re_") { Some(x) => py::from_val(x), None => { return None } }, match v.get("im") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2253(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "inverse_z_transform" => {
            let r = f_inverse_z_transform(match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(8i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_iz(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "linear_convolution" => {
            let r = f_linear_convolution(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2501(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "moving_average" => {
            let r = f_moving_average(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("M") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2562(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "mpsk_symbol_error" => {
            let r = f_mpsk_symbol_error(match v.get("M") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ebn0_db") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2596(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "mqam_symbol_error" => {
            let r = f_mqam_symbol_error(match v.get("M") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ebn0_db") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2632(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "noise_figure_cascade" => {
            let r = f_noise_figure_cascade(match v.get("nf_db") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gain_db") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2683(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "noncoherent_ber" => {
            let r = f_noncoherent_ber(match v.get("ebn0_db") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2715(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "notch_filter_design" => {
            let r = f_notch_filter_design(match v.get("f0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fs") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2753(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nyquist_rate" => {
            let r = f_nyquist_rate(match v.get("fmax") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2788(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "period_from_frequency" => {
            let r = f_period_from_frequency(match v.get("f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2823(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "periodogram" => {
            let r = f_periodogram(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fs") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2900(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "poles_zeros" => {
            let r = f_poles_zeros(match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3003(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "power_ratio_to_db" => {
            let r = f_power_ratio_to_db(match v.get("ratio") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3038(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "qpsk_error_rates" => {
            let r = f_qpsk_error_rates(match v.get("ebn0_db") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3070(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "quantization_snr" => {
            let r = f_quantization_snr(match v.get("bits") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3115(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "quarter_wave_transformer" => {
            let r = f_quarter_wave_transformer(match v.get("RL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Z0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("vf") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3155(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "radar_doppler" => {
            let r = f_radar_doppler(match v.get("v") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3195(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "radar_max_range" => {
            let r = f_radar_max_range(match v.get("Pt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("G_db") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Pmin") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3233(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "radar_received_power" => {
            let r = f_radar_received_power(match v.get("Pt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("G_db") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3271(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "rect_pulse_spectrum" => {
            let r = f_rect_pulse_spectrum(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tau") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3318(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "reflection_coefficient" => {
            let r = f_reflection_coefficient(match v.get("RL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Z0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("XL") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3357(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "reflection_from_vswr" => {
            let r = f_reflection_from_vswr(match v.get("vswr") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3394(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "second_order_poles" => {
            let r = f_second_order_poles(match v.get("a1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3435(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "shannon_capacity" => {
            let r = f_shannon_capacity(match v.get("B") { Some(x) => py::from_val(x), None => { return None } }, match v.get("snr_db") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3473(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "shannon_min_snr" => {
            let r = f_shannon_min_snr(match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("B") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3511(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "signal_statistics" => {
            let r = f_signal_statistics(match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3555(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "sine_rms" => {
            let r = f_sine_rms(match v.get("Vp") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3590(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "snr_from_powers" => {
            let r = f_snr_from_powers(match v.get("Ps") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Pn") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3626(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "step_response" => {
            let r = f_step_response(match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3693(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "thd" => {
            let r = f_thd(match v.get("V") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3744(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "vswr_from_reflection" => {
            let r = f_vswr_from_reflection(match v.get("gamma") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3781(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "watts_to_dbm" => {
            let r = f_watts_to_dbm(match v.get("P") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3816(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "window_coefficients" => {
            let r = f_window_coefficients(match v.get("window") { Some(x) => py::from_val(x), None => { return None } }, match v.get("N") { Some(x) => py::from_val(x), None => { return None } }, match v.get("form") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("symmetric")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3870(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "window_mainlobe_width" => {
            let r = f_window_mainlobe_width(match v.get("window") { Some(x) => py::from_val(x), None => { return None } }, match v.get("N") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fs") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3924(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "z_transform_sequence" => {
            let r = f_z_transform_sequence(match v.get("coefficients") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ratios") { Some(x) => py::from_val(x), None => { return None } }, match v.get("powers") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4029(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        _ => None,
    }
}

#[allow(non_snake_case)]
fn c_C0() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(299792458.0)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_EPS0() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(8.8541878188e-12)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_INF() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(f64::INFINITY)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_MU0() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(1.25663706127e-06)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_T0() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(290.0)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__MAINLOBE_BINS() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("rectangular"), Py::Int(2i64)), (Py::str("hann"), Py::Int(4i64)), (Py::str("hamming"), Py::Int(4i64)), (Py::str("blackman"), Py::Int(6i64))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__PSK_M() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::list(vec![Py::str("2"), Py::str("4"), Py::str("8"), Py::str("16"), Py::str("32"), Py::str("64")])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__QAM_M() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::list(vec![Py::str("4"), Py::str("16"), Py::str("64"), Py::str("256"), Py::str("1024")])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__S2() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(0.7071067811865476)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__WAVES() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::list(vec![Py::str("square"), Py::str("triangle"), Py::str("sawtooth")])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__WINDOWS() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::list(vec![Py::str("rectangular"), Py::str("hann"), Py::str("hamming"), Py::str("blackman")])) })());
    V.with(|v| v.clone())
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__besselj(mut v_n: Py, mut v_x: Py) -> R {
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(30i64))?;
        return Ok(py::float(&ax_engine::pymp::call("besselj", &[v_n.clone(), v_x.clone()], &[])?)?);
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__butter_ord(mut v_nat: Py, mut v_Rp: Py, mut v_As: Py) -> R {
    let mut v_GPASS: Py = Py::None;
    let mut v_GSTOP: Py = Py::None;
    let t538 = (Py::tuple(vec![py::pow(&Py::Int(10i64), &py::mul(&Py::Float(0.1), &v_As.clone())?)?, py::pow(&Py::Int(10i64), &py::mul(&Py::Float(0.1), &v_Rp.clone())?)?])).items()?;
    if t538.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_GSTOP = t538[0].clone();
    v_GPASS = t538[1].clone();
    return Ok(py::int(&py::math1("ceil", &py::truediv(&py::math1("log10", &py::truediv(&py::sub(&v_GSTOP.clone(), &Py::Float(1.0))?, &py::sub(&v_GPASS.clone(), &Py::Float(1.0))?)?)?, &py::mul(&Py::Int(2i64), &py::math1("log10", &v_nat.clone())?)?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__canon_roots(mut v_roots: Py) -> R {
    let mut v_cplx: Py = Py::None;
    let mut v_real: Py = Py::None;
    v_real = py::sorted(&{ let mut out = vec![]; for t2949 in py::iter(&(v_roots.clone()))? { let mut v_r: Py; 
v_r = t2949;
if ({ let t2950 = py::abs(&py::imag(&v_r.clone())?)?; Py::Bool({ let t2951 = py::mul(&Py::Float(1e-12), &py::minmax(&[Py::Float(1.0), py::abs(&v_r.clone())?], true)?)?; if py::le(&t2950, &t2951)? { true } else { false } }) }).truthy() { out.push(py::add(&py::float(&py::real(&v_r.clone())?)?, &Py::Float(0.0))?); }} Py::list(out) }, (Py::Bool(false)).truthy())?;
    v_cplx = py::sorted_key(&{ let mut out = vec![]; for t2952 in py::iter(&(v_roots.clone()))? { let mut v_r: Py; 
v_r = t2952;
if ({ let t2953 = py::abs(&py::imag(&v_r.clone())?)?; Py::Bool({ let t2954 = py::mul(&Py::Float(1e-12), &py::minmax(&[Py::Float(1.0), py::abs(&v_r.clone())?], true)?)?; if py::gt(&t2953, &t2954)? { true } else { false } }) }).truthy() { out.push(py::complex(&[py::add(&py::real(&v_r.clone())?, &Py::Float(0.0))?, py::add(&py::imag(&v_r.clone())?, &Py::Float(0.0))?])?); }} Py::list(out) }, &{ let v_real = v_real.clone(); let v_roots = v_roots.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_z: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![py::round(&py::real(&v_z.clone())?, Some((Py::Int(9i64)).i()?))?, py::neg(&py::imag(&v_z.clone())?)?]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?;
    return Ok(py::add(&v_real.clone(), &v_cplx.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cheb_ord(mut v_nat: Py, mut v_Rp: Py, mut v_As: Py) -> R {
    let mut v_GPASS: Py = Py::None;
    let mut v_GSTOP: Py = Py::None;
    let t583 = (Py::tuple(vec![py::pow(&Py::Int(10i64), &py::mul(&Py::Float(0.1), &v_As.clone())?)?, py::pow(&Py::Int(10i64), &py::mul(&Py::Float(0.1), &v_Rp.clone())?)?])).items()?;
    if t583.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_GSTOP = t583[0].clone();
    v_GPASS = t583[1].clone();
    return Ok(py::int(&py::math1("ceil", &py::truediv(&py::math1("acosh", &py::math1("sqrt", &py::truediv(&py::sub(&v_GSTOP.clone(), &Py::Float(1.0))?, &py::sub(&v_GPASS.clone(), &Py::Float(1.0))?)?)?)?, &py::math1("acosh", &v_nat.clone())?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cpoly(mut v_roots: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_r: Py = Py::None;
    v_c = Py::list(vec![py::add(&Py::Int(1i64), &Py::Complex(0.0, 0.0))?]);
    for t383 in py::iter(&(v_roots.clone()))? {
        v_r = t383;
        v_c = { let mut out = vec![]; for t384 in py::iter(&(py::zip(&[py::add(&v_c.clone(), &Py::list(vec![Py::Complex(0.0, 0.0)]))?, py::add(&Py::list(vec![Py::Complex(0.0, 0.0)]), &v_c.clone())?])?))? { let mut v_u: Py; let mut v_v: Py; 
let t385 = (t384).items()?;
if t385.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_u = t385[0].clone();
v_v = t385[1].clone();
out.push(py::sub(&v_u.clone(), &py::mul(&v_r.clone(), &v_v.clone())?)?); } Py::list(out) };
    }
    return Ok(v_c.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__db10(mut v_x: Py) -> R {
    return Ok(py::mul(&Py::Float(10.0), &py::math1("log10", &v_x.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__db20(mut v_x: Py) -> R {
    return Ok(py::mul(&Py::Float(20.0), &py::math1("log10", &v_x.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__de_recursion(mut v_a: Py, mut v_b: Py, mut v_ic: Py, mut v_c: Py, mut v_q: Py, mut v_n: Py) -> R {
    let mut v_acc: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_y: Py = Py::None;
    v_y = Py::list(vec![]);
    for t1292 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_m = t1292;
        v_acc = Py::Float(0.0);
        for t1293 in py::range_iter(&[(py::len(&v_b.clone())?).i()?]) {
            v_k = t1293;
            if ({ let t1294 = py::sub(&v_m.clone(), &v_k.clone())?; Py::Bool({ let t1295 = Py::Int(0i64); if py::ge(&t1294, &t1295)? { true } else { false } }) }).truthy() {
                v_acc = py::iadd(&v_acc, &py::mul(&py::mul(&py::getitem(&v_b.clone(), &v_k.clone())?, &v_c.clone())?, &py::pow(&v_q.clone(), &py::sub(&v_m.clone(), &v_k.clone())?)?)?)?;
            }
        }
        for t1296 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::len(&v_a.clone())?).i()?]) {
            v_k = t1296;
            v_acc = py::sub(&v_acc, &py::mul(&py::getitem(&v_a.clone(), &v_k.clone())?, &(if ({ let t1297 = py::sub(&v_m.clone(), &v_k.clone())?; Py::Bool({ let t1298 = Py::Int(0i64); if py::ge(&t1297, &t1298)? { true } else { false } }) }).truthy() { py::getitem(&v_y.clone(), &py::sub(&v_m.clone(), &v_k.clone())?)? } else { py::getitem(&v_ic.clone(), &py::sub(&py::sub(&v_k.clone(), &v_m.clone())?, &Py::Int(1i64))?)? }))?)?;
        }
        let _ = py::method(&v_y.clone(), "append", &[py::truediv(&v_acc.clone(), &py::getitem(&v_a.clone(), &Py::Int(0i64))?)?])?;
    }
    return Ok(v_y.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__dft_direct(mut v_x: Py, mut v_sign: Py) -> R {
    let mut v_N: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_t: Py = Py::None;
    let t2879 = (Py::tuple(vec![py::len(&v_x.clone())?, Py::list(vec![])])).items()?;
    if t2879.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_N = t2879[0].clone();
    v_out = t2879[1].clone();
    for t2880 in py::range_iter(&[(v_N.clone()).i()?]) {
        v_k = t2880;
        v_t = { let mut out = vec![]; for t2881 in py::range_iter(&[(v_N.clone()).i()?]) { let mut v_n: Py; 
v_n = t2881;
out.push(py::mul(&py::complex(&[py::getitem(&v_x.clone(), &v_n.clone())?])?, &f__tw(py::mul(&v_k.clone(), &v_n.clone())?, v_N.clone(), v_sign.clone())?)?); } Py::list(out) };
        let _ = py::method(&v_out.clone(), "append", &[py::complex(&[py::fsum(&{ let mut out = vec![]; for t2882 in py::iter(&(v_t.clone()))? { let mut v_z: Py; 
v_z = t2882;
out.push(py::real(&v_z.clone())?); } Py::list(out) })?, py::fsum(&{ let mut out = vec![]; for t2883 in py::iter(&(v_t.clone()))? { let mut v_z: Py; 
v_z = t2883;
out.push(py::imag(&v_z.clone())?); } Py::list(out) })?])?])?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__diff(mut v_a: Py, mut v_b: Py) -> R {
    let mut v_d: Py = Py::None;
    if ({ let t4035 = Py::Bool(matches!(v_a.clone(), Py::Str(_))); if t4035.truthy() { t4035 } else { Py::Bool(matches!(v_b.clone(), Py::Str(_))) } }).truthy() {
        return Ok((if ({ let t4036 = v_a.clone(); Py::Bool({ let t4037 = v_b.clone(); if py::eq(&t4036, &t4037) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { c_INF()? }));
    }
    if ({ let t4038 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if t4038.truthy() { t4038 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }).truthy() {
        if ({ let t4042 = py::not(&{ let t4039 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if !t4039.truthy() { t4039 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }); if t4042.truthy() { t4042 } else { { let t4040 = py::len(&v_a.clone())?; Py::Bool({ let t4041 = py::len(&v_b.clone())?; if py::ne(&t4040, &t4041) { true } else { false } }) } } }).truthy() {
            return Ok(c_INF()?);
        }
        return Ok({ let t = { let mut out = vec![]; for t4043 in py::iter(&(py::zip(&[v_a.clone(), v_b.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t4044 = (t4043).items()?;
if t4044.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t4044[0].clone();
v_y = t4044[1].clone();
out.push(f__diff(v_x.clone(), v_y.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    }
    if ({ let t4049 = { let t4045 = v_a.clone(); Py::Bool({ let t4046 = Py::None; if py::is(&t4045, &t4046) { true } else { false } }) }; if t4049.truthy() { t4049 } else { { let t4047 = v_b.clone(); Py::Bool({ let t4048 = Py::None; if py::is(&t4047, &t4048) { true } else { false } }) } } }).truthy() {
        return Ok((if ({ let t4050 = v_a.clone(); Py::Bool({ let t4051 = v_b.clone(); if py::is(&t4050, &t4051) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { c_INF()? }));
    }
    let t4053: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            let t4052 = (Py::tuple(vec![py::complex(&[v_a.clone()])?, py::complex(&[v_b.clone()])?])).items()?;
            if t4052.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t4052[0].clone();
            v_b = t4052[1].clone();
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4053 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    if ({ let t4054 = v_a.clone(); Py::Bool({ let t4055 = v_b.clone(); if py::eq(&t4054, &t4055) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    v_d = py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?;
    if ({ let t4056 = v_d.clone(); Py::Bool({ let t4057 = v_d.clone(); if py::ne(&t4056, &t4057) { true } else { false } }) }).truthy() {
        return Ok(c_INF()?);
    }
    return Ok(py::truediv(&v_d.clone(), &py::minmax(&[py::abs(&v_a.clone())?, py::abs(&v_b.clone())?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fft(mut v_x: Py, mut v_sign: Py) -> R {
    let mut v_N: Py = Py::None;
    v_N = py::len(&v_x.clone())?;
    return Ok((if ({ let t2870 = py::bitand(&v_N.clone(), &py::sub(&v_N.clone(), &Py::Int(1i64))?)?; Py::Bool({ let t2871 = Py::Int(0i64); if py::eq(&t2870, &t2871) { true } else { false } }) }).truthy() { f__fft2(Py::list((v_x.clone()).items()?), v_sign.clone())? } else { f__dft_direct(v_x.clone(), v_sign.clone())? }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fft2(mut v_x: Py, mut v_sign: Py) -> R {
    let mut v_N: Py = Py::None;
    let mut v_ev: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_od: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_t: Py = Py::None;
    v_N = py::len(&v_x.clone())?;
    if ({ let t2872 = v_N.clone(); Py::Bool({ let t2873 = Py::Int(1i64); if py::eq(&t2872, &t2873) { true } else { false } }) }).truthy() {
        return Ok(Py::list(vec![py::complex(&[py::getitem(&v_x.clone(), &Py::Int(0i64))?])?]));
    }
    let t2874 = (Py::tuple(vec![f__fft2(py::slice(&v_x.clone(), Some((Py::Int(0i64)).i()?), None, Some((Py::Int(2i64)).i()?))?, v_sign.clone())?, f__fft2(py::slice(&v_x.clone(), Some((Py::Int(1i64)).i()?), None, Some((Py::Int(2i64)).i()?))?, v_sign.clone())?])).items()?;
    if t2874.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ev = t2874[0].clone();
    v_od = t2874[1].clone();
    v_out = py::mul(&Py::list(vec![Py::Complex(0.0, 0.0)]), &v_N.clone())?;
    for t2875 in py::range_iter(&[(py::floordiv(&v_N.clone(), &Py::Int(2i64))?).i()?]) {
        v_k = t2875;
        v_t = py::mul(&f__tw(v_k.clone(), v_N.clone(), v_sign.clone())?, &py::getitem(&v_od.clone(), &v_k.clone())?)?;
        let t2878 = (Py::tuple(vec![py::add(&py::getitem(&v_ev.clone(), &v_k.clone())?, &v_t.clone())?, py::sub(&py::getitem(&v_ev.clone(), &v_k.clone())?, &v_t.clone())?])).items()?;
        if t2878.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        py::setitem(&v_out.clone(), &v_k.clone(), t2878[0].clone())?;
        py::setitem(&v_out.clone(), &py::add(&v_k.clone(), &py::floordiv(&v_N.clone(), &Py::Int(2i64))?)?, t2878[1].clone())?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fin(mut v_v: Py, mut v_name: Py) -> R {
    if ({ let t3471 = Py::Bool(matches!(v_v.clone(), Py::Bool(_))); if t3471.truthy() { t3471 } else { { let t3470 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_)))); if t3470.truthy() { t3470 } else { py::not(&py::math1("isfinite", &v_v.clone())?) } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be a finite real number".to_string()].concat()))));
    }
    return Ok(py::float(&v_v.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fspl_ratio(mut v_d: Py, mut v_f: Py) -> R {
    return Ok(py::pow(&py::truediv(&py::mul(&py::mul(&py::mul(&Py::Int(4i64), &Py::Float(std::f64::consts::PI))?, &v_d.clone())?, &v_f.clone())?, &c_C0()?)?, &Py::Int(2i64))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__gamma(mut v_RL: Py, mut v_XL: Py, mut v_Z0: Py) -> R {
    let mut v_d: Py = Py::None;
    let t3355 = (Py::tuple(vec![f__pos(v_RL.clone(), Py::str("load resistance"))?, f__fin(v_XL.clone(), Py::str("load reactance"))?, f__pos(v_Z0.clone(), Py::str("characteristic impedance"))?])).items()?;
    if t3355.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_RL = t3355[0].clone();
    v_XL = t3355[1].clone();
    v_Z0 = t3355[2].clone();
    v_d = py::add(&py::pow(&py::add(&v_RL.clone(), &v_Z0.clone())?, &Py::Int(2i64))?, &py::mul(&v_XL.clone(), &v_XL.clone())?)?;
    return Ok(Py::tuple(vec![py::complex(&[py::add(&py::truediv(&py::add(&py::mul(&py::sub(&v_RL.clone(), &v_Z0.clone())?, &py::add(&v_RL.clone(), &v_Z0.clone())?)?, &py::mul(&v_XL.clone(), &v_XL.clone())?)?, &v_d.clone())?, &Py::Float(0.0))?, py::add(&py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &v_Z0.clone())?, &v_XL.clone())?, &v_d.clone())?, &Py::Float(0.0))?])?, v_Z0.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__int(mut v_v: Py, mut v_name: Py, mut v_lo: Py, mut v_hi: Py) -> R {
    if ({ let t3906 = Py::Bool(matches!(v_v.clone(), Py::Bool(_))); if t3906.truthy() { t3906 } else { { let t3905 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_)))); if t3905.truthy() { t3905 } else { { let t3904 = py::not(&py::math1("isfinite", &v_v.clone())?); if t3904.truthy() { t3904 } else { { let t3902 = py::float(&v_v.clone())?; Py::Bool({ let t3903 = py::int(&v_v.clone())?; if py::ne(&t3902, &t3903) { true } else { false } }) } } } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be a whole number".to_string()].concat()))));
    }
    v_v = py::int(&v_v.clone())?;
    if ({ let t3911 = { let t3907 = v_lo.clone(); Py::Bool({ let t3908 = Py::None; if !py::is(&t3907, &t3908) { true } else { false } }) }; if !t3911.truthy() { t3911 } else { { let t3909 = v_v.clone(); Py::Bool({ let t3910 = v_lo.clone(); if py::lt(&t3909, &t3910)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be at least ".to_string(), py::format(&v_lo.clone(), "")?].concat()))));
    }
    if ({ let t3916 = { let t3912 = v_hi.clone(); Py::Bool({ let t3913 = Py::None; if !py::is(&t3912, &t3913) { true } else { false } }) }; if !t3916.truthy() { t3916 } else { { let t3914 = v_v.clone(); Py::Bool({ let t3915 = v_hi.clone(); if py::gt(&t3914, &t3915)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be at most ".to_string(), py::format(&v_hi.clone(), "")?].concat()))));
    }
    return Ok(v_v.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__lfilter(mut v_b: Py, mut v_a: Py, mut v_x: Py) -> R {
    let mut v_acc: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_y: Py = Py::None;
    if ({ let t3684 = py::getitem(&v_a.clone(), &Py::Int(0i64))?; Py::Bool({ let t3685 = Py::Int(0i64); if py::eq(&t3684, &t3685) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a[0] must not be 0"))));
    }
    v_y = Py::list(vec![]);
    for t3686 in py::range_iter(&[(py::len(&v_x.clone())?).i()?]) {
        v_n = t3686;
        v_acc = py::fsum(&py::add(&{ let mut out = vec![]; for t3687 in py::range_iter(&[(py::len(&v_b.clone())?).i()?]) { let mut v_k: Py; 
v_k = t3687;
if ({ let t3688 = py::sub(&v_n.clone(), &v_k.clone())?; Py::Bool({ let t3689 = Py::Int(0i64); if py::ge(&t3688, &t3689)? { true } else { false } }) }).truthy() { out.push(py::mul(&py::getitem(&v_b.clone(), &v_k.clone())?, &py::getitem(&v_x.clone(), &py::sub(&v_n.clone(), &v_k.clone())?)?)?); }} Py::list(out) }, &{ let mut out = vec![]; for t3690 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::len(&v_a.clone())?).i()?]) { let mut v_k: Py; 
v_k = t3690;
if ({ let t3691 = py::sub(&v_n.clone(), &v_k.clone())?; Py::Bool({ let t3692 = Py::Int(0i64); if py::ge(&t3691, &t3692)? { true } else { false } }) }).truthy() { out.push(py::mul(&py::neg(&py::getitem(&v_a.clone(), &v_k.clone())?)?, &py::getitem(&v_y.clone(), &py::sub(&v_n.clone(), &v_k.clone())?)?)?); }} Py::list(out) })?)?;
        let _ = py::method(&v_y.clone(), "append", &[py::truediv(&v_acc.clone(), &py::getitem(&v_a.clone(), &Py::Int(0i64))?)?])?;
    }
    return Ok(v_y.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__nonneg(mut v_v: Py, mut v_name: Py) -> R {
    if ({ let t3778 = Py::Bool(matches!(v_v.clone(), Py::Bool(_))); if t3778.truthy() { t3778 } else { { let t3777 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_)))); if t3777.truthy() { t3777 } else { { let t3776 = py::not(&py::math1("isfinite", &v_v.clone())?); if t3776.truthy() { t3776 } else { { let t3774 = v_v.clone(); Py::Bool({ let t3775 = Py::Int(0i64); if py::lt(&t3774, &t3775)? { true } else { false } }) } } } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be a finite number >= 0".to_string()].concat()))));
    }
    return Ok(py::float(&v_v.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__order_args(mut v_fp: Py, mut v_fstop: Py, mut v_Rp: Py, mut v_As: Py) -> R {
    let t574 = (Py::tuple(vec![f__pos(v_fp.clone(), Py::str("passband edge frequency"))?, f__pos(v_fstop.clone(), Py::str("stopband edge frequency"))?])).items()?;
    if t574.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fp = t574[0].clone();
    v_fstop = t574[1].clone();
    let t575 = (Py::tuple(vec![f__pos(v_Rp.clone(), Py::str("maximum passband attenuation (dB)"))?, f__pos(v_As.clone(), Py::str("minimum stopband attenuation (dB)"))?])).items()?;
    if t575.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Rp = t575[0].clone();
    v_As = t575[1].clone();
    if ({ let t576 = v_fp.clone(); Py::Bool({ let t577 = v_fstop.clone(); if py::eq(&t576, &t577) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the passband and stopband edges must differ"))));
    }
    if ({ let t578 = v_As.clone(); Py::Bool({ let t579 = v_Rp.clone(); if py::le(&t578, &t579)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the stopband attenuation must exceed the passband attenuation"))));
    }
    return Ok(Py::tuple(vec![v_fp.clone(), v_fstop.clone(), v_Rp.clone(), v_As.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pad2(mut v_b: Py, mut v_a: Py) -> R {
    let mut v_L: Py = Py::None;
    v_L = py::minmax(&[py::len(&v_b.clone())?, py::len(&v_a.clone())?], true)?;
    return Ok(Py::tuple(vec![py::add(&Py::list((v_b.clone()).items()?), &py::mul(&Py::list(vec![Py::Float(0.0)]), &py::sub(&v_L.clone(), &py::len(&v_b.clone())?)?)?)?, py::add(&Py::list((v_a.clone()).items()?), &py::mul(&Py::list(vec![Py::Float(0.0)]), &py::sub(&v_L.clone(), &py::len(&v_a.clone())?)?)?)?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__peak(mut v_mags: Py, mut v_scale: Py) -> R {
    let mut v_top: Py = Py::None;
    v_top = py::minmax(&[v_mags.clone()], true)?;
    return Ok((match ('g2894: { let mut out = vec![]; for t2895 in py::iter(&(py::enumerate(&v_mags.clone(), 0)?))? { let mut v_i: Py; let mut v_m: Py; 
let t2896 = (t2895).items()?;
if t2896.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_i = t2896[0].clone();
v_m = t2896[1].clone();
if ({ let t2897 = v_m.clone(); Py::Bool({ let t2898 = py::sub(&v_top.clone(), &py::mul(&Py::Float(1e-09), &py::minmax(&[v_scale.clone(), Py::Float(1e-300)], true)?)?)?; if py::ge(&t2897, &t2898)? { true } else { false } }) }).truthy() { out.push(v_i.clone()); break 'g2894 Py::list(out); }} Py::list(out) }).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pf_expand(mut v_num: Py, mut v_den: Py) -> R {
    let mut v_K: Py = Py::None;
    let mut v_Kr: Py = Py::None;
    let mut v_N: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_R: Py = Py::None;
    let mut v_Rr: Py = Py::None;
    let mut v_big: Py = Py::None;
    let mut v_bot: Py = Py::None;
    let mut v_coef: Py = Py::None;
    let mut v_d0: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_kscale: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_poles: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_rem: Py = Py::None;
    let mut v_res: Py = Py::None;
    let mut v_top: Py = Py::None;
    let mut v_w: Py = Py::None;
    let t2321 = (Py::tuple(vec![f__trim0(v_den.clone())?, f__trim0(v_num.clone())?])).items()?;
    if t2321.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_den = t2321[0].clone();
    v_num = t2321[1].clone();
    if ({ let t2322 = py::getitem(&v_den.clone(), &Py::Int(0i64))?; Py::Bool({ let t2323 = Py::Int(0i64); if py::eq(&t2322, &t2323) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a[0] must not be 0"))));
    }
    if (Py::Bool(({ let mut out = vec![]; for t2324 in py::iter(&(v_num.clone()))? { let mut v_v: Py; 
v_v = t2324;
out.push({ let t2325 = v_v.clone(); Py::Bool({ let t2326 = Py::Int(0i64); if py::eq(&t2325, &t2326) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the numerator is identically zero"))));
    }
    v_d0 = py::getitem(&v_den.clone(), &Py::Int(0i64))?;
    v_den = { let mut out = vec![]; for t2327 in py::iter(&(v_den.clone()))? { let mut v_v: Py; 
v_v = t2327;
out.push(py::truediv(&v_v.clone(), &v_d0.clone())?); } Py::list(out) };
    v_num = { let mut out = vec![]; for t2328 in py::iter(&(v_num.clone()))? { let mut v_v: Py; 
v_v = t2328;
out.push(py::truediv(&v_v.clone(), &v_d0.clone())?); } Py::list(out) };
    v_N = py::sub(&py::len(&v_den.clone())?, &Py::Int(1i64))?;
    v_rem = py::add(&Py::list((v_num.clone()).items()?), &py::mul(&Py::list(vec![Py::Float(0.0)]), &py::minmax(&[Py::Int(0i64), py::sub(&v_N.clone(), &py::len(&v_num.clone())?)?], true)?)?)?;
    v_K = Py::list(vec![]);
    if ({ let t2329 = py::len(&v_num.clone())?; Py::Bool({ let t2330 = v_N.clone(); if py::gt(&t2329, &t2330)? { true } else { false } }) }).truthy() {
        v_K = py::mul(&Py::list(vec![Py::Float(0.0)]), &py::sub(&py::len(&v_num.clone())?, &v_N.clone())?)?;
        for t2331 in py::range_iter(&[(py::sub(&py::sub(&py::len(&v_num.clone())?, &v_N.clone())?, &Py::Int(1i64))?).i()?, (py::neg(&Py::Int(1i64))?).i()?, (py::neg(&Py::Int(1i64))?).i()?]) {
            v_i = t2331;
            v_coef = py::truediv(&py::getitem(&v_rem.clone(), &py::add(&v_i.clone(), &v_N.clone())?)?, &py::getitem(&v_den.clone(), &v_N.clone())?)?;
            py::setitem(&v_K.clone(), &v_i.clone(), v_coef.clone())?;
            for t2332 in py::range_iter(&[(py::add(&v_N.clone(), &Py::Int(1i64))?).i()?]) {
                v_j = t2332;
                let t2333 = v_rem.clone(); let t2334 = py::add(&v_i.clone(), &v_j.clone())?;
                py::setitem(&t2333, &t2334, py::sub(&py::getitem(&t2333, &t2334)?, &py::mul(&v_coef.clone(), &py::getitem(&v_den.clone(), &v_j.clone())?)?)?)?;
            }
        }
    }
    v_R = py::slice(&v_rem.clone(), None, Some((v_N.clone()).i()?), None)?;
    v_kscale = py::minmax(&[py::add(&py::add(&{ let mut out = vec![]; for t2335 in py::iter(&(v_num.clone()))? { let mut v_v: Py; 
v_v = t2335;
out.push(py::abs(&v_v.clone())?); } Py::list(out) }, &{ let mut out = vec![]; for t2336 in py::iter(&(v_K.clone()))? { let mut v_v: Py; 
v_v = t2336;
out.push(py::abs(&v_v.clone())?); } Py::list(out) })?, &Py::list(vec![Py::Float(1e-300)]))?], true)?;
    v_K = { let mut out = vec![]; for t2337 in py::iter(&(v_K.clone()))? { let mut v_v: Py; 
v_v = t2337;
out.push((if ({ let t2338 = py::abs(&v_v.clone())?; Py::Bool({ let t2339 = py::mul(&Py::Float(1e-12), &v_kscale.clone())?; if py::le(&t2338, &t2339)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_v.clone() })); } Py::list(out) };
    while ({ let t2344 = { let t2340 = py::len(&v_K.clone())?; Py::Bool({ let t2341 = Py::Int(0i64); if py::gt(&t2340, &t2341)? { true } else { false } }) }; if !t2344.truthy() { t2344 } else { { let t2342 = py::getitem(&v_K.clone(), &py::neg(&Py::Int(1i64))?)?; Py::Bool({ let t2343 = Py::Int(0i64); if py::eq(&t2342, &t2343) { true } else { false } }) } } }).truthy() {
        let _ = py::method(&v_K.clone(), "pop", &[])?;
    }
    v_poles = (if ({ let t2345 = v_N.clone(); Py::Bool({ let t2346 = Py::Int(0i64); if py::gt(&t2345, &t2346)? { true } else { false } }) }).truthy() { f__canon_roots(f__roots(v_den.clone())?)? } else { Py::list(vec![]) });
    for t2397 in py::range_iter(&[(py::len(&v_poles.clone())?).i()?]) {
        v_i = t2397;
        if ({ let t2398 = py::getitem(&v_poles.clone(), &v_i.clone())?; Py::Bool({ let t2399 = Py::Int(0i64); if py::eq(&t2398, &t2399) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("a pole at z = 0 cannot occur with a non-zero last coefficient of a"))));
        }
        for t2400 in py::range_iter(&[(py::add(&v_i.clone(), &Py::Int(1i64))?).i()?, (py::len(&v_poles.clone())?).i()?]) {
            v_j = t2400;
            if ({ let t2401 = py::abs(&py::sub(&py::getitem(&v_poles.clone(), &v_i.clone())?, &py::getitem(&v_poles.clone(), &v_j.clone())?)?)?; Py::Bool({ let t2402 = py::mul(&Py::Float(0.001), &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_poles.clone(), &v_i.clone())?)?], true)?)?; if py::le(&t2401, &t2402)? { true } else { false } }) }).truthy() {
                return Err(py::value_error(py::to_str(&Py::str("repeated (or nearly repeated) poles: this method needs distinct poles at least 0.001 apart"))));
            }
        }
    }
    v_res = Py::list(vec![]);
    for t2403 in py::range_iter(&[(py::len(&v_poles.clone())?).i()?]) {
        v_i = t2403;
        v_p = py::getitem(&v_poles.clone(), &v_i.clone())?;
        v_w = py::truediv(&Py::Int(1i64), &v_p.clone())?;
        v_top = Py::Float(0.0);
        for t2404 in py::range_iter(&[(py::sub(&py::len(&v_R.clone())?, &Py::Int(1i64))?).i()?, (py::neg(&Py::Int(1i64))?).i()?, (py::neg(&Py::Int(1i64))?).i()?]) {
            v_k = t2404;
            v_top = py::add(&py::mul(&v_top.clone(), &v_w.clone())?, &py::getitem(&v_R.clone(), &v_k.clone())?)?;
        }
        v_bot = Py::Float(1.0);
        for t2405 in py::range_iter(&[(py::len(&v_poles.clone())?).i()?]) {
            v_j = t2405;
            if ({ let t2406 = v_j.clone(); Py::Bool({ let t2407 = v_i.clone(); if py::ne(&t2406, &t2407) { true } else { false } }) }).truthy() {
                v_bot = py::mul(&v_bot, &py::sub(&Py::Int(1i64), &py::truediv(&py::getitem(&v_poles.clone(), &v_j.clone())?, &v_p.clone())?)?)?;
            }
        }
        let _ = py::method(&v_res.clone(), "append", &[py::truediv(&v_top.clone(), &v_bot.clone())?])?;
    }
    v_big = py::minmax(&[py::add(&{ let mut out = vec![]; for t2408 in py::iter(&(v_res.clone()))? { let mut v_r: Py; 
v_r = t2408;
out.push(py::abs(&v_r.clone())?); } Py::list(out) }, &Py::list(vec![Py::Float(0.0)]))?], true)?;
    for t2409 in py::iter(&(v_res.clone()))? {
        v_r = t2409;
        if ({ let t2410 = py::abs(&v_r.clone())?; Py::Bool({ let t2411 = py::mul(&Py::Float(1e-09), &v_big.clone())?; if py::le(&t2410, &t2411)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("a pole is cancelled by a zero: reduce the fraction (or the equation) first"))));
        }
    }
    v_P = { let mut out = vec![]; for t2412 in py::iter(&(v_poles.clone()))? { let mut v_p: Py; 
v_p = t2412;
out.push(f__zc12(v_p.clone(), py::minmax(&[Py::Float(1.0), py::abs(&v_p.clone())?], true)?)?); } Py::list(out) };
    v_Rr = { let mut out = vec![]; for t2419 in py::iter(&(v_res.clone()))? { let mut v_r: Py; 
v_r = t2419;
out.push(f__zc12(v_r.clone(), py::minmax(&[py::abs(&v_r.clone())?, Py::Float(1e-300)], true)?)?); } Py::list(out) };
    v_Kr = { let mut out = vec![]; for t2420 in py::iter(&(v_K.clone()))? { let mut v_v: Py; 
v_v = t2420;
out.push(f__zr12(v_v.clone())?); } Py::list(out) };
    return Ok(Py::tuple(vec![v_poles.clone(), v_res.clone(), v_K.clone(), v_P.clone(), v_Rr.clone(), v_Kr.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pmul(mut v_p: Py, mut v_q: Py) -> R {
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_out: Py = Py::None;
    v_out = py::mul(&Py::list(vec![Py::Float(0.0)]), &py::sub(&py::add(&py::len(&v_p.clone())?, &py::len(&v_q.clone())?)?, &Py::Int(1i64))?)?;
    for t4001 in py::range_iter(&[(py::len(&v_p.clone())?).i()?]) {
        v_i = t4001;
        for t4002 in py::range_iter(&[(py::len(&v_q.clone())?).i()?]) {
            v_j = t4002;
            let t4003 = v_out.clone(); let t4004 = py::add(&v_i.clone(), &v_j.clone())?;
            py::setitem(&t4003, &t4004, py::iadd(&py::getitem(&t4003, &t4004)?, &py::mul(&py::getitem(&v_p.clone(), &v_i.clone())?, &py::getitem(&v_q.clone(), &v_j.clone())?)?)?)?;
        }
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__poly_at(mut v_c: Py, mut v_w: Py) -> R {
    let mut v_s: Py = Py::None;
    let mut v_sd: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_td: Py = Py::None;
    v_t = { let mut out = vec![]; for t1890 in py::iter(&(py::enumerate(&v_c.clone(), 0)?))? { let mut v_ck: Py; let mut v_k: Py; 
let t1891 = (t1890).items()?;
if t1891.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t1891[0].clone();
v_ck = t1891[1].clone();
out.push(py::mul(&v_ck.clone(), &py::cmath("exp", &[py::mul(&py::mul(&py::neg(&Py::Complex(0.0, 1.0))?, &v_w.clone())?, &v_k.clone())?])?)?); } Py::list(out) };
    v_td = { let mut out = vec![]; for t1892 in py::iter(&(py::enumerate(&v_t.clone(), 0)?))? { let mut v_k: Py; let mut v_z: Py; 
let t1893 = (t1892).items()?;
if t1893.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t1893[0].clone();
v_z = t1893[1].clone();
out.push(py::mul(&v_k.clone(), &v_z.clone())?); } Py::list(out) };
    v_s = py::complex(&[py::fsum(&{ let mut out = vec![]; for t1894 in py::iter(&(v_t.clone()))? { let mut v_z: Py; 
v_z = t1894;
out.push(py::real(&v_z.clone())?); } Py::list(out) })?, py::fsum(&{ let mut out = vec![]; for t1895 in py::iter(&(v_t.clone()))? { let mut v_z: Py; 
v_z = t1895;
out.push(py::imag(&v_z.clone())?); } Py::list(out) })?])?;
    v_sd = py::complex(&[py::fsum(&{ let mut out = vec![]; for t1896 in py::iter(&(v_td.clone()))? { let mut v_z: Py; 
v_z = t1896;
out.push(py::real(&v_z.clone())?); } Py::list(out) })?, py::fsum(&{ let mut out = vec![]; for t1897 in py::iter(&(v_td.clone()))? { let mut v_z: Py; 
v_z = t1897;
out.push(py::imag(&v_z.clone())?); } Py::list(out) })?])?;
    return Ok(Py::tuple(vec![v_s.clone(), v_sd.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pos(mut v_v: Py, mut v_name: Py) -> R {
    if ({ let t3923 = Py::Bool(matches!(v_v.clone(), Py::Bool(_))); if t3923.truthy() { t3923 } else { { let t3922 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_)))); if t3922.truthy() { t3922 } else { { let t3921 = py::not(&py::math1("isfinite", &v_v.clone())?); if t3921.truthy() { t3921 } else { { let t3919 = v_v.clone(); Py::Bool({ let t3920 = Py::Int(0i64); if py::le(&t3919, &t3920)? { true } else { false } }) } } } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be a positive finite number".to_string()].concat()))));
    }
    return Ok(py::float(&v_v.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__qf(mut v_x: Py) -> R {
    return Ok(py::mul(&Py::Float(0.5), &py::math1("erfc", &py::truediv(&v_x.clone(), &py::math1("sqrt", &Py::Float(2.0))?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__r12(mut v_x: Py) -> R {
    return Ok((if ({ let t1475 = py::abs(&v_x.clone())?; Py::Bool({ let t1476 = Py::Float(1e-15); if py::lt(&t1475, &t1476)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { py::add(&py::float(&Py::str(&[py::format(&v_x.clone(), ".12g")?].concat()))?, &Py::Float(0.0))? }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__radar_k(mut v_Pt: Py, mut v_G_db: Py, mut v_f: Py, mut v_sigma: Py) -> R {
    let mut v_G: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let t3268 = (Py::tuple(vec![f__pos(v_Pt.clone(), Py::str("peak transmit power"))?, f__pos(v_f.clone(), Py::str("frequency"))?, f__pos(v_sigma.clone(), Py::str("radar cross-section"))?])).items()?;
    if t3268.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_Pt = t3268[0].clone();
    v_f = t3268[1].clone();
    v_sigma = t3268[2].clone();
    v_G = py::pow(&Py::Int(10i64), &py::truediv(&f__fin(v_G_db.clone(), Py::str("antenna gain in dBi"))?, &Py::Int(10i64))?)?;
    v_lam = py::truediv(&c_C0()?, &v_f.clone())?;
    return Ok(py::truediv(&py::mul(&py::mul(&py::mul(&py::mul(&py::mul(&v_Pt.clone(), &v_G.clone())?, &v_G.clone())?, &v_lam.clone())?, &v_lam.clone())?, &v_sigma.clone())?, &py::pow(&py::mul(&Py::Int(4i64), &Py::Float(std::f64::consts::PI))?, &Py::Int(3i64))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__resid(mut v_got: Py, mut v_want: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_worst = Py::Float(0.0);
    for t4032 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4058 = (t4032).items()?;
        if t4058.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4058[0].clone();
        v_w = t4058[1].clone();
        if ({ let t4033 = v_k.clone(); Py::Bool({ let t4034 = v_got.clone(); if !py::contains(&t4034, &t4033)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_got.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__roots(mut v_c: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_den: Py = Py::None;
    let mut v_done: Py = Py::None;
    let mut v_dp: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_new: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_pol: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_rad: Py = Py::None;
    let mut v_step: Py = Py::None;
    let mut v_z: Py = Py::None;
    let mut v_zeros: Py = Py::None;
    v_c = { let mut out = vec![]; for t2955 in py::iter(&(v_c.clone()))? { let mut v_v: Py; 
v_v = t2955;
out.push(py::complex(&[v_v.clone()])?); } Py::list(out) };
    while ({ let t2958 = v_c.clone(); if !t2958.truthy() { t2958 } else { { let t2956 = py::getitem(&v_c.clone(), &Py::Int(0i64))?; Py::Bool({ let t2957 = Py::Int(0i64); if py::eq(&t2956, &t2957) { true } else { false } }) } } }).truthy() {
        let _ = py::method(&v_c.clone(), "pop", &[Py::Int(0i64)])?;
    }
    if (py::not(&v_c.clone())).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the polynomial is identically zero"))));
    }
    v_zeros = Py::Int(0i64);
    while ({ let t2963 = { let t2959 = py::len(&v_c.clone())?; Py::Bool({ let t2960 = Py::Int(1i64); if py::gt(&t2959, &t2960)? { true } else { false } }) }; if !t2963.truthy() { t2963 } else { { let t2961 = py::getitem(&v_c.clone(), &py::neg(&Py::Int(1i64))?)?; Py::Bool({ let t2962 = Py::Int(0i64); if py::eq(&t2961, &t2962) { true } else { false } }) } } }).truthy() {
        let _ = py::method(&v_c.clone(), "pop", &[])?;
        v_zeros = py::iadd(&v_zeros, &Py::Int(1i64))?;
    }
    v_n = py::sub(&py::len(&v_c.clone())?, &Py::Int(1i64))?;
    v_out = py::mul(&Py::list(vec![Py::Complex(0.0, 0.0)]), &v_zeros.clone())?;
    if ({ let t2964 = v_n.clone(); Py::Bool({ let t2965 = Py::Int(0i64); if py::eq(&t2964, &t2965) { true } else { false } }) }).truthy() {
        return Ok(v_out.clone());
    }
    v_a = { let mut out = vec![]; for t2966 in py::iter(&(v_c.clone()))? { let mut v_v: Py; 
v_v = t2966;
out.push(py::truediv(&v_v.clone(), &py::getitem(&v_c.clone(), &Py::Int(0i64))?)?); } Py::list(out) };
    if ({ let t2967 = v_n.clone(); Py::Bool({ let t2968 = Py::Int(1i64); if py::eq(&t2967, &t2968) { true } else { false } }) }).truthy() {
        return Ok(py::add(&v_out.clone(), &Py::list(vec![py::neg(&py::getitem(&v_a.clone(), &Py::Int(1i64))?)?]))?);
    }
    if ({ let t2969 = v_n.clone(); Py::Bool({ let t2970 = Py::Int(2i64); if py::eq(&t2969, &t2970) { true } else { false } }) }).truthy() {
        v_d = py::cmath("sqrt", &[py::sub(&py::mul(&py::getitem(&v_a.clone(), &Py::Int(1i64))?, &py::getitem(&v_a.clone(), &Py::Int(1i64))?)?, &py::mul(&Py::Int(4i64), &py::getitem(&v_a.clone(), &Py::Int(2i64))?)?)?])?;
        v_q = py::truediv(&py::neg(&py::add(&py::getitem(&v_a.clone(), &Py::Int(1i64))?, &(if ({ let t2971 = py::real(&py::mul(&py::method(&py::getitem(&v_a.clone(), &Py::Int(1i64))?, "conjugate", &[])?, &v_d.clone())?)?; Py::Bool({ let t2972 = Py::Int(0i64); if py::ge(&t2971, &t2972)? { true } else { false } }) }).truthy() { v_d.clone() } else { py::neg(&v_d.clone())? }))?)?, &Py::Int(2i64))?;
        return Ok(py::add(&v_out.clone(), &(if ({ let t2973 = v_q.clone(); Py::Bool({ let t2974 = Py::Int(0i64); if py::ne(&t2973, &t2974) { true } else { false } }) }).truthy() { Py::list(vec![v_q.clone(), py::truediv(&py::getitem(&v_a.clone(), &Py::Int(2i64))?, &v_q.clone())?]) } else { Py::list(vec![Py::Complex(0.0, 0.0), Py::Complex(0.0, 0.0)]) }))?);
    }
    v_p = { let v_a = v_a.clone(); let v_c = v_c.clone(); let v_d = v_d.clone(); let v_n = v_n.clone(); let v_out = v_out.clone(); let v_q = v_q.clone(); let v_zeros = v_zeros.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_z: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sum(&{ let mut out = vec![]; for t2975 in py::iter(&(py::enumerate(&v_a.clone(), 0)?))? { let mut v_ak: Py; let mut v_k: Py; 
let t2976 = (t2975).items()?;
if t2976.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2976[0].clone();
v_ak = t2976[1].clone();
out.push(py::mul(&v_ak.clone(), &py::pow(&v_z.clone(), &py::sub(&v_n.clone(), &v_k.clone())?)?)?); } Py::list(out) }, None)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_dp = { let v_a = v_a.clone(); let v_c = v_c.clone(); let v_d = v_d.clone(); let v_n = v_n.clone(); let v_out = v_out.clone(); let v_p = v_p.clone(); let v_q = v_q.clone(); let v_zeros = v_zeros.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_z: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sum(&{ let mut out = vec![]; for t2977 in py::iter(&(py::enumerate(&py::slice(&v_a.clone(), None, Some((py::neg(&Py::Int(1i64))?).i()?), None)?, 0)?))? { let mut v_ak: Py; let mut v_k: Py; 
let t2978 = (t2977).items()?;
if t2978.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2978[0].clone();
v_ak = t2978[1].clone();
out.push(py::mul(&py::mul(&v_ak.clone(), &py::sub(&v_n.clone(), &v_k.clone())?)?, &py::pow(&v_z.clone(), &py::sub(&py::sub(&v_n.clone(), &v_k.clone())?, &Py::Int(1i64))?)?)?); } Py::list(out) }, None)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_rad = py::add(&Py::Int(1i64), &py::minmax(&[{ let mut out = vec![]; for t2979 in py::iter(&(py::slice(&v_a.clone(), Some((Py::Int(1i64)).i()?), None, None)?))? { let mut v_v: Py; 
v_v = t2979;
out.push(py::abs(&v_v.clone())?); } Py::list(out) }], true)?)?;
    v_z = { let mut out = vec![]; for t2980 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t2980;
out.push(py::mul(&v_rad.clone(), &py::cmath("exp", &[py::mul(&Py::Complex(0.0, 1.0), &py::add(&py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_k.clone())?, &v_n.clone())?, &Py::Float(0.4))?)?])?)?); } Py::list(out) };
    for t2981 in py::range_iter(&[(Py::Int(500i64)).i()?]) {
        v__ = t2981;
        v_new = Py::list(vec![]);
        for t2982 in py::range_iter(&[(v_n.clone()).i()?]) {
            v_i = t2982;
            v_den = py::add(&Py::Int(1i64), &Py::Complex(0.0, 0.0))?;
            for t2983 in py::range_iter(&[(v_n.clone()).i()?]) {
                v_j = t2983;
                if ({ let t2984 = v_j.clone(); Py::Bool({ let t2985 = v_i.clone(); if py::ne(&t2984, &t2985) { true } else { false } }) }).truthy() {
                    v_den = py::mul(&v_den, &py::sub(&py::getitem(&v_z.clone(), &v_i.clone())?, &py::getitem(&v_z.clone(), &v_j.clone())?)?)?;
                }
            }
            let _ = py::method(&v_new.clone(), "append", &[(if ({ let t2986 = v_den.clone(); Py::Bool({ let t2987 = Py::Int(0i64); if py::ne(&t2986, &t2987) { true } else { false } }) }).truthy() { py::sub(&py::getitem(&v_z.clone(), &v_i.clone())?, &py::truediv(&py::call(&v_p.clone(), &[py::getitem(&v_z.clone(), &v_i.clone())?])?, &v_den.clone())?)? } else { py::add(&py::getitem(&v_z.clone(), &v_i.clone())?, &Py::Float(1e-09))? })])?;
        }
        v_done = { let t2988 = py::minmax(&[{ let mut out = vec![]; for t2990 in py::iter(&(py::zip(&[v_new.clone(), v_z.clone()])?))? { let mut v_u: Py; let mut v_v: Py; 
let t2991 = (t2990).items()?;
if t2991.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_u = t2991[0].clone();
v_v = t2991[1].clone();
out.push(py::abs(&py::sub(&v_u.clone(), &v_v.clone())?)?); } Py::list(out) }], true)?; Py::Bool({ let t2989 = py::mul(&Py::Float(1e-15), &py::minmax(&[Py::Float(1.0), py::minmax(&[{ let mut out = vec![]; for t2992 in py::iter(&(v_new.clone()))? { let mut v_v: Py; 
v_v = t2992;
out.push(py::abs(&v_v.clone())?); } Py::list(out) }], true)?], true)?)?; if py::le(&t2988, &t2989)? { true } else { false } }) };
        v_z = v_new.clone();
        if (v_done.clone()).truthy() {
            break;
        }
    }
    v_pol = Py::list(vec![]);
    for t2993 in py::iter(&(v_z.clone()))? {
        v_r = t2993;
        for t2994 in py::range_iter(&[(Py::Int(3i64)).i()?]) {
            v__ = t2994;
            v_d = py::call(&v_dp.clone(), &[v_r.clone()])?;
            if ({ let t2995 = v_d.clone(); Py::Bool({ let t2996 = Py::Int(0i64); if py::eq(&t2995, &t2996) { true } else { false } }) }).truthy() {
                break;
            }
            v_step = py::truediv(&py::call(&v_p.clone(), &[v_r.clone()])?, &v_d.clone())?;
            if ({ let t2997 = py::abs(&v_step.clone())?; Py::Bool({ let t2998 = py::mul(&Py::Float(1e-06), &py::minmax(&[Py::Float(1.0), py::abs(&v_r.clone())?], true)?)?; if py::gt(&t2997, &t2998)? { true } else { false } }) }).truthy() {
                break;
            }
            v_r = py::sub(&v_r, &v_step.clone())?;
        }
        let _ = py::method(&v_pol.clone(), "append", &[v_r.clone()])?;
    }
    return Ok(py::add(&v_out.clone(), &v_pol.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__snap(mut v_z: Py, mut v_scale: Py) -> R {
    let mut v_tol: Py = Py::None;
    v_tol = py::mul(&Py::Float(1e-11), &v_scale.clone())?;
    v_z = py::complex(&[v_z.clone()])?;
    return Ok(py::complex(&[py::add(&(if ({ let t2247 = py::abs(&py::real(&v_z.clone())?)?; Py::Bool({ let t2248 = v_tol.clone(); if py::gt(&t2247, &t2248)? { true } else { false } }) }).truthy() { py::real(&v_z.clone())? } else { Py::Float(0.0) }), &Py::Float(0.0))?, py::add(&(if ({ let t2249 = py::abs(&py::imag(&v_z.clone())?)?; Py::Bool({ let t2250 = v_tol.clone(); if py::gt(&t2249, &t2250)? { true } else { false } }) }).truthy() { py::imag(&v_z.clone())? } else { Py::Float(0.0) }), &Py::Float(0.0))?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__trim0(mut v_c: Py) -> R {
    v_c = Py::list((v_c.clone()).items()?);
    while ({ let t4027 = { let t4023 = py::len(&v_c.clone())?; Py::Bool({ let t4024 = Py::Int(1i64); if py::gt(&t4023, &t4024)? { true } else { false } }) }; if !t4027.truthy() { t4027 } else { { let t4025 = py::getitem(&v_c.clone(), &py::neg(&Py::Int(1i64))?)?; Py::Bool({ let t4026 = Py::Int(0i64); if py::eq(&t4025, &t4026) { true } else { false } }) } } }).truthy() {
        let _ = py::method(&v_c.clone(), "pop", &[])?;
    }
    return Ok(v_c.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__tw(mut v_m: Py, mut v_N: Py, mut v_sign: Py) -> R {
    let mut v_a: Py = Py::None;
    v_m = py::modulo(&v_m, &v_N.clone())?;
    if ({ let t2876 = py::modulo(&py::mul(&Py::Int(4i64), &v_m.clone())?, &v_N.clone())?; Py::Bool({ let t2877 = Py::Int(0i64); if py::eq(&t2876, &t2877) { true } else { false } }) }).truthy() {
        return Ok(py::getitem(&Py::tuple(vec![py::add(&Py::Int(1i64), &Py::Complex(0.0, 0.0))?, py::complex(&[Py::Int(0i64), v_sign.clone()])?, py::add(&py::neg(&Py::Int(1i64))?, &Py::Complex(0.0, 0.0))?, py::complex(&[Py::Int(0i64), py::neg(&v_sign.clone())?])?]), &py::floordiv(&py::mul(&Py::Int(4i64), &v_m.clone())?, &v_N.clone())?)?);
    }
    v_a = py::truediv(&py::mul(&py::mul(&py::mul(&v_sign.clone(), &Py::Float(2.0))?, &Py::Float(std::f64::consts::PI))?, &v_m.clone())?, &v_N.clone())?;
    return Ok(py::complex(&[py::math1("cos", &v_a.clone())?, py::math1("sin", &v_a.clone())?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__vec(mut v_v: Py, mut v_name: Py, mut v_lo: Py, mut v_hi: Py) -> R {
    if ({ let t3957 = py::not(&Py::Bool(matches!(v_v.clone(), Py::List(_) | Py::Tuple(_)))); if t3957.truthy() { t3957 } else { py::not(&Py::Bool(({ let mut out = vec![]; for t3954 in py::iter(&(v_v.clone()))? { let mut v_x: Py; 
v_x = t3954;
out.push({ let t3956 = Py::Bool(matches!(v_x.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3956.truthy() { t3956 } else { { let t3955 = py::not(&Py::Bool(matches!(v_x.clone(), Py::Bool(_)))); if !t3955.truthy() { t3955 } else { py::math1("isfinite", &v_x.clone())? } } } }); } Py::list(out) }).items()?.iter().all(|x| x.truthy()))) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be a list of finite numbers".to_string()].concat()))));
    }
    if ({ let t3958 = py::len(&v_v.clone())?; Py::Bool({ let t3959 = v_lo.clone(); if py::lt(&t3958, &t3959)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " needs at least ".to_string(), py::format(&v_lo.clone(), "")?, " values".to_string()].concat()))));
    }
    if ({ let t3964 = { let t3960 = v_hi.clone(); Py::Bool({ let t3961 = Py::None; if !py::is(&t3960, &t3961) { true } else { false } }) }; if !t3964.truthy() { t3964 } else { { let t3962 = py::len(&v_v.clone())?; Py::Bool({ let t3963 = v_hi.clone(); if py::gt(&t3962, &t3963)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " allows at most ".to_string(), py::format(&v_hi.clone(), "")?, " values".to_string()].concat()))));
    }
    return Ok({ let mut out = vec![]; for t3965 in py::iter(&(v_v.clone()))? { let mut v_x: Py; 
v_x = t3965;
out.push(py::float(&v_x.clone())?); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_de(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_rec: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1303: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_n = (if ({ let t1263 = Py::str("n"); Py::Bool({ let t1264 = v_inputs.clone(); if py::contains(&t1264, &t1263)? { true } else { false } }) }).truthy() { py::getitem(&v_inputs.clone(), &Py::str("n"))? } else { Py::Int(8i64) });
            v_want = f_difference_equation_z(py::getitem(&v_inputs.clone(), &Py::str("a"))?, py::getitem(&v_inputs.clone(), &Py::str("b"))?, py::getitem(&v_inputs.clone(), &Py::str("initial_conditions"))?, py::getitem(&v_inputs.clone(), &Py::str("input_amplitude"))?, py::getitem(&v_inputs.clone(), &Py::str("input_ratio"))?, v_n.clone())?;
            v_worst = f__resid(v_outputs.clone(), v_want.clone())?;
            v_rec = f__de_recursion(f__vec(py::getitem(&v_inputs.clone(), &Py::str("a"))?, Py::str("a"), Py::Int(2i64), Py::Int(7i64))?, f__vec(py::getitem(&v_inputs.clone(), &Py::str("b"))?, Py::str("b"), Py::Int(1i64), Py::Int(7i64))?, f__vec(py::getitem(&v_inputs.clone(), &Py::str("initial_conditions"))?, Py::str("initial conditions"), Py::Int(1i64), Py::Int(6i64))?, py::float(&py::getitem(&v_inputs.clone(), &Py::str("input_amplitude"))?)?, py::float(&py::getitem(&v_inputs.clone(), &Py::str("input_ratio"))?)?, py::int(&v_n.clone())?)?;
            v_worst = py::minmax(&[v_worst.clone(), f__zdev(py::getitem(&v_outputs.clone(), &Py::str("samples"))?, v_rec.clone())?], true)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1303 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_iz(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_aa: Py = Py::None;
    let mut v_bb: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_rec: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2485: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_n = (if ({ let t2442 = Py::str("n"); Py::Bool({ let t2443 = v_inputs.clone(); if py::contains(&t2443, &t2442)? { true } else { false } }) }).truthy() { py::getitem(&v_inputs.clone(), &Py::str("n"))? } else { Py::Int(8i64) });
            v_want = f_inverse_z_transform(py::getitem(&v_inputs.clone(), &Py::str("b"))?, py::getitem(&v_inputs.clone(), &Py::str("a"))?, v_n.clone())?;
            v_worst = f__resid(v_outputs.clone(), v_want.clone())?;
            let t2471 = (Py::tuple(vec![f__vec(py::getitem(&v_inputs.clone(), &Py::str("b"))?, Py::str("b"), Py::Int(1i64), Py::Int(7i64))?, f__vec(py::getitem(&v_inputs.clone(), &Py::str("a"))?, Py::str("a"), Py::Int(1i64), Py::Int(7i64))?])).items()?;
            if t2471.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_bb = t2471[0].clone();
            v_aa = t2471[1].clone();
            v_rec = f__lfilter(v_bb.clone(), v_aa.clone(), py::add(&Py::list(vec![Py::Float(1.0)]), &py::mul(&Py::list(vec![Py::Float(0.0)]), &py::sub(&py::int(&v_n.clone())?, &Py::Int(1i64))?)?)?)?;
            v_worst = py::minmax(&[v_worst.clone(), f__zdev(py::getitem(&v_outputs.clone(), &Py::str("samples"))?, v_rec.clone())?], true)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2485 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__zc12(mut v_z: Py, mut v_scale: Py) -> R {
    let mut v_im: Py = Py::None;
    let mut v_re_: Py = Py::None;
    v_z = py::complex(&[v_z.clone()])?;
    v_re_ = (if ({ let t2413 = py::abs(&py::real(&v_z.clone())?)?; Py::Bool({ let t2414 = py::mul(&Py::Float(1e-12), &v_scale.clone())?; if py::gt(&t2413, &t2414)? { true } else { false } }) }).truthy() { py::real(&v_z.clone())? } else { Py::Float(0.0) });
    v_im = (if ({ let t2415 = py::abs(&py::imag(&v_z.clone())?)?; Py::Bool({ let t2416 = py::mul(&Py::Float(1e-12), &v_scale.clone())?; if py::gt(&t2415, &t2416)? { true } else { false } }) }).truthy() { py::imag(&v_z.clone())? } else { Py::Float(0.0) });
    if ({ let t2417 = v_im.clone(); Py::Bool({ let t2418 = Py::Float(0.0); if py::eq(&t2417, &t2418) { true } else { false } }) }).truthy() {
        return Ok(f__zr12(v_re_.clone())?);
    }
    return Ok(py::complex(&[f__zr12(v_re_.clone())?, f__zr12(v_im.clone())?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__zdev(mut v_s: Py, mut v_rec: Py) -> R {
    let mut v_m: Py = Py::None;
    let mut v_scale: Py = Py::None;
    let mut v_worst: Py = Py::None;
    if ({ let t2481 = py::len(&v_s.clone())?; Py::Bool({ let t2482 = py::len(&v_rec.clone())?; if py::ne(&t2481, &t2482) { true } else { false } }) }).truthy() {
        return Ok(c_INF()?);
    }
    v_scale = py::minmax(&[py::add(&{ let mut out = vec![]; for t2483 in py::iter(&(v_rec.clone()))? { let mut v_v: Py; 
v_v = t2483;
out.push(py::abs(&v_v.clone())?); } Py::list(out) }, &Py::list(vec![Py::Float(1e-300)]))?], true)?;
    v_worst = Py::Float(0.0);
    for t2484 in py::range_iter(&[(py::len(&v_rec.clone())?).i()?]) {
        v_m = t2484;
        v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::abs(&py::sub(&py::getitem(&v_s.clone(), &v_m.clone())?, &py::getitem(&v_rec.clone(), &v_m.clone())?)?)?, &v_scale.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__zfmt(mut v_v: Py) -> R {
    let mut v_s: Py = Py::None;
    v_s = Py::str(&[py::format(&f__zr12(v_v.clone())?, ".6g")?].concat());
    return Ok((if ({ let t2431 = v_s.clone(); Py::Bool({ let t2432 = Py::str("-0"); if py::eq(&t2431, &t2432) { true } else { false } }) }).truthy() { Py::str("0") } else { v_s.clone() }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__zform(mut v_lhs: Py, mut v_P: Py, mut v_Rr: Py, mut v_Kr: Py) -> R {
    let mut v_amp: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_terms: Py = Py::None;
    v_terms = Py::list(vec![]);
    for t2428 in py::range_iter(&[(py::len(&v_Kr.clone())?).i()?]) {
        v_j = t2428;
        if ({ let t2429 = py::getitem(&v_Kr.clone(), &v_j.clone())?; Py::Bool({ let t2430 = Py::Int(0i64); if py::ne(&t2429, &t2430) { true } else { false } }) }).truthy() {
            let _ = py::method(&v_terms.clone(), "append", &[py::add(&f__zfmt(py::getitem(&v_Kr.clone(), &v_j.clone())?)?, &(if ({ let t2433 = v_j.clone(); Py::Bool({ let t2434 = Py::Int(0i64); if py::eq(&t2433, &t2434) { true } else { false } }) }).truthy() { Py::str("*delta[n]") } else { py::add(&py::add(&Py::str("*delta[n-"), &Py::str(&py::to_str(&v_j.clone())))?, &Py::str("]"))? }))?])?;
        }
    }
    for t2435 in py::range_iter(&[(py::len(&v_P.clone())?).i()?]) {
        v_i = t2435;
        let t2436 = (Py::tuple(vec![py::complex(&[py::getitem(&v_P.clone(), &v_i.clone())?])?, py::complex(&[py::getitem(&v_Rr.clone(), &v_i.clone())?])?])).items()?;
        if t2436.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_p = t2436[0].clone();
        v_r = t2436[1].clone();
        if ({ let t2437 = py::imag(&v_p.clone())?; Py::Bool({ let t2438 = Py::Int(0i64); if py::eq(&t2437, &t2438) { true } else { false } }) }).truthy() {
            let _ = py::method(&v_terms.clone(), "append", &[py::add(&py::add(&py::add(&f__zfmt(py::real(&v_r.clone())?)?, &Py::str("*("))?, &f__zfmt(py::real(&v_p.clone())?)?)?, &Py::str(")^n"))?])?;
        } else {
            if ({ let t2439 = py::imag(&v_p.clone())?; Py::Bool({ let t2440 = Py::Int(0i64); if py::gt(&t2439, &t2440)? { true } else { false } }) }).truthy() {
                v_amp = py::mul(&Py::Int(2i64), &py::hypot(&[py::real(&v_r.clone())?, py::imag(&v_r.clone())?])?)?;
                let _ = py::method(&v_terms.clone(), "append", &[py::add(&py::add(&py::add(&py::add(&py::add(&py::add(&py::add(&f__zfmt(v_amp.clone())?, &Py::str("*("))?, &f__zfmt(py::hypot(&[py::real(&v_p.clone())?, py::imag(&v_p.clone())?])?)?)?, &Py::str(")^n*cos("))?, &f__zfmt(py::atan2(&py::imag(&v_p.clone())?, &py::real(&v_p.clone())?)?)?)?, &Py::str("*n + "))?, &f__zfmt(py::atan2(&py::imag(&v_r.clone())?, &py::real(&v_r.clone())?)?)?)?, &Py::str(")"))?])?;
            }
        }
    }
    if (py::not(&v_terms.clone())).truthy() {
        v_terms = Py::list(vec![Py::str("0")]);
    }
    return Ok(py::method(&py::add(&py::add(&py::add(&v_lhs.clone(), &Py::str(" = "))?, &py::method(&Py::str(" + "), "join", &[v_terms.clone()])?)?, &Py::str(", n >= 0"))?, "replace", &[Py::str("+ -"), Py::str("- ")])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__zr12(mut v_x: Py) -> R {
    return Ok(py::add(&py::float(&Py::str(&[py::format(&v_x.clone(), ".12g")?].concat()))?, &Py::Float(0.0))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__zroc(mut v_poles: Py) -> R {
    return Ok(f__zr12(py::minmax(&[py::add(&{ let mut out = vec![]; for t2441 in py::iter(&(v_poles.clone()))? { let mut v_p: Py; 
v_p = t2441;
out.push(py::hypot(&[py::real(&py::complex(&[v_p.clone()])?)?, py::imag(&py::complex(&[v_p.clone()])?)?])?); } Py::list(out) }, &Py::list(vec![Py::Float(0.0)]))?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__zsamples(mut v_poles: Py, mut v_res: Py, mut v_K: Py, mut v_n: Py) -> R {
    let mut v_acc: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_scale: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_out = Py::list(vec![]);
    for t2422 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_m = t2422;
        v_acc = Py::Complex(0.0, 0.0);
        v_scale = Py::Float(0.0);
        for t2423 in py::range_iter(&[(py::len(&v_poles.clone())?).i()?]) {
            v_i = t2423;
            v_t = py::mul(&py::getitem(&v_res.clone(), &v_i.clone())?, &py::pow(&py::getitem(&v_poles.clone(), &v_i.clone())?, &v_m.clone())?)?;
            v_acc = py::iadd(&v_acc, &v_t.clone())?;
            v_scale = py::iadd(&v_scale, &py::abs(&v_t.clone())?)?;
        }
        if ({ let t2424 = v_m.clone(); Py::Bool({ let t2425 = py::len(&v_K.clone())?; if py::lt(&t2424, &t2425)? { true } else { false } }) }).truthy() {
            v_acc = py::iadd(&v_acc, &py::getitem(&v_K.clone(), &v_m.clone())?)?;
            v_scale = py::iadd(&v_scale, &py::abs(&py::getitem(&v_K.clone(), &v_m.clone())?)?)?;
        }
        v_v = py::real(&v_acc.clone())?;
        let _ = py::method(&v_out.clone(), "append", &[(if ({ let t2426 = py::abs(&v_v.clone())?; Py::Bool({ let t2427 = py::mul(&Py::Float(1e-12), &v_scale.clone())?; if py::le(&t2426, &t2427)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { f__zr12(v_v.clone())? })])?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_alias_frequency(mut v_f: Py, mut v_fs: Py) -> R {
    let mut v_r: Py = Py::None;
    let t11 = (Py::tuple(vec![f__nonneg(v_f.clone(), Py::str("signal frequency"))?, f__pos(v_fs.clone(), Py::str("sampling rate"))?])).items()?;
    if t11.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_f = t11[0].clone();
    v_fs = t11[1].clone();
    v_r = py::fmod(&v_f.clone(), &v_fs.clone())?;
    return Ok(Py::dict(vec![(Py::str("f_alias"), py::minmax(&[v_r.clone(), py::sub(&v_fs.clone(), &v_r.clone())?], false)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_am_modulation_index(mut v_Vmax: Py, mut v_Vmin: Py) -> R {
    let t52 = (Py::tuple(vec![f__pos(v_Vmax.clone(), Py::str("maximum envelope"))?, f__nonneg(v_Vmin.clone(), Py::str("minimum envelope"))?])).items()?;
    if t52.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Vmax = t52[0].clone();
    v_Vmin = t52[1].clone();
    if ({ let t53 = v_Vmin.clone(); Py::Bool({ let t54 = v_Vmax.clone(); if py::ge(&t53, &t54)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the maximum of the envelope must exceed its minimum"))));
    }
    return Ok(Py::dict(vec![(Py::str("m"), py::truediv(&py::sub(&v_Vmax.clone(), &v_Vmin.clone())?, &py::add(&v_Vmax.clone(), &v_Vmin.clone())?)?), (Py::str("Ac"), py::truediv(&py::add(&v_Vmax.clone(), &v_Vmin.clone())?, &Py::Int(2i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_am_power(mut v_Pc: Py, mut v_m: Py) -> R {
    let t90 = (Py::tuple(vec![f__pos(v_Pc.clone(), Py::str("carrier power"))?, f__pos(v_m.clone(), Py::str("modulation index"))?])).items()?;
    if t90.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Pc = t90[0].clone();
    v_m = t90[1].clone();
    if ({ let t91 = v_m.clone(); Py::Bool({ let t92 = Py::Int(1i64); if py::gt(&t91, &t92)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the modulation index must be at most 1 (no overmodulation)"))));
    }
    return Ok(Py::dict(vec![(Py::str("Pt"), py::mul(&v_Pc.clone(), &py::add(&Py::Int(1i64), &py::truediv(&py::mul(&v_m.clone(), &v_m.clone())?, &Py::Int(2i64))?)?)?), (Py::str("Psb"), py::truediv(&py::mul(&py::mul(&v_Pc.clone(), &v_m.clone())?, &v_m.clone())?, &Py::Int(2i64))?), (Py::str("Pusb"), py::truediv(&py::mul(&py::mul(&v_Pc.clone(), &v_m.clone())?, &v_m.clone())?, &Py::Int(4i64))?), (Py::str("efficiency"), py::truediv(&py::mul(&v_m.clone(), &v_m.clone())?, &py::add(&Py::Int(2i64), &py::mul(&v_m.clone(), &v_m.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_amplitude_ratio_to_db(mut v_ratio: Py) -> R {
    return Ok(Py::dict(vec![(Py::str("dB"), f__db20(f__pos(v_ratio.clone(), Py::str("amplitude ratio"))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_analytic_envelope(mut v_x: Py) -> R {
    let mut v_N: Py = Py::None;
    let mut v_X: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_z: Py = Py::None;
    v_x = f__vec(v_x.clone(), Py::str("signal x"), Py::Int(1i64), Py::Int(65536i64))?;
    let t171 = (Py::tuple(vec![py::len(&v_x.clone())?, py::fsum(&{ let mut out = vec![]; for t170 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t170;
out.push(py::abs(&v_v.clone())?); } Py::list(out) })?])).items()?;
    if t171.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_N = t171[0].clone();
    v_s = t171[1].clone();
    v_X = f__fft(v_x.clone(), py::neg(&Py::Int(1i64))?)?;
    v_h = py::mul(&Py::list(vec![Py::Float(0.0)]), &v_N.clone())?;
    py::setitem(&v_h.clone(), &Py::Int(0i64), Py::Float(1.0))?;
    if ({ let t186 = py::modulo(&v_N.clone(), &Py::Int(2i64))?; Py::Bool({ let t187 = Py::Int(0i64); if py::eq(&t186, &t187) { true } else { false } }) }).truthy() {
        py::setitem(&v_h.clone(), &py::floordiv(&v_N.clone(), &Py::Int(2i64))?, Py::Float(1.0))?;
        for t188 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::floordiv(&v_N.clone(), &Py::Int(2i64))?).i()?]) {
            v_k = t188;
            py::setitem(&v_h.clone(), &v_k.clone(), Py::Float(2.0))?;
        }
    } else {
        for t189 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::floordiv(&py::add(&v_N.clone(), &Py::Int(1i64))?, &Py::Int(2i64))?).i()?]) {
            v_k = t189;
            py::setitem(&v_h.clone(), &v_k.clone(), Py::Float(2.0))?;
        }
    }
    v_z = { let mut out = vec![]; for t192 in py::iter(&(f__fft({ let mut out = vec![]; for t190 in py::iter(&(py::zip(&[v_X.clone(), v_h.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t191 = (t190).items()?;
if t191.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t191[0].clone();
v_b = t191[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) }, py::pos(&Py::Int(1i64))?)?))? { let mut v_v: Py; 
v_v = t192;
out.push(f__snap(py::truediv(&v_v.clone(), &v_N.clone())?, v_s.clone())?); } Py::list(out) };
    return Ok(Py::dict(vec![(Py::str("envelope"), { let mut out = vec![]; for t197 in py::iter(&(v_z.clone()))? { let mut v_v: Py; 
v_v = t197;
out.push(py::abs(&v_v.clone())?); } Py::list(out) }), (Py::str("hilbert"), { let mut out = vec![]; for t198 in py::iter(&(v_z.clone()))? { let mut v_v: Py; 
v_v = t198;
out.push(py::imag(&v_v.clone())?); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bilinear_rc_lowpass(mut v_fc: Py, mut v_fs: Py) -> R {
    let mut v_K: Py = Py::None;
    let mut v_wc: Py = Py::None;
    let t234 = (Py::tuple(vec![f__pos(v_fc.clone(), Py::str("cutoff frequency"))?, f__pos(v_fs.clone(), Py::str("sampling rate"))?])).items()?;
    if t234.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fc = t234[0].clone();
    v_fs = t234[1].clone();
    let t235 = (Py::tuple(vec![py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_fc.clone())?, py::mul(&Py::Int(2i64), &v_fs.clone())?])).items()?;
    if t235.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_wc = t235[0].clone();
    v_K = t235[1].clone();
    return Ok(Py::dict(vec![(Py::str("b0"), py::truediv(&v_wc.clone(), &py::add(&v_wc.clone(), &v_K.clone())?)?), (Py::str("b1"), py::truediv(&v_wc.clone(), &py::add(&v_wc.clone(), &v_K.clone())?)?), (Py::str("a1"), py::truediv(&py::sub(&v_wc.clone(), &v_K.clone())?, &py::add(&v_wc.clone(), &v_K.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bin_frequency(mut v_k: Py, mut v_fs: Py, mut v_N: Py) -> R {
    v_N = f__int(v_N.clone(), Py::str("number of points"), Py::Int(2i64), Py::None)?;
    v_k = f__int(v_k.clone(), Py::str("bin index"), Py::Int(0i64), py::floordiv(&v_N.clone(), &Py::Int(2i64))?)?;
    v_fs = f__pos(v_fs.clone(), Py::str("sampling rate"))?;
    return Ok(Py::dict(vec![(Py::str("f_k"), py::truediv(&py::mul(&v_k.clone(), &v_fs.clone())?, &v_N.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bpsk_ber(mut v_ebn0_db: Py) -> R {
    let mut v_g: Py = Py::None;
    v_g = py::pow(&Py::Int(10i64), &py::truediv(&f__fin(v_ebn0_db.clone(), Py::str("Eb/N0 in dB"))?, &Py::Int(10i64))?)?;
    return Ok(Py::dict(vec![(Py::str("ber"), py::mul(&Py::Float(0.5), &py::math1("erfc", &py::math1("sqrt", &v_g.clone())?)?)?), (Py::str("ber_bfsk"), py::mul(&Py::Float(0.5), &py::math1("erfc", &py::math1("sqrt", &py::truediv(&v_g.clone(), &Py::Int(2i64))?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_butterworth_design(mut v_N: Py, mut v_fc: Py, mut v_fs: Py, mut v_btype: Py) -> R {
    let mut v_den: Py = Py::None;
    let mut v_fs2: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_kz: Py = Py::None;
    let mut v_num: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_pz: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_wo: Py = Py::None;
    let mut v_z: Py = Py::None;
    let mut v_zz: Py = Py::None;
    v_N = f__int(v_N.clone(), Py::str("order"), Py::Int(1i64), Py::Int(8i64))?;
    let t368 = (Py::tuple(vec![f__pos(v_fc.clone(), Py::str("cutoff frequency"))?, f__pos(v_fs.clone(), Py::str("sampling rate"))?])).items()?;
    if t368.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fc = t368[0].clone();
    v_fs = t368[1].clone();
    if ({ let t369 = v_btype.clone(); Py::Bool({ let t370 = Py::tuple(vec![Py::str("lowpass"), Py::str("highpass")]); if !py::contains(&t370, &t369)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("btype must be lowpass or highpass"))));
    }
    if ({ let t371 = v_fc.clone(); Py::Bool({ let t372 = py::truediv(&v_fs.clone(), &Py::Int(2i64))?; if py::ge(&t371, &t372)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the cutoff must lie below fs/2"))));
    }
    v_fs2 = py::mul(&Py::Int(2i64), &v_fs.clone())?;
    v_wo = py::mul(&v_fs2.clone(), &py::math1("tan", &py::truediv(&py::mul(&Py::Float(std::f64::consts::PI), &v_fc.clone())?, &v_fs.clone())?)?)?;
    v_p = { let mut out = vec![]; for t373 in py::range_iter(&[(py::add(&py::neg(&v_N.clone())?, &Py::Int(1i64))?).i()?, (v_N.clone()).i()?, (Py::Int(2i64)).i()?]) { let mut v_m: Py; 
v_m = t373;
out.push(py::neg(&py::cmath("exp", &[py::truediv(&py::mul(&py::mul(&Py::Complex(0.0, 1.0), &Py::Float(std::f64::consts::PI))?, &v_m.clone())?, &py::mul(&Py::Int(2i64), &v_N.clone())?)?])?)?); } Py::list(out) };
    if ({ let t374 = v_btype.clone(); Py::Bool({ let t375 = Py::str("lowpass"); if py::eq(&t374, &t375) { true } else { false } }) }).truthy() {
        v_p = { let mut out = vec![]; for t376 in py::iter(&(v_p.clone()))? { let mut v_v: Py; 
v_v = t376;
out.push(py::mul(&v_wo.clone(), &v_v.clone())?); } Py::list(out) };
        v_k = py::pow(&v_wo.clone(), &v_N.clone())?;
        v_z = Py::list(vec![]);
    } else {
        v_k = Py::Float(1.0);
        for t377 in py::iter(&(v_p.clone()))? {
            v_v = t377;
            v_k = py::truediv(&v_k, &py::neg(&v_v.clone())?)?;
        }
        v_k = py::real(&v_k.clone())?;
        v_p = { let mut out = vec![]; for t378 in py::iter(&(v_p.clone()))? { let mut v_v: Py; 
v_v = t378;
out.push(py::truediv(&v_wo.clone(), &v_v.clone())?); } Py::list(out) };
        v_z = py::mul(&Py::list(vec![Py::Complex(0.0, 0.0)]), &v_N.clone())?;
    }
    v_pz = { let mut out = vec![]; for t379 in py::iter(&(v_p.clone()))? { let mut v_v: Py; 
v_v = t379;
out.push(py::truediv(&py::add(&v_fs2.clone(), &v_v.clone())?, &py::sub(&v_fs2.clone(), &v_v.clone())?)?); } Py::list(out) };
    v_zz = py::add(&{ let mut out = vec![]; for t380 in py::iter(&(v_z.clone()))? { let mut v_v: Py; 
v_v = t380;
out.push(py::truediv(&py::add(&v_fs2.clone(), &v_v.clone())?, &py::sub(&v_fs2.clone(), &v_v.clone())?)?); } Py::list(out) }, &py::mul(&Py::list(vec![py::add(&py::neg(&Py::Int(1i64))?, &Py::Complex(0.0, 0.0))?]), &py::sub(&py::len(&v_p.clone())?, &py::len(&v_z.clone())?)?)?)?;
    v_num = py::add(&Py::Int(1i64), &Py::Complex(0.0, 0.0))?;
    for t381 in py::iter(&(v_z.clone()))? {
        v_v = t381;
        v_num = py::mul(&v_num, &py::sub(&v_fs2.clone(), &v_v.clone())?)?;
    }
    v_den = py::add(&Py::Int(1i64), &Py::Complex(0.0, 0.0))?;
    for t382 in py::iter(&(v_p.clone()))? {
        v_v = t382;
        v_den = py::mul(&v_den, &py::sub(&v_fs2.clone(), &v_v.clone())?)?;
    }
    v_kz = py::mul(&v_k.clone(), &py::real(&py::truediv(&v_num.clone(), &v_den.clone())?)?)?;
    return Ok(Py::dict(vec![(Py::str("b"), { let mut out = vec![]; for t386 in py::iter(&(f__cpoly(v_zz.clone())?))? { let mut v_v: Py; 
v_v = t386;
out.push(f__r12(py::real(&py::mul(&v_kz.clone(), &v_v.clone())?)?)?); } Py::list(out) }), (Py::str("a"), { let mut out = vec![]; for t389 in py::iter(&(f__cpoly(v_pz.clone())?))? { let mut v_v: Py; 
v_v = t389;
out.push(f__r12(py::real(&v_v.clone())?)?); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_butterworth_magnitude(mut v_N: Py, mut v_fc: Py, mut v_f: Py) -> R {
    let mut v_r: Py = Py::None;
    v_N = f__int(v_N.clone(), Py::str("order"), Py::Int(1i64), Py::Int(30i64))?;
    let t445 = (Py::tuple(vec![f__pos(v_fc.clone(), Py::str("cutoff frequency"))?, f__nonneg(v_f.clone(), Py::str("frequency"))?])).items()?;
    if t445.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fc = t445[0].clone();
    v_f = t445[1].clone();
    v_r = py::pow(&py::truediv(&v_f.clone(), &v_fc.clone())?, &py::mul(&Py::Int(2i64), &v_N.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("magnitude"), py::truediv(&Py::Int(1i64), &py::math1("sqrt", &py::add(&Py::Int(1i64), &v_r.clone())?)?)?), (Py::str("attenuation_db"), py::truediv(&py::mul(&Py::Int(10i64), &py::math1("log1p", &v_r.clone())?)?, &py::math1("log", &Py::Int(10i64))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_butterworth_order_analog(mut v_fp: Py, mut v_fstop: Py, mut v_Rp: Py, mut v_As: Py) -> R {
    let mut v_low: Py = Py::None;
    let t487 = (f__order_args(v_fp.clone(), v_fstop.clone(), v_Rp.clone(), v_As.clone())?).items()?;
    if t487.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_fp = t487[0].clone();
    v_fstop = t487[1].clone();
    v_Rp = t487[2].clone();
    v_As = t487[3].clone();
    v_low = { let t488 = v_fp.clone(); Py::Bool({ let t489 = v_fstop.clone(); if py::lt(&t488, &t489)? { true } else { false } }) };
    return Ok(Py::dict(vec![(Py::str("order"), f__butter_ord((if (v_low.clone()).truthy() { py::truediv(&v_fstop.clone(), &v_fp.clone())? } else { py::truediv(&v_fp.clone(), &v_fstop.clone())? }), v_Rp.clone(), v_As.clone())?), (Py::str("type"), (if (v_low.clone()).truthy() { Py::str("lowpass") } else { Py::str("highpass") }))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_butterworth_order_digital(mut v_fp: Py, mut v_fstop: Py, mut v_fs: Py, mut v_Rp: Py, mut v_As: Py) -> R {
    let mut v_low: Py = Py::None;
    let mut v_wp: Py = Py::None;
    let mut v_ws: Py = Py::None;
    let t532 = (f__order_args(v_fp.clone(), v_fstop.clone(), v_Rp.clone(), v_As.clone())?).items()?;
    if t532.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_fp = t532[0].clone();
    v_fstop = t532[1].clone();
    v_Rp = t532[2].clone();
    v_As = t532[3].clone();
    v_fs = f__pos(v_fs.clone(), Py::str("sampling rate"))?;
    if ({ let t533 = py::minmax(&[v_fp.clone(), v_fstop.clone()], true)?; Py::Bool({ let t534 = py::truediv(&v_fs.clone(), &Py::Int(2i64))?; if py::ge(&t533, &t534)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("both edge frequencies must lie below fs/2"))));
    }
    let t535 = (Py::tuple(vec![py::math1("tan", &py::truediv(&py::mul(&Py::Float(std::f64::consts::PI), &v_fp.clone())?, &v_fs.clone())?)?, py::math1("tan", &py::truediv(&py::mul(&Py::Float(std::f64::consts::PI), &v_fstop.clone())?, &v_fs.clone())?)?])).items()?;
    if t535.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_wp = t535[0].clone();
    v_ws = t535[1].clone();
    v_low = { let t536 = v_fp.clone(); Py::Bool({ let t537 = v_fstop.clone(); if py::lt(&t536, &t537)? { true } else { false } }) };
    return Ok(Py::dict(vec![(Py::str("order"), f__butter_ord((if (v_low.clone()).truthy() { py::truediv(&v_ws.clone(), &v_wp.clone())? } else { py::truediv(&v_wp.clone(), &v_ws.clone())? }), v_Rp.clone(), v_As.clone())?), (Py::str("type"), (if (v_low.clone()).truthy() { Py::str("lowpass") } else { Py::str("highpass") }))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_chebyshev1_order_analog(mut v_fp: Py, mut v_fstop: Py, mut v_Rp: Py, mut v_As: Py) -> R {
    let mut v_low: Py = Py::None;
    let t580 = (f__order_args(v_fp.clone(), v_fstop.clone(), v_Rp.clone(), v_As.clone())?).items()?;
    if t580.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_fp = t580[0].clone();
    v_fstop = t580[1].clone();
    v_Rp = t580[2].clone();
    v_As = t580[3].clone();
    v_low = { let t581 = v_fp.clone(); Py::Bool({ let t582 = v_fstop.clone(); if py::lt(&t581, &t582)? { true } else { false } }) };
    return Ok(Py::dict(vec![(Py::str("order"), f__cheb_ord((if (v_low.clone()).truthy() { py::truediv(&v_fstop.clone(), &v_fp.clone())? } else { py::truediv(&v_fp.clone(), &v_fstop.clone())? }), v_Rp.clone(), v_As.clone())?), (Py::str("type"), (if (v_low.clone()).truthy() { Py::str("lowpass") } else { Py::str("highpass") }))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_circular_convolution(mut v_x: Py, mut v_h: Py) -> R {
    let mut v_N: Py = Py::None;
    let t626 = (Py::tuple(vec![f__vec(v_x.clone(), Py::str("x"), Py::Int(1i64), Py::Int(20000i64))?, f__vec(v_h.clone(), Py::str("h"), Py::Int(1i64), Py::Int(20000i64))?])).items()?;
    if t626.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t626[0].clone();
    v_h = t626[1].clone();
    if ({ let t627 = py::len(&v_x.clone())?; Py::Bool({ let t628 = py::len(&v_h.clone())?; if py::ne(&t627, &t628) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("circular convolution needs two sequences of the same length N"))));
    }
    v_N = py::len(&v_x.clone())?;
    return Ok(Py::dict(vec![(Py::str("y"), { let mut out = vec![]; for t629 in py::range_iter(&[(v_N.clone()).i()?]) { let mut v_n: Py; 
v_n = t629;
out.push(py::fsum(&{ let mut out = vec![]; for t630 in py::range_iter(&[(v_N.clone()).i()?]) { let mut v_k: Py; 
v_k = t630;
out.push(py::mul(&py::getitem(&v_x.clone(), &v_k.clone())?, &py::getitem(&v_h.clone(), &py::modulo(&py::sub(&v_n.clone(), &v_k.clone())?, &v_N.clone())?)?)?); } Py::list(out) })?); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_coax_impedance(mut v_D: Py, mut v_d: Py, mut v_er: Py) -> R {
    let mut v_eta0: Py = Py::None;
    let t666 = (Py::tuple(vec![f__pos(v_D.clone(), Py::str("outer conductor diameter"))?, f__pos(v_d.clone(), Py::str("inner conductor diameter"))?, f__pos(v_er.clone(), Py::str("relative permittivity"))?])).items()?;
    if t666.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_D = t666[0].clone();
    v_d = t666[1].clone();
    v_er = t666[2].clone();
    if ({ let t667 = v_d.clone(); Py::Bool({ let t668 = v_D.clone(); if py::ge(&t667, &t668)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the inner diameter must be smaller than the outer diameter"))));
    }
    if ({ let t669 = v_er.clone(); Py::Bool({ let t670 = Py::Int(1i64); if py::lt(&t669, &t670)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the relative permittivity is at least 1"))));
    }
    v_eta0 = py::math1("sqrt", &py::truediv(&c_MU0()?, &c_EPS0()?)?)?;
    return Ok(Py::dict(vec![(Py::str("Z0"), py::mul(&py::truediv(&v_eta0.clone(), &py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &py::math1("sqrt", &v_er.clone())?)?)?, &py::math1("log", &py::truediv(&v_D.clone(), &v_d.clone())?)?)?), (Py::str("vf"), py::truediv(&Py::Int(1i64), &py::math1("sqrt", &v_er.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cross_correlation(mut v_x: Py, mut v_y: Py) -> R {
    let mut v_i: Py = Py::None;
    let mut v_lags: Py = Py::None;
    let mut v_r: Py = Py::None;
    let t713 = (Py::tuple(vec![f__vec(v_x.clone(), Py::str("x"), Py::Int(1i64), Py::Int(20000i64))?, f__vec(v_y.clone(), Py::str("y"), Py::Int(1i64), Py::Int(20000i64))?])).items()?;
    if t713.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t713[0].clone();
    v_y = t713[1].clone();
    v_lags = Py::list((Py::list((py::range_iter(&[(py::neg(&py::sub(&py::len(&v_y.clone())?, &Py::Int(1i64))?)?).i()?, (py::len(&v_x.clone())?).i()?])).into_iter().collect::<Vec<Py>>())).items()?);
    v_r = { let mut out = vec![]; for t714 in py::iter(&(v_lags.clone()))? { let mut v_L: Py; 
v_L = t714;
out.push(py::fsum(&{ let mut out = vec![]; for t715 in py::range_iter(&[(py::minmax(&[Py::Int(0i64), py::neg(&v_L.clone())?], true)?).i()?, (py::minmax(&[py::len(&v_y.clone())?, py::sub(&py::len(&v_x.clone())?, &v_L.clone())?], false)?).i()?]) { let mut v_n: Py; 
v_n = t715;
out.push(py::mul(&py::getitem(&v_x.clone(), &py::add(&v_n.clone(), &v_L.clone())?)?, &py::getitem(&v_y.clone(), &v_n.clone())?)?); } Py::list(out) })?); } Py::list(out) };
    v_i = f__peak(v_r.clone(), py::minmax(&[{ let mut out = vec![]; for t721 in py::iter(&(v_r.clone()))? { let mut v_v: Py; 
v_v = t721;
out.push(py::abs(&v_v.clone())?); } Py::list(out) }], true)?)?;
    return Ok(Py::dict(vec![(Py::str("peak_lag"), py::getitem(&v_lags.clone(), &v_i.clone())?), (Py::str("peak_value"), py::getitem(&v_r.clone(), &v_i.clone())?), (Py::str("r"), v_r.clone()), (Py::str("lags"), v_lags.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_db_to_amplitude_ratio(mut v_dB: Py) -> R {
    return Ok(Py::dict(vec![(Py::str("ratio"), py::pow(&Py::Float(10.0), &py::truediv(&f__fin(v_dB.clone(), Py::str("level in dB"))?, &Py::Float(20.0))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_db_to_power_ratio(mut v_dB: Py) -> R {
    return Ok(Py::dict(vec![(Py::str("ratio"), py::pow(&Py::Float(10.0), &py::truediv(&f__fin(v_dB.clone(), Py::str("level in dB"))?, &Py::Float(10.0))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dbm_power_sum(mut v_levels: Py) -> R {
    let mut v_L: Py = Py::None;
    let mut v_p: Py = Py::None;
    v_L = f__vec(v_levels.clone(), Py::str("power levels in dBm"), Py::Int(1i64), Py::None)?;
    v_p = py::mul(&py::fsum(&{ let mut out = vec![]; for t828 in py::iter(&(v_L.clone()))? { let mut v_v: Py; 
v_v = t828;
out.push(py::pow(&Py::Float(10.0), &py::truediv(&v_v.clone(), &Py::Float(10.0))?)?); } Py::list(out) })?, &Py::Float(0.001))?;
    return Ok(Py::dict(vec![(Py::str("total_dBm"), f__db10(py::truediv(&v_p.clone(), &Py::Float(0.001))?)?), (Py::str("total_W"), v_p.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dbm_to_watts(mut v_dBm: Py) -> R {
    v_dBm = f__fin(v_dBm.clone(), Py::str("power level in dBm"))?;
    return Ok(Py::dict(vec![(Py::str("P"), py::mul(&Py::Float(0.001), &py::pow(&Py::Float(10.0), &py::truediv(&v_dBm.clone(), &Py::Float(10.0))?)?)?), (Py::str("dBW"), py::sub(&v_dBm.clone(), &Py::Float(30.0))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dc_gain(mut v_b: Py, mut v_a: Py) -> R {
    let mut v_g: Py = Py::None;
    let mut v_sa: Py = Py::None;
    v_b = f__vec(v_b.clone(), Py::str("b"), Py::Int(1i64), Py::Int(10000i64))?;
    v_a = (if ({ let t903 = v_a.clone(); Py::Bool({ let t904 = Py::None; if py::is(&t903, &t904) { true } else { false } }) }).truthy() { Py::list(vec![Py::Float(1.0)]) } else { f__vec(v_a.clone(), Py::str("a"), Py::Int(1i64), Py::Int(10000i64))? });
    v_sa = py::fsum(&v_a.clone())?;
    if ({ let t905 = v_sa.clone(); Py::Bool({ let t906 = Py::Int(0i64); if py::eq(&t905, &t906) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the sum of the denominator coefficients is 0 (a pole at z = 1): the DC gain is infinite"))));
    }
    v_g = py::truediv(&py::fsum(&v_b.clone())?, &v_sa.clone())?;
    return Ok(Py::dict(vec![(Py::str("gain"), v_g.clone()), (Py::str("gain_db"), (if ({ let t907 = v_g.clone(); Py::Bool({ let t908 = Py::Int(0i64); if py::ne(&t907, &t908) { true } else { false } }) }).truthy() { f__db20(py::abs(&v_g.clone())?)? } else { py::neg(&c_INF()?)? }))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dft_bin(mut v_x: Py, mut v_k: Py) -> R {
    let mut v_N: Py = Py::None;
    let mut v_X: Py = Py::None;
    let mut v_t: Py = Py::None;
    v_x = f__vec(v_x.clone(), Py::str("signal x"), Py::Int(1i64), Py::Int(65536i64))?;
    v_N = py::len(&v_x.clone())?;
    v_k = f__int(v_k.clone(), Py::str("bin index k"), Py::Int(0i64), py::sub(&v_N.clone(), &Py::Int(1i64))?)?;
    v_t = { let mut out = vec![]; for t966 in py::range_iter(&[(v_N.clone()).i()?]) { let mut v_n: Py; 
v_n = t966;
out.push(py::mul(&py::getitem(&v_x.clone(), &v_n.clone())?, &f__tw(py::mul(&v_k.clone(), &v_n.clone())?, v_N.clone(), py::neg(&Py::Int(1i64))?)?)?); } Py::list(out) };
    v_X = f__snap(py::complex(&[py::fsum(&{ let mut out = vec![]; for t973 in py::iter(&(v_t.clone()))? { let mut v_z: Py; 
v_z = t973;
out.push(py::real(&v_z.clone())?); } Py::list(out) })?, py::fsum(&{ let mut out = vec![]; for t974 in py::iter(&(v_t.clone()))? { let mut v_z: Py; 
v_z = t974;
out.push(py::imag(&v_z.clone())?); } Py::list(out) })?])?, py::fsum(&{ let mut out = vec![]; for t975 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t975;
out.push(py::abs(&v_v.clone())?); } Py::list(out) })?)?;
    return Ok(Py::dict(vec![(Py::str("magnitude"), py::abs(&v_X.clone())?), (Py::str("phase"), py::atan2(&py::imag(&v_X.clone())?, &py::real(&v_X.clone())?)?), (Py::str("re"), py::real(&v_X.clone())?), (Py::str("im"), py::imag(&v_X.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dft_spectrum(mut v_x: Py) -> R {
    let mut v_X: Py = Py::None;
    let mut v_kp: Py = Py::None;
    let mut v_mags: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_x = f__vec(v_x.clone(), Py::str("signal x"), Py::Int(1i64), Py::Int(65536i64))?;
    v_s = py::fsum(&{ let mut out = vec![]; for t1018 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t1018;
out.push(py::abs(&v_v.clone())?); } Py::list(out) })?;
    v_X = { let mut out = vec![]; for t1033 in py::iter(&(f__fft(v_x.clone(), py::neg(&Py::Int(1i64))?)?))? { let mut v_z: Py; 
v_z = t1033;
out.push(f__snap(v_z.clone(), v_s.clone())?); } Py::list(out) };
    v_mags = { let mut out = vec![]; for t1038 in py::iter(&(v_X.clone()))? { let mut v_z: Py; 
v_z = t1038;
out.push(py::abs(&v_z.clone())?); } Py::list(out) };
    v_kp = f__peak(py::slice(&v_mags.clone(), None, Some((py::add(&py::floordiv(&py::len(&v_x.clone())?, &Py::Int(2i64))?, &Py::Int(1i64))?).i()?), None)?, v_s.clone())?;
    return Ok(Py::dict(vec![(Py::str("peak_magnitude"), py::getitem(&v_mags.clone(), &v_kp.clone())?), (Py::str("peak_bin"), v_kp.clone()), (Py::str("magnitude"), v_mags.clone()), (Py::str("phase"), { let mut out = vec![]; for t1044 in py::iter(&(v_X.clone()))? { let mut v_z: Py; 
v_z = t1044;
out.push(py::atan2(&py::imag(&v_z.clone())?, &py::real(&v_z.clone())?)?); } Py::list(out) }), (Py::str("real"), { let mut out = vec![]; for t1045 in py::iter(&(v_X.clone()))? { let mut v_z: Py; 
v_z = t1045;
out.push(py::real(&v_z.clone())?); } Py::list(out) }), (Py::str("imag"), { let mut out = vec![]; for t1046 in py::iter(&(v_X.clone()))? { let mut v_z: Py; 
v_z = t1046;
out.push(py::imag(&v_z.clone())?); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_difference_equation_z(mut v_a: Py, mut v_b: Py, mut v_initial_conditions: Py, mut v_input_amplitude: Py, mut v_input_ratio: Py, mut v_n: Py) -> R {
    let mut v_K: Py = Py::None;
    let mut v_Kr: Py = Py::None;
    let mut v_N: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_Rr: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_den: Py = Py::None;
    let mut v_ic: Py = Py::None;
    let mut v_icp: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_num: Py = Py::None;
    let mut v_poles: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_res: Py = Py::None;
    let t1089 = (Py::tuple(vec![f__vec(v_a.clone(), Py::str("output coefficients a"), Py::Int(2i64), Py::Int(7i64))?, f__vec(v_b.clone(), Py::str("input coefficients b"), Py::Int(1i64), Py::Int(7i64))?])).items()?;
    if t1089.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1089[0].clone();
    v_b = t1089[1].clone();
    v_ic = f__vec(v_initial_conditions.clone(), Py::str("initial conditions"), Py::Int(1i64), Py::Int(6i64))?;
    let t1092 = (Py::tuple(vec![f__fin(v_input_amplitude.clone(), Py::str("input amplitude"))?, f__fin(v_input_ratio.clone(), Py::str("input ratio"))?])).items()?;
    if t1092.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c = t1092[0].clone();
    v_q = t1092[1].clone();
    v_n = f__int(v_n.clone(), Py::str("number of samples"), Py::Int(1i64), Py::Int(200i64))?;
    if ({ let t1108 = py::getitem(&v_a.clone(), &Py::Int(0i64))?; Py::Bool({ let t1109 = Py::Int(0i64); if py::eq(&t1108, &t1109) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a[0] (the coefficient of y[n]) must not be 0"))));
    }
    if ({ let t1110 = py::getitem(&v_a.clone(), &py::neg(&Py::Int(1i64))?)?; Py::Bool({ let t1111 = Py::Int(0i64); if py::eq(&t1110, &t1111) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the last coefficient of a must not be 0 (it sets the order of the equation)"))));
    }
    v_N = py::sub(&py::len(&v_a.clone())?, &Py::Int(1i64))?;
    if ({ let t1112 = py::len(&v_ic.clone())?; Py::Bool({ let t1113 = v_N.clone(); if py::ne(&t1112, &t1113) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["the equation is of order ".to_string(), py::format(&v_N.clone(), "")?, ": give ".to_string(), py::format(&v_N.clone(), "")?, " initial conditions y[-1], ..., y[-".to_string(), py::format(&v_N.clone(), "")?, "]".to_string()].concat()))));
    }
    v_icp = py::mul(&Py::list(vec![Py::Float(0.0)]), &v_N.clone())?;
    for t1114 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_N.clone(), &Py::Int(1i64))?).i()?]) {
        v_k = t1114;
        for t1115 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_k.clone(), &Py::Int(1i64))?).i()?]) {
            v_m = t1115;
            let t1116 = v_icp.clone(); let t1117 = py::sub(&v_k.clone(), &v_m.clone())?;
            py::setitem(&t1116, &t1117, py::iadd(&py::getitem(&t1116, &t1117)?, &py::mul(&py::getitem(&v_a.clone(), &v_k.clone())?, &py::getitem(&v_ic.clone(), &py::sub(&v_m.clone(), &Py::Int(1i64))?)?)?)?)?;
        }
    }
    if ({ let t1118 = v_c.clone(); Py::Bool({ let t1119 = Py::Int(0i64); if py::eq(&t1118, &t1119) { true } else { false } }) }).truthy() {
        let t1121 = (Py::tuple(vec![{ let mut out = vec![]; for t1120 in py::iter(&(v_icp.clone()))? { let mut v_v: Py; 
v_v = t1120;
out.push(py::neg(&v_v.clone())?); } Py::list(out) }, Py::list((v_a.clone()).items()?)])).items()?;
        if t1121.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_num = t1121[0].clone();
        v_den = t1121[1].clone();
    } else {
        v_num = py::mul(&Py::list(vec![Py::Float(0.0)]), &py::minmax(&[py::len(&v_b.clone())?, py::add(&v_N.clone(), &Py::Int(1i64))?], true)?)?;
        for t1122 in py::range_iter(&[(py::len(&v_b.clone())?).i()?]) {
            v_k = t1122;
            let t1123 = v_num.clone(); let t1124 = v_k.clone();
            py::setitem(&t1123, &t1124, py::iadd(&py::getitem(&t1123, &t1124)?, &py::mul(&v_c.clone(), &py::getitem(&v_b.clone(), &v_k.clone())?)?)?)?;
        }
        for t1125 in py::range_iter(&[(v_N.clone()).i()?]) {
            v_k = t1125;
            let t1126 = v_num.clone(); let t1127 = v_k.clone();
            py::setitem(&t1126, &t1127, py::sub(&py::getitem(&t1126, &t1127)?, &py::getitem(&v_icp.clone(), &v_k.clone())?)?)?;
            let t1128 = v_num.clone(); let t1129 = py::add(&v_k.clone(), &Py::Int(1i64))?;
            py::setitem(&t1128, &t1129, py::iadd(&py::getitem(&t1128, &t1129)?, &py::mul(&v_q.clone(), &py::getitem(&v_icp.clone(), &v_k.clone())?)?)?)?;
        }
        v_den = f__pmul(v_a.clone(), Py::list(vec![Py::Float(1.0), py::neg(&v_q.clone())?]))?;
    }
    if (Py::Bool(({ let mut out = vec![]; for t1134 in py::iter(&(v_num.clone()))? { let mut v_v: Py; 
v_v = t1134;
out.push({ let t1135 = v_v.clone(); Py::Bool({ let t1136 = Py::Int(0i64); if py::eq(&t1135, &t1136) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the input and the initial conditions are all zero: y[n] = 0 for every n"))));
    }
    let t1242 = (f__pf_expand(v_num.clone(), v_den.clone())?).items()?;
    if t1242.len() != 6 { return Err(PyErr::Value("unpack".into())); }
    v_poles = t1242[0].clone();
    v_res = t1242[1].clone();
    v_K = t1242[2].clone();
    v_P = t1242[3].clone();
    v_Rr = t1242[4].clone();
    v_Kr = t1242[5].clone();
    return Ok(Py::dict(vec![(Py::str("residues"), v_Rr.clone()), (Py::str("poles"), v_P.clone()), (Py::str("direct_terms"), v_Kr.clone()), (Py::str("samples"), f__zsamples(v_poles.clone(), v_res.clone(), v_K.clone(), v_n.clone())?), (Py::str("closed_form"), f__zform(Py::str("y[n]"), v_P.clone(), v_Rr.clone(), v_Kr.clone())?), (Py::str("roc_radius"), f__zroc(v_poles.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dish_antenna_gain(mut v_D: Py, mut v_f: Py, mut v_eta: Py) -> R {
    let mut v_G: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let t1309 = (Py::tuple(vec![f__pos(v_D.clone(), Py::str("dish diameter"))?, f__pos(v_f.clone(), Py::str("frequency"))?, f__pos(v_eta.clone(), Py::str("aperture efficiency"))?])).items()?;
    if t1309.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_D = t1309[0].clone();
    v_f = t1309[1].clone();
    v_eta = t1309[2].clone();
    if ({ let t1310 = v_eta.clone(); Py::Bool({ let t1311 = Py::Int(1i64); if py::gt(&t1310, &t1311)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the aperture efficiency must be at most 1 (100 %)"))));
    }
    v_lam = py::truediv(&c_C0()?, &v_f.clone())?;
    if ({ let t1312 = v_D.clone(); Py::Bool({ let t1313 = v_lam.clone(); if py::lt(&t1312, &t1313)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the aperture must be at least one wavelength across"))));
    }
    v_G = py::mul(&v_eta.clone(), &py::pow(&py::truediv(&py::mul(&Py::Float(std::f64::consts::PI), &v_D.clone())?, &v_lam.clone())?, &Py::Int(2i64))?)?;
    return Ok(Py::dict(vec![(Py::str("G_dbi"), f__db10(v_G.clone())?), (Py::str("G"), v_G.clone()), (Py::str("Ae"), py::truediv(&py::mul(&py::mul(&py::mul(&v_eta.clone(), &Py::Float(std::f64::consts::PI))?, &v_D.clone())?, &v_D.clone())?, &Py::Int(4i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_exponential_pulse_spectrum(mut v_A: Py, mut v_tau: Py, mut v_f: Py) -> R {
    let mut v_x: Py = Py::None;
    let t1354 = (Py::tuple(vec![f__pos(v_A.clone(), Py::str("amplitude"))?, f__pos(v_tau.clone(), Py::str("time constant"))?, f__nonneg(v_f.clone(), Py::str("frequency"))?])).items()?;
    if t1354.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t1354[0].clone();
    v_tau = t1354[1].clone();
    v_f = t1354[2].clone();
    v_x = py::mul(&py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_f.clone())?, &v_tau.clone())?;
    return Ok(Py::dict(vec![(Py::str("magnitude"), py::truediv(&py::mul(&v_A.clone(), &v_tau.clone())?, &py::math1("sqrt", &py::add(&Py::Int(1i64), &py::mul(&v_x.clone(), &v_x.clone())?)?)?)?), (Py::str("phase"), py::add(&py::neg(&py::math1("atan", &v_x.clone())?)?, &Py::Float(0.0))?), (Py::str("re"), py::truediv(&py::mul(&v_A.clone(), &v_tau.clone())?, &py::add(&Py::Int(1i64), &py::mul(&v_x.clone(), &v_x.clone())?)?)?), (Py::str("im"), py::add(&py::truediv(&py::mul(&py::mul(&py::neg(&v_A.clone())?, &v_tau.clone())?, &v_x.clone())?, &py::add(&Py::Int(1i64), &py::mul(&v_x.clone(), &v_x.clone())?)?)?, &Py::Float(0.0))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fir_filter_output(mut v_b: Py, mut v_x: Py) -> R {
    let t1397 = (Py::tuple(vec![f__vec(v_b.clone(), Py::str("coefficients b"), Py::Int(1i64), Py::Int(10000i64))?, f__vec(v_x.clone(), Py::str("input x"), Py::Int(1i64), Py::Int(100000i64))?])).items()?;
    if t1397.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_b = t1397[0].clone();
    v_x = t1397[1].clone();
    return Ok(Py::dict(vec![(Py::str("y"), f__lfilter(v_b.clone(), Py::list(vec![Py::Float(1.0)]), v_x.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fir_window_design(mut v_N: Py, mut v_fc: Py, mut v_fs: Py, mut v_window: Py) -> R {
    let mut v_alpha: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_N = f__int(v_N.clone(), Py::str("number of taps"), Py::Int(1i64), Py::Int(10001i64))?;
    let t1457 = (Py::tuple(vec![f__pos(v_fc.clone(), Py::str("cutoff frequency"))?, f__pos(v_fs.clone(), Py::str("sampling rate"))?])).items()?;
    if t1457.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fc = t1457[0].clone();
    v_fs = t1457[1].clone();
    if ({ let t1458 = v_fc.clone(); Py::Bool({ let t1459 = py::truediv(&v_fs.clone(), &Py::Int(2i64))?; if py::ge(&t1458, &t1459)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the cutoff must lie below fs/2"))));
    }
    v_w = (if ({ let t1460 = v_N.clone(); Py::Bool({ let t1461 = Py::Int(1i64); if py::gt(&t1460, &t1461)? { true } else { false } }) }).truthy() { py::getitem(&f_window_coefficients(v_window.clone(), v_N.clone(), Py::str("symmetric"))?, &Py::str("w"))? } else { Py::list(vec![Py::Float(1.0)]) });
    v_c = py::truediv(&py::mul(&Py::Int(2i64), &v_fc.clone())?, &v_fs.clone())?;
    v_alpha = py::truediv(&py::sub(&v_N.clone(), &Py::Int(1i64))?, &Py::Int(2i64))?;
    v_h = Py::list(vec![]);
    for t1471 in py::range_iter(&[(v_N.clone()).i()?]) {
        v_n = t1471;
        v_x = py::mul(&v_c.clone(), &py::sub(&v_n.clone(), &v_alpha.clone())?)?;
        let _ = py::method(&v_h.clone(), "append", &[py::mul(&py::mul(&v_c.clone(), &(if ({ let t1472 = v_x.clone(); Py::Bool({ let t1473 = Py::Int(0i64); if py::eq(&t1472, &t1473) { true } else { false } }) }).truthy() { Py::Float(1.0) } else { py::truediv(&py::math1("sin", &py::mul(&Py::Float(std::f64::consts::PI), &v_x.clone())?)?, &py::mul(&Py::Float(std::f64::consts::PI), &v_x.clone())?)? }))?, &py::getitem(&v_w.clone(), &v_n.clone())?)?])?;
    }
    v_s = py::fsum(&v_h.clone())?;
    return Ok(Py::dict(vec![(Py::str("h"), { let mut out = vec![]; for t1474 in py::iter(&(v_h.clone()))? { let mut v_v: Py; 
v_v = t1474;
out.push(f__r12(py::truediv(&v_v.clone(), &v_s.clone())?)?); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fm_spectrum(mut v_df: Py, mut v_fm: Py, mut v_n: Py) -> R {
    let mut v_beta: Py = Py::None;
    let mut v_jn: Py = Py::None;
    let mut v_nmax: Py = Py::None;
    let mut v_pin: Py = Py::None;
    let t1512 = (Py::tuple(vec![f__pos(v_df.clone(), Py::str("peak frequency deviation"))?, f__pos(v_fm.clone(), Py::str("modulating frequency"))?])).items()?;
    if t1512.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_df = t1512[0].clone();
    v_fm = t1512[1].clone();
    v_n = f__int(v_n.clone(), Py::str("sideband number"), Py::Int(0i64), Py::Int(100i64))?;
    v_beta = py::truediv(&v_df.clone(), &v_fm.clone())?;
    if ({ let t1528 = v_beta.clone(); Py::Bool({ let t1529 = Py::Int(100i64); if py::gt(&t1528, &t1529)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the modulation index df/fm must be at most 100"))));
    }
    v_jn = f__besselj(v_n.clone(), v_beta.clone())?;
    v_nmax = py::int(&py::math1("floor", &py::add(&py::add(&v_beta.clone(), &Py::Int(1i64))?, &Py::Float(1e-09))?)?)?;
    v_pin = py::add(&py::pow(&f__besselj(Py::Int(0i64), v_beta.clone())?, &Py::Int(2i64))?, &py::mul(&Py::Int(2i64), &py::fsum(&{ let mut out = vec![]; for t1530 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_nmax.clone(), &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t1530;
out.push(py::pow(&f__besselj(v_k.clone(), v_beta.clone())?, &Py::Int(2i64))?); } Py::list(out) })?)?)?;
    return Ok(Py::dict(vec![(Py::str("carson_bandwidth"), py::mul(&Py::Int(2i64), &py::add(&v_df.clone(), &v_fm.clone())?)?), (Py::str("beta"), v_beta.clone()), (Py::str("carrier"), f__besselj(Py::Int(0i64), v_beta.clone())?), (Py::str("Jn"), v_jn.clone()), (Py::str("pair_power"), py::mul(&py::mul(&v_jn.clone(), &v_jn.clone())?, &(if ({ let t1531 = v_n.clone(); Py::Bool({ let t1532 = Py::Int(0i64); if py::eq(&t1531, &t1532) { true } else { false } }) }).truthy() { Py::Int(1i64) } else { Py::Int(2i64) }))?), (Py::str("power_in_carson"), v_pin.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fourier_series_harmonic(mut v_waveform: Py, mut v_A: Py, mut v_n: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_rel: Py = Py::None;
    if ({ let t1563 = v_waveform.clone(); Py::Bool({ let t1564 = c__WAVES()?; if !py::contains(&t1564, &t1563)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("waveform must be square, triangle or sawtooth"))));
    }
    let t1585 = (Py::tuple(vec![f__pos(v_A.clone(), Py::str("peak amplitude"))?, f__int(v_n.clone(), Py::str("harmonic number"), Py::Int(1i64), Py::Int(1000i64))?])).items()?;
    if t1585.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t1585[0].clone();
    v_n = t1585[1].clone();
    if ({ let t1586 = v_waveform.clone(); Py::Bool({ let t1587 = Py::str("square"); if py::eq(&t1586, &t1587) { true } else { false } }) }).truthy() {
        let t1588 = ((if (py::modulo(&v_n.clone(), &Py::Int(2i64))?).truthy() { Py::tuple(vec![py::truediv(&py::mul(&Py::Int(4i64), &v_A.clone())?, &py::mul(&v_n.clone(), &Py::Float(std::f64::consts::PI))?)?, py::truediv(&Py::Float(1.0), &v_n.clone())?]) } else { Py::tuple(vec![Py::Float(0.0), Py::Float(0.0)]) })).items()?;
        if t1588.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_c = t1588[0].clone();
        v_rel = t1588[1].clone();
    } else {
        if ({ let t1589 = v_waveform.clone(); Py::Bool({ let t1590 = Py::str("triangle"); if py::eq(&t1589, &t1590) { true } else { false } }) }).truthy() {
            let t1591 = ((if (py::modulo(&v_n.clone(), &Py::Int(2i64))?).truthy() { Py::tuple(vec![py::truediv(&py::mul(&Py::Int(8i64), &v_A.clone())?, &py::mul(&py::mul(&py::pow(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?, &v_n.clone())?, &v_n.clone())?)?, py::truediv(&Py::Float(1.0), &py::mul(&v_n.clone(), &v_n.clone())?)?]) } else { Py::tuple(vec![Py::Float(0.0), Py::Float(0.0)]) })).items()?;
            if t1591.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_c = t1591[0].clone();
            v_rel = t1591[1].clone();
        } else {
            let t1592 = (Py::tuple(vec![py::truediv(&py::mul(&Py::Int(2i64), &v_A.clone())?, &py::mul(&v_n.clone(), &Py::Float(std::f64::consts::PI))?)?, py::truediv(&Py::Float(1.0), &v_n.clone())?])).items()?;
            if t1592.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_c = t1592[0].clone();
            v_rel = t1592[1].clone();
        }
    }
    return Ok(Py::dict(vec![(Py::str("amplitude"), v_c.clone()), (Py::str("relative"), v_rel.clone()), (Py::str("rms"), py::truediv(&v_c.clone(), &py::math1("sqrt", &Py::Int(2i64))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_free_space_path_loss(mut v_d: Py, mut v_f: Py) -> R {
    let mut v_L: Py = Py::None;
    let t1628 = (Py::tuple(vec![f__pos(v_d.clone(), Py::str("distance"))?, f__pos(v_f.clone(), Py::str("frequency"))?])).items()?;
    if t1628.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_d = t1628[0].clone();
    v_f = t1628[1].clone();
    if ({ let t1629 = v_d.clone(); Py::Bool({ let t1630 = py::truediv(&c_C0()?, &v_f.clone())?; if py::lt(&t1629, &t1630)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the distance must be at least one wavelength (far field)"))));
    }
    v_L = f__fspl_ratio(v_d.clone(), v_f.clone())?;
    return Ok(Py::dict(vec![(Py::str("loss_db"), f__db10(v_L.clone())?), (Py::str("loss"), v_L.clone()), (Py::str("wavelength"), py::truediv(&c_C0()?, &v_f.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_frequency_from_period(mut v_T: Py) -> R {
    v_T = f__pos(v_T.clone(), Py::str("period"))?;
    return Ok(Py::dict(vec![(Py::str("f"), py::truediv(&Py::Float(1.0), &v_T.clone())?), (Py::str("omega"), py::truediv(&py::mul(&Py::Float(2.0), &Py::Float(std::f64::consts::PI))?, &v_T.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_frequency_resolution(mut v_fs: Py, mut v_N: Py) -> R {
    let t1716 = (Py::tuple(vec![f__pos(v_fs.clone(), Py::str("sampling rate"))?, f__int(v_N.clone(), Py::str("number of points"), Py::Int(2i64), Py::None)?])).items()?;
    if t1716.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fs = t1716[0].clone();
    v_N = t1716[1].clone();
    return Ok(Py::dict(vec![(Py::str("df"), py::truediv(&v_fs.clone(), &v_N.clone())?), (Py::str("duration"), py::truediv(&v_N.clone(), &v_fs.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_frequency_response(mut v_b: Py, mut v_f: Py, mut v_fs: Py, mut v_a: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_H: Py = Py::None;
    let mut v_w: Py = Py::None;
    v_b = f__vec(v_b.clone(), Py::str("b"), Py::Int(1i64), Py::Int(10000i64))?;
    v_a = (if ({ let t1759 = v_a.clone(); Py::Bool({ let t1760 = Py::None; if py::is(&t1759, &t1760) { true } else { false } }) }).truthy() { Py::list(vec![Py::Float(1.0)]) } else { f__vec(v_a.clone(), Py::str("a"), Py::Int(1i64), Py::Int(10000i64))? });
    let t1771 = (Py::tuple(vec![f__pos(v_fs.clone(), Py::str("sampling rate"))?, f__nonneg(v_f.clone(), Py::str("frequency"))?])).items()?;
    if t1771.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fs = t1771[0].clone();
    v_f = t1771[1].clone();
    if ({ let t1772 = v_f.clone(); Py::Bool({ let t1773 = py::mul(&py::truediv(&v_fs.clone(), &Py::Int(2i64))?, &py::add(&Py::Int(1i64), &Py::Float(1e-12))?)?; if py::gt(&t1772, &t1773)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the frequency must lie between 0 and fs/2"))));
    }
    v_w = py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_f.clone())?, &v_fs.clone())?;
    v_B = f__snap(py::getitem(&f__poly_at(v_b.clone(), v_w.clone())?, &Py::Int(0i64))?, py::fsum(&{ let mut out = vec![]; for t1786 in py::iter(&(v_b.clone()))? { let mut v_v: Py; 
v_v = t1786;
out.push(py::abs(&v_v.clone())?); } Py::list(out) })?)?;
    v_A = f__snap(py::getitem(&f__poly_at(v_a.clone(), v_w.clone())?, &Py::Int(0i64))?, py::fsum(&{ let mut out = vec![]; for t1787 in py::iter(&(v_a.clone()))? { let mut v_v: Py; 
v_v = t1787;
out.push(py::abs(&v_v.clone())?); } Py::list(out) })?)?;
    if ({ let t1788 = v_A.clone(); Py::Bool({ let t1789 = Py::Int(0i64); if py::eq(&t1788, &t1789) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the denominator vanishes at this frequency (a pole on the unit circle)"))));
    }
    v_H = py::truediv(&v_B.clone(), &v_A.clone())?;
    if ({ let t1790 = v_H.clone(); Py::Bool({ let t1791 = Py::Int(0i64); if py::eq(&t1790, &t1791) { true } else { false } }) }).truthy() {
        return Ok(Py::dict(vec![(Py::str("magnitude"), Py::Float(0.0)), (Py::str("magnitude_db"), py::neg(&c_INF()?)?), (Py::str("phase"), Py::Float(0.0))]));
    }
    return Ok(Py::dict(vec![(Py::str("magnitude"), py::abs(&v_H.clone())?), (Py::str("magnitude_db"), f__db20(py::abs(&v_H.clone())?)?), (Py::str("phase"), py::atan2(&py::imag(&v_H.clone())?, &py::real(&v_H.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_friis_received_power(mut v_Pt: Py, mut v_Gt_db: Py, mut v_Gr_db: Py, mut v_f: Py, mut v_d: Py) -> R {
    let mut v_pr_dbm: Py = Py::None;
    let t1827 = (Py::tuple(vec![f__pos(v_Pt.clone(), Py::str("transmit power"))?, f__pos(v_f.clone(), Py::str("frequency"))?, f__pos(v_d.clone(), Py::str("distance"))?])).items()?;
    if t1827.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_Pt = t1827[0].clone();
    v_f = t1827[1].clone();
    v_d = t1827[2].clone();
    let t1830 = (Py::tuple(vec![f__fin(v_Gt_db.clone(), Py::str("transmit antenna gain"))?, f__fin(v_Gr_db.clone(), Py::str("receive antenna gain"))?])).items()?;
    if t1830.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Gt_db = t1830[0].clone();
    v_Gr_db = t1830[1].clone();
    if ({ let t1831 = v_d.clone(); Py::Bool({ let t1832 = py::truediv(&c_C0()?, &v_f.clone())?; if py::lt(&t1831, &t1832)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the distance must be at least one wavelength (far field)"))));
    }
    v_pr_dbm = py::sub(&py::add(&py::add(&f__db10(py::truediv(&v_Pt.clone(), &Py::Float(0.001))?)?, &v_Gt_db.clone())?, &v_Gr_db.clone())?, &f__db10(f__fspl_ratio(v_d.clone(), v_f.clone())?)?)?;
    return Ok(Py::dict(vec![(Py::str("Pr_dbm"), v_pr_dbm.clone()), (Py::str("Pr"), py::mul(&Py::Float(0.001), &py::pow(&Py::Int(10i64), &py::truediv(&v_pr_dbm.clone(), &Py::Int(10i64))?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_group_delay(mut v_b: Py, mut v_f: Py, mut v_fs: Py, mut v_a: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_Ad: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_Bd: Py = Py::None;
    let mut v_tau: Py = Py::None;
    let mut v_w: Py = Py::None;
    v_b = f__vec(v_b.clone(), Py::str("b"), Py::Int(1i64), Py::Int(10000i64))?;
    v_a = (if ({ let t1875 = v_a.clone(); Py::Bool({ let t1876 = Py::None; if py::is(&t1875, &t1876) { true } else { false } }) }).truthy() { Py::list(vec![Py::Float(1.0)]) } else { f__vec(v_a.clone(), Py::str("a"), Py::Int(1i64), Py::Int(10000i64))? });
    let t1887 = (Py::tuple(vec![f__pos(v_fs.clone(), Py::str("sampling rate"))?, f__nonneg(v_f.clone(), Py::str("frequency"))?])).items()?;
    if t1887.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fs = t1887[0].clone();
    v_f = t1887[1].clone();
    if ({ let t1888 = v_f.clone(); Py::Bool({ let t1889 = py::mul(&py::truediv(&v_fs.clone(), &Py::Int(2i64))?, &py::add(&Py::Int(1i64), &Py::Float(1e-12))?)?; if py::gt(&t1888, &t1889)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the frequency must lie between 0 and fs/2"))));
    }
    v_w = py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_f.clone())?, &v_fs.clone())?;
    let t1898 = (Py::tuple(vec![f__poly_at(v_b.clone(), v_w.clone())?, f__poly_at(v_a.clone(), v_w.clone())?])).items()?;
    if t1898.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    let t1899 = (t1898[0].clone()).items()?;
    if t1899.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_B = t1899[0].clone();
    v_Bd = t1899[1].clone();
    let t1900 = (t1898[1].clone()).items()?;
    if t1900.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t1900[0].clone();
    v_Ad = t1900[1].clone();
    if ({ let t1907 = { let t1901 = py::abs(&v_B.clone())?; Py::Bool({ let t1902 = py::mul(&Py::Float(1e-12), &py::fsum(&{ let mut out = vec![]; for t1903 in py::iter(&(v_b.clone()))? { let mut v_v: Py; 
v_v = t1903;
out.push(py::abs(&v_v.clone())?); } Py::list(out) })?)?; if py::le(&t1901, &t1902)? { true } else { false } }) }; if t1907.truthy() { t1907 } else { { let t1904 = py::abs(&v_A.clone())?; Py::Bool({ let t1905 = py::mul(&Py::Float(1e-12), &py::fsum(&{ let mut out = vec![]; for t1906 in py::iter(&(v_a.clone()))? { let mut v_v: Py; 
v_v = t1906;
out.push(py::abs(&v_v.clone())?); } Py::list(out) })?)?; if py::le(&t1904, &t1905)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the group delay is undefined at a zero or pole on the unit circle"))));
    }
    v_tau = py::sub(&py::real(&py::truediv(&v_Bd.clone(), &v_B.clone())?)?, &py::real(&py::truediv(&v_Ad.clone(), &v_A.clone())?)?)?;
    return Ok(Py::dict(vec![(Py::str("samples"), v_tau.clone()), (Py::str("seconds"), py::truediv(&v_tau.clone(), &v_fs.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_haar_dwt(mut v_x: Py, mut v_levels: Py) -> R {
    let mut v_N: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_details: Py = Py::None;
    v_x = f__vec(v_x.clone(), Py::str("signal x"), Py::Int(2i64), Py::Int(65536i64))?;
    v_N = py::len(&v_x.clone())?;
    if (py::bitand(&v_N.clone(), &py::sub(&v_N.clone(), &Py::Int(1i64))?)?).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the length must be a power of two"))));
    }
    v_levels = f__int(v_levels.clone(), Py::str("number of levels"), Py::Int(1i64), py::sub(&py::method(&v_N.clone(), "bit_length", &[])?, &Py::Int(1i64))?)?;
    let t1965 = (Py::tuple(vec![v_x.clone(), Py::list(vec![])])).items()?;
    if t1965.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1965[0].clone();
    v_details = t1965[1].clone();
    for t1966 in py::range_iter(&[(v_levels.clone()).i()?]) {
        v__ = t1966;
        let _ = py::method(&v_details.clone(), "insert", &[Py::Int(0i64), { let mut out = vec![]; for t1967 in py::range_iter(&[(py::floordiv(&py::len(&v_a.clone())?, &Py::Int(2i64))?).i()?]) { let mut v_i: Py; 
v_i = t1967;
out.push(py::mul(&c__S2()?, &py::sub(&py::getitem(&v_a.clone(), &py::mul(&Py::Int(2i64), &v_i.clone())?)?, &py::getitem(&v_a.clone(), &py::add(&py::mul(&Py::Int(2i64), &v_i.clone())?, &Py::Int(1i64))?)?)?)?); } Py::list(out) }])?;
        v_a = { let mut out = vec![]; for t1968 in py::range_iter(&[(py::floordiv(&py::len(&v_a.clone())?, &Py::Int(2i64))?).i()?]) { let mut v_i: Py; 
v_i = t1968;
out.push(py::mul(&c__S2()?, &py::add(&py::getitem(&v_a.clone(), &py::mul(&Py::Int(2i64), &v_i.clone())?)?, &py::getitem(&v_a.clone(), &py::add(&py::mul(&Py::Int(2i64), &v_i.clone())?, &Py::Int(1i64))?)?)?)?); } Py::list(out) };
    }
    return Ok(Py::dict(vec![(Py::str("approximation"), v_a.clone()), (Py::str("details"), v_details.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_haar_idwt(mut v_cA: Py, mut v_cD: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_x: Py = Py::None;
    let t2011 = (Py::tuple(vec![f__vec(v_cA.clone(), Py::str("approximation coefficients"), Py::Int(1i64), Py::Int(32768i64))?, f__vec(v_cD.clone(), Py::str("detail coefficients"), Py::Int(1i64), Py::Int(32768i64))?])).items()?;
    if t2011.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_cA = t2011[0].clone();
    v_cD = t2011[1].clone();
    if ({ let t2012 = py::len(&v_cA.clone())?; Py::Bool({ let t2013 = py::len(&v_cD.clone())?; if py::ne(&t2012, &t2013) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the approximation and detail lists must have the same length"))));
    }
    v_x = Py::list(vec![]);
    for t2014 in py::iter(&(py::zip(&[v_cA.clone(), v_cD.clone()])?))? {
        let t2015 = (t2014).items()?;
        if t2015.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a = t2015[0].clone();
        v_d = t2015[1].clone();
        v_x = py::iadd(&v_x, &Py::list(vec![py::mul(&c__S2()?, &py::add(&v_a.clone(), &v_d.clone())?)?, py::mul(&c__S2()?, &py::sub(&v_a.clone(), &v_d.clone())?)?]))?;
    }
    return Ok(Py::dict(vec![(Py::str("x"), v_x.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_iir_filter_output(mut v_b: Py, mut v_a: Py, mut v_x: Py) -> R {
    let t2058 = (Py::tuple(vec![f__vec(v_b.clone(), Py::str("b"), Py::Int(1i64), Py::Int(1000i64))?, f__vec(v_a.clone(), Py::str("a"), Py::Int(1i64), Py::Int(1000i64))?, f__vec(v_x.clone(), Py::str("input x"), Py::Int(1i64), Py::Int(100000i64))?])).items()?;
    if t2058.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_b = t2058[0].clone();
    v_a = t2058[1].clone();
    v_x = t2058[2].clone();
    return Ok(Py::dict(vec![(Py::str("y"), f__lfilter(v_b.clone(), v_a.clone(), v_x.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_impulse_response(mut v_b: Py, mut v_a: Py, mut v_n: Py) -> R {
    let t2110 = (Py::tuple(vec![f__vec(v_b.clone(), Py::str("b"), Py::Int(1i64), Py::Int(1000i64))?, f__vec(v_a.clone(), Py::str("a"), Py::Int(1i64), Py::Int(1000i64))?])).items()?;
    if t2110.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_b = t2110[0].clone();
    v_a = t2110[1].clone();
    v_n = f__int(v_n.clone(), Py::str("number of samples"), Py::Int(1i64), Py::Int(100000i64))?;
    return Ok(Py::dict(vec![(Py::str("h"), f__lfilter(v_b.clone(), v_a.clone(), py::add(&Py::list(vec![Py::Float(1.0)]), &py::mul(&Py::list(vec![Py::Float(0.0)]), &py::sub(&v_n.clone(), &Py::Int(1i64))?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_input_impedance(mut v_RL: Py, mut v_Z0: Py, mut v_length: Py, mut v_XL: Py) -> R {
    let mut v_g: Py = Py::None;
    let mut v_gi: Py = Py::None;
    let mut v_z: Py = Py::None;
    let t2173 = (f__gamma(v_RL.clone(), v_XL.clone(), v_Z0.clone())?).items()?;
    if t2173.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_g = t2173[0].clone();
    v_Z0 = t2173[1].clone();
    v_length = f__nonneg(v_length.clone(), Py::str("line length in wavelengths"))?;
    v_gi = py::mul(&v_g.clone(), &py::cmath("exp", &[py::complex(&[Py::Int(0i64), py::mul(&py::mul(&py::neg(&Py::Int(4i64))?, &Py::Float(std::f64::consts::PI))?, &v_length.clone())?])?])?)?;
    v_z = f__snap(py::truediv(&py::mul(&v_Z0.clone(), &py::add(&Py::Int(1i64), &v_gi.clone())?)?, &py::sub(&Py::Int(1i64), &v_gi.clone())?)?, v_Z0.clone())?;
    return Ok(Py::dict(vec![(Py::str("Rin"), py::add(&py::real(&v_z.clone())?, &Py::Float(0.0))?), (Py::str("Xin"), py::add(&py::imag(&v_z.clone())?, &Py::Float(0.0))?), (Py::str("abs_zin"), py::abs(&v_z.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_inverse_dft(mut v_re_: Py, mut v_im: Py) -> R {
    let mut v_N: Py = Py::None;
    let mut v_X: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_re_ = f__vec(v_re_.clone(), Py::str("real parts"), Py::Int(1i64), Py::Int(65536i64))?;
    v_im = (if ({ let t2225 = v_im.clone(); Py::Bool({ let t2226 = Py::None; if py::is(&t2225, &t2226) { true } else { false } }) }).truthy() { py::mul(&Py::list(vec![Py::Float(0.0)]), &py::len(&v_re_.clone())?)? } else { f__vec(v_im.clone(), Py::str("imaginary parts"), Py::Int(1i64), Py::Int(65536i64))? });
    if ({ let t2227 = py::len(&v_im.clone())?; Py::Bool({ let t2228 = py::len(&v_re_.clone())?; if py::ne(&t2227, &t2228) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the real and imaginary parts must have the same length"))));
    }
    v_X = { let mut out = vec![]; for t2229 in py::iter(&(py::zip(&[v_re_.clone(), v_im.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t2230 = (t2229).items()?;
if t2230.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2230[0].clone();
v_b = t2230[1].clone();
out.push(py::complex(&[v_a.clone(), v_b.clone()])?); } Py::list(out) };
    v_N = py::len(&v_X.clone())?;
    v_s = py::truediv(&py::fsum(&{ let mut out = vec![]; for t2231 in py::iter(&(v_X.clone()))? { let mut v_z: Py; 
v_z = t2231;
out.push(py::abs(&v_z.clone())?); } Py::list(out) })?, &v_N.clone())?;
    v_x = { let mut out = vec![]; for t2246 in py::iter(&(f__fft(v_X.clone(), py::pos(&Py::Int(1i64))?)?))? { let mut v_z: Py; 
v_z = t2246;
out.push(f__snap(py::truediv(&v_z.clone(), &v_N.clone())?, v_s.clone())?); } Py::list(out) };
    return Ok(Py::dict(vec![(Py::str("real"), { let mut out = vec![]; for t2251 in py::iter(&(v_x.clone()))? { let mut v_z: Py; 
v_z = t2251;
out.push(py::real(&v_z.clone())?); } Py::list(out) }), (Py::str("imag"), { let mut out = vec![]; for t2252 in py::iter(&(v_x.clone()))? { let mut v_z: Py; 
v_z = t2252;
out.push(py::imag(&v_z.clone())?); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_inverse_z_transform(mut v_b: Py, mut v_a: Py, mut v_n: Py) -> R {
    let mut v_K: Py = Py::None;
    let mut v_Kr: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_Rr: Py = Py::None;
    let mut v_poles: Py = Py::None;
    let mut v_res: Py = Py::None;
    let t2295 = (Py::tuple(vec![f__vec(v_b.clone(), Py::str("numerator b"), Py::Int(1i64), Py::Int(7i64))?, f__vec(v_a.clone(), Py::str("denominator a"), Py::Int(1i64), Py::Int(7i64))?])).items()?;
    if t2295.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_b = t2295[0].clone();
    v_a = t2295[1].clone();
    v_n = f__int(v_n.clone(), Py::str("number of samples"), Py::Int(1i64), Py::Int(200i64))?;
    if ({ let t2311 = py::getitem(&v_a.clone(), &Py::Int(0i64))?; Py::Bool({ let t2312 = Py::Int(0i64); if py::eq(&t2311, &t2312) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a[0] must not be 0"))));
    }
    if (Py::Bool(({ let mut out = vec![]; for t2313 in py::iter(&(v_b.clone()))? { let mut v_v: Py; 
v_v = t2313;
out.push({ let t2314 = v_v.clone(); Py::Bool({ let t2315 = Py::Int(0i64); if py::eq(&t2314, &t2315) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the numerator is identically zero"))));
    }
    let t2421 = (f__pf_expand(v_b.clone(), v_a.clone())?).items()?;
    if t2421.len() != 6 { return Err(PyErr::Value("unpack".into())); }
    v_poles = t2421[0].clone();
    v_res = t2421[1].clone();
    v_K = t2421[2].clone();
    v_P = t2421[3].clone();
    v_Rr = t2421[4].clone();
    v_Kr = t2421[5].clone();
    return Ok(Py::dict(vec![(Py::str("residues"), v_Rr.clone()), (Py::str("poles"), v_P.clone()), (Py::str("direct_terms"), v_Kr.clone()), (Py::str("samples"), f__zsamples(v_poles.clone(), v_res.clone(), v_K.clone(), v_n.clone())?), (Py::str("closed_form"), f__zform(Py::str("x[n]"), v_P.clone(), v_Rr.clone(), v_Kr.clone())?), (Py::str("roc_radius"), f__zroc(v_poles.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_linear_convolution(mut v_x: Py, mut v_h: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_y: Py = Py::None;
    let t2498 = (Py::tuple(vec![f__vec(v_x.clone(), Py::str("x"), Py::Int(1i64), Py::Int(20000i64))?, f__vec(v_h.clone(), Py::str("h"), Py::Int(1i64), Py::Int(20000i64))?])).items()?;
    if t2498.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t2498[0].clone();
    v_h = t2498[1].clone();
    v_n = py::sub(&py::add(&py::len(&v_x.clone())?, &py::len(&v_h.clone())?)?, &Py::Int(1i64))?;
    v_y = { let mut out = vec![]; for t2499 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t2499;
out.push(py::fsum(&{ let mut out = vec![]; for t2500 in py::range_iter(&[(py::minmax(&[Py::Int(0i64), py::add(&py::sub(&v_i.clone(), &py::len(&v_h.clone())?)?, &Py::Int(1i64))?], true)?).i()?, (py::add(&py::minmax(&[v_i.clone(), py::sub(&py::len(&v_x.clone())?, &Py::Int(1i64))?], false)?, &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t2500;
out.push(py::mul(&py::getitem(&v_x.clone(), &v_k.clone())?, &py::getitem(&v_h.clone(), &py::sub(&v_i.clone(), &v_k.clone())?)?)?); } Py::list(out) })?); } Py::list(out) };
    return Ok(Py::dict(vec![(Py::str("y"), v_y.clone()), (Py::str("length"), v_n.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_moving_average(mut v_x: Py, mut v_M: Py) -> R {
    v_x = f__vec(v_x.clone(), Py::str("signal x"), Py::Int(1i64), Py::Int(100000i64))?;
    v_M = f__int(v_M.clone(), Py::str("window length M"), Py::Int(1i64), Py::Int(100000i64))?;
    return Ok(Py::dict(vec![(Py::str("y"), { let mut out = vec![]; for t2558 in py::range_iter(&[(py::len(&v_x.clone())?).i()?]) { let mut v_n: Py; 
v_n = t2558;
out.push(py::truediv(&py::fsum(&{ let mut out = vec![]; for t2559 in py::range_iter(&[(v_M.clone()).i()?]) { let mut v_i: Py; 
v_i = t2559;
if ({ let t2560 = py::sub(&v_n.clone(), &v_i.clone())?; Py::Bool({ let t2561 = Py::Int(0i64); if py::ge(&t2560, &t2561)? { true } else { false } }) }).truthy() { out.push(py::getitem(&v_x.clone(), &py::sub(&v_n.clone(), &v_i.clone())?)?); }} Py::list(out) })?, &v_M.clone())?); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mpsk_symbol_error(mut v_M: Py, mut v_ebn0_db: Py) -> R {
    let mut v_gs: Py = Py::None;
    let mut v_ps: Py = Py::None;
    let mut v_s2: Py = Py::None;
    let mut v_v: Py = Py::None;
    if ({ let t2592 = Py::str(&py::to_str(&v_M.clone())); Py::Bool({ let t2593 = c__PSK_M()?; if !py::contains(&t2593, &t2592)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("M must be 2, 4, 8, 16, 32 or 64"))));
    }
    v_M = py::int(&v_M.clone())?;
    v_gs = py::mul(&py::pow(&Py::Int(10i64), &py::truediv(&f__fin(v_ebn0_db.clone(), Py::str("Eb/N0 in dB"))?, &Py::Int(10i64))?)?, &py::math1("log2", &v_M.clone())?)?;
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(30i64))?;
        v_s2 = py::pow(&ax_engine::pymp::call("sin", &[py::truediv(&ax_engine::pymp::constant("pi")?, &v_M.clone())?], &[])?, &Py::Int(2i64))?;
        v_v = py::truediv(&ax_engine::pymp::call("quad", &[{ let v_M = v_M.clone(); let v_ebn0_db = v_ebn0_db.clone(); let v_gs = v_gs.clone(); let v_s2 = v_s2.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_t: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(ax_engine::pymp::call("exp", &[py::truediv(&py::mul(&py::neg(&v_gs.clone())?, &v_s2.clone())?, &py::pow(&ax_engine::pymp::call("sin", &[v_t.clone()], &[])?, &Py::Int(2i64))?)?], &[])?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, Py::list(vec![Py::Int(0i64), py::truediv(&ax_engine::pymp::constant("pi")?, &Py::Int(2i64))?, py::truediv(&py::mul(&py::sub(&v_M.clone(), &Py::Int(1i64))?, &ax_engine::pymp::constant("pi")?)?, &v_M.clone())?])], &[])?, &ax_engine::pymp::constant("pi")?)?;
    }
    v_ps = py::float(&v_v.clone())?;
    return Ok(Py::dict(vec![(Py::str("ser"), v_ps.clone()), (Py::str("ber_approx"), py::truediv(&v_ps.clone(), &py::math1("log2", &v_M.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mqam_symbol_error(mut v_M: Py, mut v_ebn0_db: Py) -> R {
    let mut v_gs: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_ps: Py = Py::None;
    v_M = (if ({ let t2626 = Py::str(&py::to_str(&v_M.clone())); Py::Bool({ let t2627 = c__QAM_M()?; if py::contains(&t2627, &t2626)? { true } else { false } }) }).truthy() { py::int(&v_M.clone())? } else { Py::None });
    if ({ let t2628 = v_M.clone(); Py::Bool({ let t2629 = Py::None; if py::is(&t2628, &t2629) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("M must be 4, 16, 64, 256 or 1024 (square QAM)"))));
    }
    v_k = py::int(&py::round(&py::math1("log2", &v_M.clone())?, None)?)?;
    v_gs = py::mul(&py::pow(&Py::Int(10i64), &py::truediv(&f__fin(v_ebn0_db.clone(), Py::str("Eb/N0 in dB"))?, &Py::Int(10i64))?)?, &v_k.clone())?;
    v_p = py::mul(&py::mul(&Py::Int(2i64), &py::sub(&Py::Int(1i64), &py::truediv(&Py::Int(1i64), &py::math1("sqrt", &v_M.clone())?)?)?)?, &f__qf(py::math1("sqrt", &py::truediv(&py::mul(&Py::Int(3i64), &v_gs.clone())?, &py::sub(&v_M.clone(), &Py::Int(1i64))?)?)?)?)?;
    v_ps = py::mul(&v_p.clone(), &py::sub(&Py::Int(2i64), &v_p.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("ser"), v_ps.clone()), (Py::str("ber_approx"), py::truediv(&v_ps.clone(), &v_k.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_noise_figure_cascade(mut v_nf_db: Py, mut v_gain_db: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_Ft: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_gd: Py = Py::None;
    let mut v_nf: Py = Py::None;
    let mut v_terms: Py = Py::None;
    let t2674 = (Py::tuple(vec![f__vec(v_nf_db.clone(), Py::str("noise figures in dB"), Py::Int(1i64), Py::Int(100i64))?, f__vec(v_gain_db.clone(), Py::str("gains in dB"), Py::Int(1i64), Py::Int(100i64))?])).items()?;
    if t2674.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_nf_db = t2674[0].clone();
    v_gain_db = t2674[1].clone();
    if ({ let t2675 = py::len(&v_nf_db.clone())?; Py::Bool({ let t2676 = py::len(&v_gain_db.clone())?; if py::ne(&t2675, &t2676) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give one noise figure and one gain per stage"))));
    }
    if (Py::Bool(({ let mut out = vec![]; for t2677 in py::iter(&(v_nf_db.clone()))? { let mut v_v: Py; 
v_v = t2677;
out.push({ let t2678 = v_v.clone(); Py::Bool({ let t2679 = Py::Int(0i64); if py::lt(&t2678, &t2679)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a noise figure in dB is never negative"))));
    }
    let t2680 = (Py::tuple(vec![py::pow(&Py::Int(10i64), &py::truediv(&py::getitem(&v_nf_db.clone(), &Py::Int(0i64))?, &Py::Int(10i64))?)?, py::pow(&Py::Int(10i64), &py::truediv(&py::getitem(&v_gain_db.clone(), &Py::Int(0i64))?, &Py::Int(10i64))?)?])).items()?;
    if t2680.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_F = t2680[0].clone();
    v_g = t2680[1].clone();
    v_terms = Py::list(vec![v_F.clone()]);
    for t2681 in py::iter(&(py::zip(&[py::slice(&v_nf_db.clone(), Some((Py::Int(1i64)).i()?), None, None)?, py::slice(&v_gain_db.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? {
        let t2682 = (t2681).items()?;
        if t2682.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_nf = t2682[0].clone();
        v_gd = t2682[1].clone();
        let _ = py::method(&v_terms.clone(), "append", &[py::truediv(&py::sub(&py::pow(&Py::Int(10i64), &py::truediv(&v_nf.clone(), &Py::Int(10i64))?)?, &Py::Int(1i64))?, &v_g.clone())?])?;
        v_g = py::mul(&v_g, &py::pow(&Py::Int(10i64), &py::truediv(&v_gd.clone(), &Py::Int(10i64))?)?)?;
    }
    v_Ft = py::fsum(&v_terms.clone())?;
    return Ok(Py::dict(vec![(Py::str("nf_db"), f__db10(v_Ft.clone())?), (Py::str("F"), v_Ft.clone()), (Py::str("Te"), py::mul(&c_T0()?, &py::sub(&v_Ft.clone(), &Py::Int(1i64))?)?), (Py::str("gain_db"), py::fsum(&v_gain_db.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_noncoherent_ber(mut v_ebn0_db: Py) -> R {
    let mut v_g: Py = Py::None;
    v_g = py::pow(&Py::Int(10i64), &py::truediv(&f__fin(v_ebn0_db.clone(), Py::str("Eb/N0 in dB"))?, &Py::Int(10i64))?)?;
    return Ok(Py::dict(vec![(Py::str("ber_dpsk"), py::mul(&Py::Float(0.5), &py::math1("exp", &py::neg(&v_g.clone())?)?)?), (Py::str("ber_ncfsk"), py::mul(&Py::Float(0.5), &py::math1("exp", &py::truediv(&py::neg(&v_g.clone())?, &Py::Int(2i64))?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_notch_filter_design(mut v_f0: Py, mut v_Q: Py, mut v_fs: Py) -> R {
    let mut v_beta: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_w0: Py = Py::None;
    let t2750 = (Py::tuple(vec![f__pos(v_f0.clone(), Py::str("notch frequency"))?, f__pos(v_Q.clone(), Py::str("quality factor"))?, f__pos(v_fs.clone(), Py::str("sampling rate"))?])).items()?;
    if t2750.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_f0 = t2750[0].clone();
    v_Q = t2750[1].clone();
    v_fs = t2750[2].clone();
    if ({ let t2751 = v_f0.clone(); Py::Bool({ let t2752 = py::truediv(&v_fs.clone(), &Py::Int(2i64))?; if py::ge(&t2751, &t2752)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the notch frequency must lie below fs/2"))));
    }
    v_w0 = py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_f0.clone())?, &v_fs.clone())?;
    v_beta = py::math1("tan", &py::truediv(&py::truediv(&v_w0.clone(), &v_Q.clone())?, &Py::Int(2i64))?)?;
    v_g = py::truediv(&Py::Int(1i64), &py::add(&Py::Int(1i64), &v_beta.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("b"), Py::list(vec![v_g.clone(), py::mul(&py::mul(&py::neg(&Py::Int(2i64))?, &v_g.clone())?, &py::math1("cos", &v_w0.clone())?)?, v_g.clone()])), (Py::str("a"), Py::list(vec![Py::Float(1.0), py::mul(&py::mul(&py::neg(&Py::Int(2i64))?, &v_g.clone())?, &py::math1("cos", &v_w0.clone())?)?, py::sub(&py::mul(&Py::Int(2i64), &v_g.clone())?, &Py::Int(1i64))?]))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_nyquist_rate(mut v_fmax: Py) -> R {
    v_fmax = f__pos(v_fmax.clone(), Py::str("maximum frequency"))?;
    return Ok(Py::dict(vec![(Py::str("fs_min"), py::mul(&Py::Float(2.0), &v_fmax.clone())?), (Py::str("Ts_max"), py::truediv(&Py::Float(1.0), &py::mul(&Py::Float(2.0), &v_fmax.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_period_from_frequency(mut v_f: Py) -> R {
    v_f = f__pos(v_f.clone(), Py::str("frequency"))?;
    return Ok(Py::dict(vec![(Py::str("T"), py::truediv(&Py::Float(1.0), &v_f.clone())?), (Py::str("omega"), py::mul(&py::mul(&Py::Float(2.0), &Py::Float(std::f64::consts::PI))?, &v_f.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_periodogram(mut v_x: Py, mut v_fs: Py) -> R {
    let mut v_K: Py = Py::None;
    let mut v_N: Py = Py::None;
    let mut v_X: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_mu: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_psd: Py = Py::None;
    v_x = f__vec(v_x.clone(), Py::str("signal x"), Py::Int(2i64), Py::Int(65536i64))?;
    v_fs = f__pos(v_fs.clone(), Py::str("sampling rate"))?;
    v_N = py::len(&v_x.clone())?;
    v_mu = py::truediv(&py::fsum(&v_x.clone())?, &v_N.clone())?;
    v_X = f__fft({ let mut out = vec![]; for t2884 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t2884;
out.push(py::sub(&v_v.clone(), &v_mu.clone())?); } Py::list(out) }, py::neg(&Py::Int(1i64))?)?;
    v_K = py::add(&py::floordiv(&v_N.clone(), &Py::Int(2i64))?, &Py::Int(1i64))?;
    v_psd = Py::list(vec![]);
    for t2885 in py::range_iter(&[(v_K.clone()).i()?]) {
        v_k = t2885;
        v_p = py::truediv(&py::pow(&py::abs(&py::getitem(&v_X.clone(), &v_k.clone())?)?, &Py::Int(2i64))?, &py::mul(&v_fs.clone(), &v_N.clone())?)?;
        if ({ let t2893 = { let t2886 = v_k.clone(); Py::Bool({ let t2887 = Py::Int(0i64); if py::ne(&t2886, &t2887) { true } else { false } }) }; if !t2893.truthy() { t2893 } else { py::not(&{ let t2892 = { let t2888 = py::modulo(&v_N.clone(), &Py::Int(2i64))?; Py::Bool({ let t2889 = Py::Int(0i64); if py::eq(&t2888, &t2889) { true } else { false } }) }; if !t2892.truthy() { t2892 } else { { let t2890 = v_k.clone(); Py::Bool({ let t2891 = py::floordiv(&v_N.clone(), &Py::Int(2i64))?; if py::eq(&t2890, &t2891) { true } else { false } }) } } }) } }).truthy() {
            v_p = py::mul(&v_p, &Py::Int(2i64))?;
        }
        let _ = py::method(&v_psd.clone(), "append", &[v_p.clone()])?;
    }
    v_i = f__peak(v_psd.clone(), py::minmax(&[v_psd.clone()], true)?)?;
    return Ok(Py::dict(vec![(Py::str("peak_frequency"), py::truediv(&py::mul(&v_i.clone(), &v_fs.clone())?, &v_N.clone())?), (Py::str("freqs"), { let mut out = vec![]; for t2899 in py::range_iter(&[(v_K.clone()).i()?]) { let mut v_k: Py; 
v_k = t2899;
out.push(py::truediv(&py::mul(&v_k.clone(), &v_fs.clone())?, &v_N.clone())?); } Py::list(out) }), (Py::str("psd"), v_psd.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_poles_zeros(mut v_b: Py, mut v_a: Py) -> R {
    let mut v_ap: Py = Py::None;
    let mut v_bp: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_rad: Py = Py::None;
    let mut v_z: Py = Py::None;
    let t2942 = (Py::tuple(vec![f__vec(v_b.clone(), Py::str("numerator b"), Py::Int(1i64), Py::Int(5i64))?, f__vec(v_a.clone(), Py::str("denominator a"), Py::Int(1i64), Py::Int(5i64))?])).items()?;
    if t2942.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_b = t2942[0].clone();
    v_a = t2942[1].clone();
    if ({ let t2943 = py::getitem(&v_a.clone(), &Py::Int(0i64))?; Py::Bool({ let t2944 = Py::Int(0i64); if py::eq(&t2943, &t2944) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a[0] must not be 0"))));
    }
    if (Py::Bool(({ let mut out = vec![]; for t2945 in py::iter(&(v_b.clone()))? { let mut v_v: Py; 
v_v = t2945;
out.push({ let t2946 = v_v.clone(); Py::Bool({ let t2947 = Py::Int(0i64); if py::eq(&t2946, &t2947) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the numerator is identically zero"))));
    }
    let t2948 = (f__pad2(v_b.clone(), v_a.clone())?).items()?;
    if t2948.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_bp = t2948[0].clone();
    v_ap = t2948[1].clone();
    let t2999 = (Py::tuple(vec![f__canon_roots(f__roots(v_bp.clone())?)?, f__canon_roots(f__roots(v_ap.clone())?)?])).items()?;
    if t2999.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_z = t2999[0].clone();
    v_p = t2999[1].clone();
    v_rad = { let t = { let mut out = vec![]; for t3000 in py::iter(&(v_p.clone()))? { let mut v_v: Py; 
v_v = t3000;
out.push(py::abs(&v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } };
    return Ok(Py::dict(vec![(Py::str("max_pole_radius"), v_rad.clone()), (Py::str("zeros"), v_z.clone()), (Py::str("poles"), v_p.clone()), (Py::str("stability"), (if ({ let t3001 = v_rad.clone(); Py::Bool({ let t3002 = Py::Int(1i64); if py::lt(&t3001, &t3002)? { true } else { false } }) }).truthy() { Py::str("stable") } else { Py::str("unstable") }))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_power_ratio_to_db(mut v_ratio: Py) -> R {
    return Ok(Py::dict(vec![(Py::str("dB"), f__db10(f__pos(v_ratio.clone(), Py::str("power ratio"))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_qpsk_error_rates(mut v_ebn0_db: Py) -> R {
    let mut v_g: Py = Py::None;
    let mut v_q: Py = Py::None;
    v_g = py::pow(&Py::Int(10i64), &py::truediv(&f__fin(v_ebn0_db.clone(), Py::str("Eb/N0 in dB"))?, &Py::Int(10i64))?)?;
    v_q = f__qf(py::math1("sqrt", &py::mul(&Py::Int(2i64), &v_g.clone())?)?)?;
    return Ok(Py::dict(vec![(Py::str("ber"), v_q.clone()), (Py::str("ser"), py::mul(&v_q.clone(), &py::sub(&Py::Int(2i64), &v_q.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_quantization_snr(mut v_bits: Py) -> R {
    let mut v_b: Py = Py::None;
    v_b = f__int(v_bits.clone(), Py::str("number of bits"), Py::Int(1i64), Py::Int(64i64))?;
    return Ok(Py::dict(vec![(Py::str("sqnr_db"), py::add(&f__db10(Py::Float(1.5))?, &py::mul(&py::mul(&Py::Int(2i64), &v_b.clone())?, &f__db10(Py::Float(2.0))?)?)?), (Py::str("dynamic_range_db"), py::mul(&v_b.clone(), &f__db20(Py::Float(2.0))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_quarter_wave_transformer(mut v_RL: Py, mut v_Z0: Py, mut v_f: Py, mut v_vf: Py) -> R {
    let mut v_out: Py = Py::None;
    let t3150 = (Py::tuple(vec![f__pos(v_RL.clone(), Py::str("load resistance"))?, f__pos(v_Z0.clone(), Py::str("line impedance to match"))?])).items()?;
    if t3150.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_RL = t3150[0].clone();
    v_Z0 = t3150[1].clone();
    v_vf = f__pos(v_vf.clone(), Py::str("velocity factor"))?;
    if ({ let t3151 = v_vf.clone(); Py::Bool({ let t3152 = Py::Int(1i64); if py::gt(&t3151, &t3152)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the velocity factor must be at most 1"))));
    }
    v_out = Py::dict(vec![(Py::str("Zt"), py::math1("sqrt", &py::mul(&v_RL.clone(), &v_Z0.clone())?)?)]);
    if ({ let t3153 = v_f.clone(); Py::Bool({ let t3154 = Py::None; if !py::is(&t3153, &t3154) { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("length"), py::truediv(&py::mul(&v_vf.clone(), &c_C0()?)?, &py::mul(&Py::Int(4i64), &f__pos(v_f.clone(), Py::str("frequency"))?)?)?)?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_radar_doppler(mut v_v: Py, mut v_f: Py) -> R {
    let t3192 = (Py::tuple(vec![f__fin(v_v.clone(), Py::str("radial velocity"))?, f__pos(v_f.clone(), Py::str("frequency"))?])).items()?;
    if t3192.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_v = t3192[0].clone();
    v_f = t3192[1].clone();
    if ({ let t3193 = py::abs(&v_v.clone())?; Py::Bool({ let t3194 = py::truediv(&c_C0()?, &Py::Int(10i64))?; if py::ge(&t3193, &t3194)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the radial velocity must be far below the speed of light"))));
    }
    return Ok(Py::dict(vec![(Py::str("fd"), py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &v_v.clone())?, &v_f.clone())?, &py::sub(&c_C0()?, &v_v.clone())?)?), (Py::str("wavelength"), py::truediv(&c_C0()?, &v_f.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_radar_max_range(mut v_Pt: Py, mut v_G_db: Py, mut v_f: Py, mut v_sigma: Py, mut v_Pmin: Py) -> R {
    v_Pmin = f__pos(v_Pmin.clone(), Py::str("minimum detectable signal"))?;
    return Ok(Py::dict(vec![(Py::str("Rmax"), py::pow(&py::truediv(&f__radar_k(v_Pt.clone(), v_G_db.clone(), v_f.clone(), v_sigma.clone())?, &v_Pmin.clone())?, &Py::Float(0.25))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_radar_received_power(mut v_Pt: Py, mut v_G_db: Py, mut v_f: Py, mut v_sigma: Py, mut v_R: Py) -> R {
    let mut v_pr: Py = Py::None;
    v_R = f__pos(v_R.clone(), Py::str("range"))?;
    v_pr = py::truediv(&f__radar_k(v_Pt.clone(), v_G_db.clone(), v_f.clone(), v_sigma.clone())?, &py::pow(&v_R.clone(), &Py::Int(4i64))?)?;
    return Ok(Py::dict(vec![(Py::str("Pr"), v_pr.clone()), (Py::str("Pr_dbm"), f__db10(py::truediv(&v_pr.clone(), &Py::Float(0.001))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rect_pulse_spectrum(mut v_A: Py, mut v_tau: Py, mut v_f: Py) -> R {
    let mut v_s: Py = Py::None;
    let mut v_x: Py = Py::None;
    let t3313 = (Py::tuple(vec![f__fin(v_A.clone(), Py::str("amplitude"))?, f__pos(v_tau.clone(), Py::str("pulse width"))?, f__nonneg(v_f.clone(), Py::str("frequency"))?])).items()?;
    if t3313.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t3313[0].clone();
    v_tau = t3313[1].clone();
    v_f = t3313[2].clone();
    if ({ let t3314 = v_A.clone(); Py::Bool({ let t3315 = Py::Int(0i64); if py::eq(&t3314, &t3315) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the amplitude must not be 0"))));
    }
    v_x = py::mul(&v_f.clone(), &v_tau.clone())?;
    v_s = (if ({ let t3316 = v_x.clone(); Py::Bool({ let t3317 = Py::Int(0i64); if py::eq(&t3316, &t3317) { true } else { false } }) }).truthy() { Py::Float(1.0) } else { py::truediv(&py::math1("sin", &py::mul(&Py::Float(std::f64::consts::PI), &v_x.clone())?)?, &py::mul(&Py::Float(std::f64::consts::PI), &v_x.clone())?)? });
    return Ok(Py::dict(vec![(Py::str("X"), py::mul(&py::mul(&v_A.clone(), &v_tau.clone())?, &v_s.clone())?), (Py::str("magnitude"), py::abs(&py::mul(&py::mul(&v_A.clone(), &v_tau.clone())?, &v_s.clone())?)?), (Py::str("first_null"), py::truediv(&Py::Float(1.0), &v_tau.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_reflection_coefficient(mut v_RL: Py, mut v_Z0: Py, mut v_XL: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_g: Py = Py::None;
    let t3356 = (f__gamma(v_RL.clone(), v_XL.clone(), v_Z0.clone())?).items()?;
    if t3356.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_g = t3356[0].clone();
    v_Z0 = t3356[1].clone();
    v_a = py::abs(&v_g.clone())?;
    return Ok(Py::dict(vec![(Py::str("gamma_mag"), v_a.clone()), (Py::str("gamma_angle_deg"), (if (v_a.clone()).truthy() { py::add(&py::math1("degrees", &py::atan2(&py::imag(&v_g.clone())?, &py::real(&v_g.clone())?)?)?, &Py::Float(0.0))? } else { Py::Float(0.0) })), (Py::str("gamma_re"), py::real(&v_g.clone())?), (Py::str("gamma_im"), py::imag(&v_g.clone())?), (Py::str("vswr"), py::truediv(&py::add(&Py::Int(1i64), &v_a.clone())?, &py::sub(&Py::Int(1i64), &v_a.clone())?)?), (Py::str("return_loss_db"), (if (v_a.clone()).truthy() { py::neg(&f__db20(v_a.clone())?)? } else { c_INF()? })), (Py::str("mismatch_loss_db"), py::neg(&f__db10(py::sub(&Py::Int(1i64), &py::mul(&v_a.clone(), &v_a.clone())?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_reflection_from_vswr(mut v_vswr: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_s = f__pos(v_vswr.clone(), Py::str("VSWR"))?;
    if ({ let t3392 = v_s.clone(); Py::Bool({ let t3393 = Py::Int(1i64); if py::lt(&t3392, &t3393)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the VSWR is at least 1"))));
    }
    v_a = py::truediv(&py::sub(&v_s.clone(), &Py::Int(1i64))?, &py::add(&v_s.clone(), &Py::Int(1i64))?)?;
    return Ok(Py::dict(vec![(Py::str("gamma"), v_a.clone()), (Py::str("return_loss_db"), (if (v_a.clone()).truthy() { py::neg(&f__db20(v_a.clone())?)? } else { c_INF()? })), (Py::str("mismatch_loss_db"), py::neg(&f__db10(py::sub(&Py::Int(1i64), &py::mul(&v_a.clone(), &v_a.clone())?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_second_order_poles(mut v_a1: Py, mut v_a2: Py) -> R {
    let mut v_disc: Py = Py::None;
    let mut v_im: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_rad: Py = Py::None;
    let mut v_re_: Py = Py::None;
    let t3426 = (Py::tuple(vec![f__fin(v_a1.clone(), Py::str("a1"))?, f__fin(v_a2.clone(), Py::str("a2"))?])).items()?;
    if t3426.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a1 = t3426[0].clone();
    v_a2 = t3426[1].clone();
    if ({ let t3427 = v_a2.clone(); Py::Bool({ let t3428 = Py::Int(0i64); if py::eq(&t3427, &t3428) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a2 must not be 0 (otherwise the system is first order)"))));
    }
    v_disc = py::sub(&py::mul(&v_a1.clone(), &v_a1.clone())?, &py::mul(&Py::Int(4i64), &v_a2.clone())?)?;
    if ({ let t3429 = v_disc.clone(); Py::Bool({ let t3430 = Py::Int(0i64); if py::ge(&t3429, &t3430)? { true } else { false } }) }).truthy() {
        v_q = py::truediv(&py::neg(&py::add(&v_a1.clone(), &py::copysign(&py::math1("sqrt", &v_disc.clone())?, &v_a1.clone())?)?)?, &Py::Int(2i64))?;
        v_r = py::sorted(&Py::list(vec![v_q.clone(), py::truediv(&v_a2.clone(), &v_q.clone())?]), (Py::Bool(false)).truthy())?;
        v_p = Py::list(vec![py::add(&py::getitem(&v_r.clone(), &Py::Int(0i64))?, &Py::Float(0.0))?, py::add(&py::getitem(&v_r.clone(), &Py::Int(1i64))?, &Py::Float(0.0))?]);
    } else {
        let t3431 = (Py::tuple(vec![py::add(&py::truediv(&py::neg(&v_a1.clone())?, &Py::Int(2i64))?, &Py::Float(0.0))?, py::truediv(&py::math1("sqrt", &py::neg(&v_disc.clone())?)?, &Py::Int(2i64))?])).items()?;
        if t3431.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_re_ = t3431[0].clone();
        v_im = t3431[1].clone();
        v_p = Py::list(vec![py::complex(&[v_re_.clone(), v_im.clone()])?, py::complex(&[v_re_.clone(), py::neg(&v_im.clone())?])?]);
    }
    v_rad = py::minmax(&[{ let mut out = vec![]; for t3432 in py::iter(&(v_p.clone()))? { let mut v_v: Py; 
v_v = t3432;
out.push(py::abs(&v_v.clone())?); } Py::list(out) }], true)?;
    return Ok(Py::dict(vec![(Py::str("radius"), v_rad.clone()), (Py::str("p1"), py::getitem(&v_p.clone(), &Py::Int(0i64))?), (Py::str("p2"), py::getitem(&v_p.clone(), &Py::Int(1i64))?), (Py::str("stability"), (if ({ let t3433 = v_rad.clone(); Py::Bool({ let t3434 = Py::Int(1i64); if py::lt(&t3433, &t3434)? { true } else { false } }) }).truthy() { Py::str("stable") } else { Py::str("unstable") }))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_shannon_capacity(mut v_B: Py, mut v_snr_db: Py) -> R {
    let mut v_C: Py = Py::None;
    let t3472 = (Py::tuple(vec![f__pos(v_B.clone(), Py::str("bandwidth"))?, f__fin(v_snr_db.clone(), Py::str("SNR in dB"))?])).items()?;
    if t3472.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_B = t3472[0].clone();
    v_snr_db = t3472[1].clone();
    v_C = py::mul(&v_B.clone(), &py::math1("log2", &py::add(&Py::Int(1i64), &py::pow(&Py::Int(10i64), &py::truediv(&v_snr_db.clone(), &Py::Int(10i64))?)?)?)?)?;
    return Ok(Py::dict(vec![(Py::str("C"), v_C.clone()), (Py::str("efficiency"), py::truediv(&v_C.clone(), &v_B.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_shannon_min_snr(mut v_C: Py, mut v_B: Py) -> R {
    let mut v_eta: Py = Py::None;
    let mut v_s: Py = Py::None;
    let t3508 = (Py::tuple(vec![f__pos(v_C.clone(), Py::str("data rate"))?, f__pos(v_B.clone(), Py::str("bandwidth"))?])).items()?;
    if t3508.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_C = t3508[0].clone();
    v_B = t3508[1].clone();
    v_eta = py::truediv(&v_C.clone(), &v_B.clone())?;
    if ({ let t3509 = v_eta.clone(); Py::Bool({ let t3510 = Py::Int(200i64); if py::gt(&t3509, &t3510)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the spectral efficiency C/B must be at most 200 bit/s/Hz"))));
    }
    v_s = py::math1("expm1", &py::mul(&v_eta.clone(), &py::math1("log", &Py::Int(2i64))?)?)?;
    return Ok(Py::dict(vec![(Py::str("snr_db"), f__db10(v_s.clone())?), (Py::str("snr"), v_s.clone()), (Py::str("ebn0_min_db"), f__db10(py::truediv(&v_s.clone(), &v_eta.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_signal_statistics(mut v_x: Py) -> R {
    let mut v_e: Py = Py::None;
    let mut v_n: Py = Py::None;
    v_x = f__vec(v_x.clone(), Py::str("signal x"), Py::Int(1i64), Py::None)?;
    v_n = py::len(&v_x.clone())?;
    v_e = py::fsum(&{ let mut out = vec![]; for t3553 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t3553;
out.push(py::mul(&v_v.clone(), &v_v.clone())?); } Py::list(out) })?;
    return Ok(Py::dict(vec![(Py::str("mean"), py::truediv(&py::fsum(&v_x.clone())?, &v_n.clone())?), (Py::str("rms"), py::math1("sqrt", &py::truediv(&v_e.clone(), &v_n.clone())?)?), (Py::str("energy"), v_e.clone()), (Py::str("power"), py::truediv(&v_e.clone(), &v_n.clone())?), (Py::str("peak"), py::minmax(&[{ let mut out = vec![]; for t3554 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t3554;
out.push(py::abs(&v_v.clone())?); } Py::list(out) }], true)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_sine_rms(mut v_Vp: Py) -> R {
    v_Vp = f__pos(v_Vp.clone(), Py::str("peak value"))?;
    return Ok(Py::dict(vec![(Py::str("Vrms"), py::truediv(&v_Vp.clone(), &py::math1("sqrt", &Py::Float(2.0))?)?), (Py::str("Vpp"), py::mul(&Py::Float(2.0), &v_Vp.clone())?), (Py::str("Vavg"), py::truediv(&py::mul(&Py::Float(2.0), &v_Vp.clone())?, &Py::Float(std::f64::consts::PI))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_snr_from_powers(mut v_Ps: Py, mut v_Pn: Py) -> R {
    let t3625 = (Py::tuple(vec![f__pos(v_Ps.clone(), Py::str("signal power"))?, f__pos(v_Pn.clone(), Py::str("noise power"))?])).items()?;
    if t3625.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Ps = t3625[0].clone();
    v_Pn = t3625[1].clone();
    return Ok(Py::dict(vec![(Py::str("snr"), py::truediv(&v_Ps.clone(), &v_Pn.clone())?), (Py::str("snr_db"), f__db10(py::truediv(&v_Ps.clone(), &v_Pn.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_step_response(mut v_b: Py, mut v_a: Py, mut v_n: Py) -> R {
    let t3668 = (Py::tuple(vec![f__vec(v_b.clone(), Py::str("b"), Py::Int(1i64), Py::Int(1000i64))?, f__vec(v_a.clone(), Py::str("a"), Py::Int(1i64), Py::Int(1000i64))?])).items()?;
    if t3668.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_b = t3668[0].clone();
    v_a = t3668[1].clone();
    v_n = f__int(v_n.clone(), Py::str("number of samples"), Py::Int(1i64), Py::Int(100000i64))?;
    return Ok(Py::dict(vec![(Py::str("s"), f__lfilter(v_b.clone(), v_a.clone(), py::mul(&Py::list(vec![Py::Float(1.0)]), &v_n.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_thd(mut v_V: Py) -> R {
    let mut v_t: Py = Py::None;
    v_V = f__vec(v_V.clone(), Py::str("harmonic amplitudes"), Py::Int(2i64), Py::Int(10000i64))?;
    if ({ let t3740 = { let t3735 = py::getitem(&v_V.clone(), &Py::Int(0i64))?; Py::Bool({ let t3736 = Py::Int(0i64); if py::le(&t3735, &t3736)? { true } else { false } }) }; if t3740.truthy() { t3740 } else { Py::Bool(({ let mut out = vec![]; for t3737 in py::iter(&(v_V.clone()))? { let mut v_v: Py; 
v_v = t3737;
out.push({ let t3738 = v_v.clone(); Py::Bool({ let t3739 = Py::Int(0i64); if py::lt(&t3738, &t3739)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the amplitudes must be >= 0 and the fundamental > 0"))));
    }
    v_t = py::truediv(&py::math1("sqrt", &py::fsum(&{ let mut out = vec![]; for t3741 in py::iter(&(py::slice(&v_V.clone(), Some((Py::Int(1i64)).i()?), None, None)?))? { let mut v_v: Py; 
v_v = t3741;
out.push(py::mul(&v_v.clone(), &v_v.clone())?); } Py::list(out) })?)?, &py::getitem(&v_V.clone(), &Py::Int(0i64))?)?;
    return Ok(Py::dict(vec![(Py::str("thd"), v_t.clone()), (Py::str("percent"), py::mul(&Py::Int(100i64), &v_t.clone())?), (Py::str("db"), (if ({ let t3742 = v_t.clone(); Py::Bool({ let t3743 = Py::Int(0i64); if py::gt(&t3742, &t3743)? { true } else { false } }) }).truthy() { f__db20(v_t.clone())? } else { py::neg(&c_INF()?)? }))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_vswr_from_reflection(mut v_gamma: Py) -> R {
    let mut v_a: Py = Py::None;
    v_a = f__nonneg(v_gamma.clone(), Py::str("reflection coefficient magnitude"))?;
    if ({ let t3779 = v_a.clone(); Py::Bool({ let t3780 = Py::Int(1i64); if py::ge(&t3779, &t3780)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("|Gamma| must be below 1"))));
    }
    return Ok(Py::dict(vec![(Py::str("vswr"), py::truediv(&py::add(&Py::Int(1i64), &v_a.clone())?, &py::sub(&Py::Int(1i64), &v_a.clone())?)?), (Py::str("return_loss_db"), (if (v_a.clone()).truthy() { py::neg(&f__db20(v_a.clone())?)? } else { c_INF()? })), (Py::str("mismatch_loss_db"), py::neg(&f__db10(py::sub(&Py::Int(1i64), &py::mul(&v_a.clone(), &v_a.clone())?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_watts_to_dbm(mut v_P: Py) -> R {
    v_P = f__pos(v_P.clone(), Py::str("power"))?;
    return Ok(Py::dict(vec![(Py::str("dBm"), f__db10(py::truediv(&v_P.clone(), &Py::Float(0.001))?)?), (Py::str("dBW"), f__db10(v_P.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_window_coefficients(mut v_window: Py, mut v_N: Py, mut v_form: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_coef: Py = Py::None;
    let mut v_w: Py = Py::None;
    if ({ let t3846 = v_window.clone(); Py::Bool({ let t3847 = c__MAINLOBE_BINS()?; if !py::contains(&t3847, &t3846)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&py::add(&Py::str("window must be one of "), &py::method(&Py::str(", "), "join", &[c__WINDOWS()?])?)?)));
    }
    if ({ let t3848 = v_form.clone(); Py::Bool({ let t3849 = Py::tuple(vec![Py::str("symmetric"), Py::str("periodic")]); if !py::contains(&t3849, &t3848)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("form must be symmetric or periodic"))));
    }
    v_N = f__int(v_N.clone(), Py::str("window length N"), Py::Int(2i64), Py::Int(100000i64))?;
    v_M = (if ({ let t3865 = v_form.clone(); Py::Bool({ let t3866 = Py::str("symmetric"); if py::eq(&t3865, &t3866) { true } else { false } }) }).truthy() { py::sub(&v_N.clone(), &Py::Int(1i64))? } else { v_N.clone() });
    v_coef = py::getitem(&Py::dict(vec![(Py::str("rectangular"), Py::list(vec![Py::Float(1.0)])), (Py::str("hann"), Py::list(vec![Py::Float(0.5), Py::Float(0.5)])), (Py::str("hamming"), Py::list(vec![Py::Float(0.54), Py::Float(0.46)])), (Py::str("blackman"), Py::list(vec![Py::Float(0.42), Py::Float(0.5), Py::Float(0.08)]))]), &v_window.clone())?;
    v_w = { let mut out = vec![]; for t3867 in py::range_iter(&[(v_N.clone()).i()?]) { let mut v_n: Py; 
v_n = t3867;
out.push(py::fsum(&{ let mut out = vec![]; for t3868 in py::iter(&(py::enumerate(&v_coef.clone(), 0)?))? { let mut v_c: Py; let mut v_k: Py; 
let t3869 = (t3868).items()?;
if t3869.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3869[0].clone();
v_c = t3869[1].clone();
out.push(py::mul(&py::mul(&py::pow(&py::neg(&Py::Int(1i64))?, &v_k.clone())?, &v_c.clone())?, &py::math1("cos", &py::truediv(&py::mul(&py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_k.clone())?, &v_n.clone())?, &v_M.clone())?)?)?); } Py::list(out) })?); } Py::list(out) };
    return Ok(Py::dict(vec![(Py::str("w"), v_w.clone()), (Py::str("coherent_gain"), py::truediv(&py::fsum(&v_w.clone())?, &v_N.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_window_mainlobe_width(mut v_window: Py, mut v_N: Py, mut v_fs: Py) -> R {
    let mut v_bins: Py = Py::None;
    let mut v_out: Py = Py::None;
    if ({ let t3900 = v_window.clone(); Py::Bool({ let t3901 = c__MAINLOBE_BINS()?; if !py::contains(&t3901, &t3900)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&py::add(&Py::str("window must be one of "), &py::method(&Py::str(", "), "join", &[c__WINDOWS()?])?)?)));
    }
    v_N = f__int(v_N.clone(), Py::str("window length N"), Py::Int(8i64), Py::None)?;
    v_bins = py::float(&py::getitem(&c__MAINLOBE_BINS()?, &v_window.clone())?)?;
    v_out = Py::dict(vec![(Py::str("width_bins"), v_bins.clone()), (Py::str("first_null_bins"), py::truediv(&v_bins.clone(), &Py::Int(2i64))?)]);
    if ({ let t3917 = v_fs.clone(); Py::Bool({ let t3918 = Py::None; if !py::is(&t3917, &t3918) { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("width_hz"), py::truediv(&py::mul(&v_bins.clone(), &f__pos(v_fs.clone(), Py::str("sampling rate"))?)?, &v_N.clone())?)?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_z_transform_sequence(mut v_coefficients: Py, mut v_ratios: Py, mut v_powers: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_den: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_mult: Py = Py::None;
    let mut v_num: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_part: Py = Py::None;
    let mut v_ps: Py = Py::None;
    let mut v_scale: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_c = f__vec(v_coefficients.clone(), Py::str("coefficients"), Py::Int(1i64), Py::Int(6i64))?;
    v_p = f__vec(v_ratios.clone(), Py::str("ratios"), Py::Int(1i64), Py::Int(6i64))?;
    v_k = (if ({ let t3966 = v_powers.clone(); Py::Bool({ let t3967 = Py::None; if py::is(&t3966, &t3967) { true } else { false } }) }).truthy() { py::mul(&Py::list(vec![Py::Float(0.0)]), &py::len(&v_c.clone())?)? } else { f__vec(v_powers.clone(), Py::str("powers"), Py::Int(1i64), Py::Int(6i64))? });
    if ({ let t3972 = { let t3968 = py::len(&v_p.clone())?; Py::Bool({ let t3969 = py::len(&v_c.clone())?; if py::ne(&t3968, &t3969) { true } else { false } }) }; if t3972.truthy() { t3972 } else { { let t3970 = py::len(&v_k.clone())?; Py::Bool({ let t3971 = py::len(&v_c.clone())?; if py::ne(&t3970, &t3971) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("coefficients, ratios and powers need the same number of entries"))));
    }
    for t3973 in py::range_iter(&[(py::len(&v_c.clone())?).i()?]) {
        v_i = t3973;
        if ({ let t3974 = py::getitem(&v_c.clone(), &v_i.clone())?; Py::Bool({ let t3975 = Py::Int(0i64); if py::eq(&t3974, &t3975) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("every coefficient must be non-zero"))));
        }
        if ({ let t3976 = py::getitem(&v_p.clone(), &v_i.clone())?; Py::Bool({ let t3977 = Py::Int(0i64); if py::eq(&t3976, &t3977) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("every ratio must be non-zero"))));
        }
        if ({ let t3982 = { let t3978 = py::getitem(&v_k.clone(), &v_i.clone())?; Py::Bool({ let t3979 = Py::Int(0i64); if py::ne(&t3978, &t3979) { true } else { false } }) }; if !t3982.truthy() { t3982 } else { { let t3980 = py::getitem(&v_k.clone(), &v_i.clone())?; Py::Bool({ let t3981 = Py::Int(1i64); if py::ne(&t3980, &t3981) { true } else { false } }) } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("each power is 0 (the term c p^n) or 1 (the term c n p^n)"))));
        }
        for t3983 in py::range_iter(&[(v_i.clone()).i()?]) {
            v_j = t3983;
            if ({ let t3988 = { let t3984 = py::getitem(&v_p.clone(), &v_j.clone())?; Py::Bool({ let t3985 = py::getitem(&v_p.clone(), &v_i.clone())?; if py::eq(&t3984, &t3985) { true } else { false } }) }; if !t3988.truthy() { t3988 } else { { let t3986 = py::getitem(&v_k.clone(), &v_j.clone())?; Py::Bool({ let t3987 = py::getitem(&v_k.clone(), &v_i.clone())?; if py::eq(&t3986, &t3987) { true } else { false } }) } } }).truthy() {
                return Err(py::value_error(py::to_str(&Py::str("two terms with the same ratio and power: add their coefficients"))));
            }
        }
    }
    v_ps = Py::list(vec![]);
    for t3989 in py::iter(&(v_p.clone()))? {
        v_v = t3989;
        if ({ let t3990 = v_v.clone(); Py::Bool({ let t3991 = v_ps.clone(); if !py::contains(&t3991, &t3990)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_ps.clone(), "append", &[v_v.clone()])?;
        }
    }
    let _ = py::method_sort(&v_ps.clone(), None, false)?;
    v_mult = Py::list(vec![]);
    for t3992 in py::iter(&(v_ps.clone()))? {
        v_v = t3992;
        v_m = Py::Int(1i64);
        for t3993 in py::range_iter(&[(py::len(&v_p.clone())?).i()?]) {
            v_i = t3993;
            if ({ let t3998 = { let t3994 = py::getitem(&v_p.clone(), &v_i.clone())?; Py::Bool({ let t3995 = v_v.clone(); if py::eq(&t3994, &t3995) { true } else { false } }) }; if !t3998.truthy() { t3998 } else { { let t3996 = py::getitem(&v_k.clone(), &v_i.clone())?; Py::Bool({ let t3997 = Py::Int(1i64); if py::eq(&t3996, &t3997) { true } else { false } }) } } }).truthy() {
                v_m = Py::Int(2i64);
            }
        }
        let _ = py::method(&v_mult.clone(), "append", &[v_m.clone()])?;
    }
    v_den = Py::list(vec![Py::Float(1.0)]);
    for t3999 in py::range_iter(&[(py::len(&v_ps.clone())?).i()?]) {
        v_j = t3999;
        for t4000 in py::range_iter(&[(py::getitem(&v_mult.clone(), &v_j.clone())?).i()?]) {
            v__ = t4000;
            v_den = f__pmul(v_den.clone(), Py::list(vec![Py::Float(1.0), py::neg(&py::getitem(&v_ps.clone(), &v_j.clone())?)?]))?;
        }
    }
    v_D = py::sub(&py::len(&v_den.clone())?, &Py::Int(1i64))?;
    let t4005 = (Py::tuple(vec![py::mul(&Py::list(vec![Py::Float(0.0)]), &v_D.clone())?, py::mul(&Py::list(vec![Py::Float(0.0)]), &v_D.clone())?])).items()?;
    if t4005.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_num = t4005[0].clone();
    v_scale = t4005[1].clone();
    for t4006 in py::range_iter(&[(py::len(&v_c.clone())?).i()?]) {
        v_i = t4006;
        v_part = (if ({ let t4007 = py::getitem(&v_k.clone(), &v_i.clone())?; Py::Bool({ let t4008 = Py::Int(0i64); if py::eq(&t4007, &t4008) { true } else { false } }) }).truthy() { Py::list(vec![py::getitem(&v_c.clone(), &v_i.clone())?]) } else { Py::list(vec![Py::Float(0.0), py::mul(&py::getitem(&v_c.clone(), &v_i.clone())?, &py::getitem(&v_p.clone(), &v_i.clone())?)?]) });
        for t4009 in py::range_iter(&[(py::len(&v_ps.clone())?).i()?]) {
            v_j = t4009;
            v_e = py::sub(&py::getitem(&v_mult.clone(), &v_j.clone())?, &(if ({ let t4010 = py::getitem(&v_ps.clone(), &v_j.clone())?; Py::Bool({ let t4011 = py::getitem(&v_p.clone(), &v_i.clone())?; if py::eq(&t4010, &t4011) { true } else { false } }) }).truthy() { (if ({ let t4012 = py::getitem(&v_k.clone(), &v_i.clone())?; Py::Bool({ let t4013 = Py::Int(0i64); if py::eq(&t4012, &t4013) { true } else { false } }) }).truthy() { Py::Int(1i64) } else { Py::Int(2i64) }) } else { Py::Int(0i64) }))?;
            for t4014 in py::range_iter(&[(v_e.clone()).i()?]) {
                v__ = t4014;
                v_part = f__pmul(v_part.clone(), Py::list(vec![Py::Float(1.0), py::neg(&py::getitem(&v_ps.clone(), &v_j.clone())?)?]))?;
            }
        }
        for t4015 in py::range_iter(&[(py::len(&v_part.clone())?).i()?]) {
            v_t = t4015;
            let t4016 = v_num.clone(); let t4017 = v_t.clone();
            py::setitem(&t4016, &t4017, py::iadd(&py::getitem(&t4016, &t4017)?, &py::getitem(&v_part.clone(), &v_t.clone())?)?)?;
            let t4018 = v_scale.clone(); let t4019 = v_t.clone();
            py::setitem(&t4018, &t4019, py::iadd(&py::getitem(&t4018, &t4019)?, &py::abs(&py::getitem(&v_part.clone(), &v_t.clone())?)?)?)?;
        }
    }
    for t4020 in py::range_iter(&[(v_D.clone()).i()?]) {
        v_t = t4020;
        if ({ let t4021 = py::abs(&py::getitem(&v_num.clone(), &v_t.clone())?)?; Py::Bool({ let t4022 = py::mul(&Py::Float(1e-12), &py::getitem(&v_scale.clone(), &v_t.clone())?)?; if py::le(&t4021, &t4022)? { true } else { false } }) }).truthy() {
            py::setitem(&v_num.clone(), &v_t.clone(), Py::Float(0.0))?;
        }
    }
    return Ok(Py::dict(vec![(Py::str("b"), f__trim0(v_num.clone())?), (Py::str("a"), v_den.clone()), (Py::str("roc_radius"), py::minmax(&[{ let mut out = vec![]; for t4028 in py::iter(&(v_ps.clone()))? { let mut v_v: Py; 
v_v = t4028;
out.push(py::abs(&v_v.clone())?); } Py::list(out) }], true)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1047(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t1049: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1048 = v_inputs.clone(); f_dft_spectrum(py::getitem(&t1048, &Py::str("x"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1049 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_12(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t14: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t13 = v_inputs.clone(); f_alias_frequency(py::getitem(&t13, &Py::str("f"))?, py::getitem(&t13, &Py::str("fs"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t14 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_128(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t130: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t129 = v_inputs.clone(); f_amplitude_ratio_to_db(py::getitem(&t129, &Py::str("ratio"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t130 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1314(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t1316: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1315 = v_inputs.clone(); f_dish_antenna_gain(py::getitem(&t1315, &Py::str("D"))?, py::getitem(&t1315, &Py::str("f"))?, py::getitem(&t1315, &Py::str("eta"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1316 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1355(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t1357: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1356 = v_inputs.clone(); f_exponential_pulse_spectrum(py::getitem(&t1356, &Py::str("A"))?, py::getitem(&t1356, &Py::str("tau"))?, py::getitem(&t1356, &Py::str("f"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1357 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1407(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t1409: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1408 = v_inputs.clone(); f_fir_filter_output(py::getitem(&t1408, &Py::str("b"))?, py::getitem(&t1408, &Py::str("x"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1409 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1477(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t1479: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1478 = v_inputs.clone(); f_fir_window_design(py::getitem(&t1478, &Py::str("N"))?, py::getitem(&t1478, &Py::str("fc"))?, py::getitem(&t1478, &Py::str("fs"))?, py::getitem(&t1478, &Py::str("window"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1479 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1533(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t1535: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1534 = v_inputs.clone(); f_fm_spectrum(py::getitem(&t1534, &Py::str("df"))?, py::getitem(&t1534, &Py::str("fm"))?, py::method(&t1534, "get", &[Py::str("n"), Py::Int(1i64)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1535 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1593(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t1595: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1594 = v_inputs.clone(); f_fourier_series_harmonic(py::getitem(&t1594, &Py::str("waveform"))?, py::getitem(&t1594, &Py::str("A"))?, py::getitem(&t1594, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1595 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1631(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t1633: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1632 = v_inputs.clone(); f_free_space_path_loss(py::getitem(&t1632, &Py::str("d"))?, py::getitem(&t1632, &Py::str("f"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1633 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1666(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t1668: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1667 = v_inputs.clone(); f_frequency_from_period(py::getitem(&t1667, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1668 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1717(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t1719: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1718 = v_inputs.clone(); f_frequency_resolution(py::getitem(&t1718, &Py::str("fs"))?, py::getitem(&t1718, &Py::str("N"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1719 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1792(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t1794: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1793 = v_inputs.clone(); f_frequency_response(py::getitem(&t1793, &Py::str("b"))?, py::getitem(&t1793, &Py::str("f"))?, py::getitem(&t1793, &Py::str("fs"))?, py::method(&t1793, "get", &[Py::str("a"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1794 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1833(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t1835: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1834 = v_inputs.clone(); f_friis_received_power(py::getitem(&t1834, &Py::str("Pt"))?, py::getitem(&t1834, &Py::str("Gt_db"))?, py::getitem(&t1834, &Py::str("Gr_db"))?, py::getitem(&t1834, &Py::str("f"))?, py::getitem(&t1834, &Py::str("d"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1835 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1908(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t1910: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1909 = v_inputs.clone(); f_group_delay(py::getitem(&t1909, &Py::str("b"))?, py::getitem(&t1909, &Py::str("f"))?, py::getitem(&t1909, &Py::str("fs"))?, py::method(&t1909, "get", &[Py::str("a"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1910 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1969(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t1971: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1970 = v_inputs.clone(); f_haar_dwt(py::getitem(&t1970, &Py::str("x"))?, py::method(&t1970, "get", &[Py::str("levels"), Py::Int(1i64)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1971 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_199(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t201: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t200 = v_inputs.clone(); f_analytic_envelope(py::getitem(&t200, &Py::str("x"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t201 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2016(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2018: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2017 = v_inputs.clone(); f_haar_idwt(py::getitem(&t2017, &Py::str("cA"))?, py::getitem(&t2017, &Py::str("cD"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2018 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2068(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2070: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2069 = v_inputs.clone(); f_iir_filter_output(py::getitem(&t2069, &Py::str("b"))?, py::getitem(&t2069, &Py::str("a"))?, py::getitem(&t2069, &Py::str("x"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2070 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2135(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2137: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2136 = v_inputs.clone(); f_impulse_response(py::getitem(&t2136, &Py::str("b"))?, py::getitem(&t2136, &Py::str("a"))?, py::getitem(&t2136, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2137 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2183(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2185: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2184 = v_inputs.clone(); f_input_impedance(py::getitem(&t2184, &Py::str("RL"))?, py::getitem(&t2184, &Py::str("Z0"))?, py::getitem(&t2184, &Py::str("length"))?, py::method(&t2184, "get", &[Py::str("XL"), Py::Float(0.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2185 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2253(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2255: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2254 = v_inputs.clone(); f_inverse_dft(py::getitem(&t2254, &Py::str("re_"))?, py::method(&t2254, "get", &[Py::str("im"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2255 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_236(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t238: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t237 = v_inputs.clone(); f_bilinear_rc_lowpass(py::getitem(&t237, &Py::str("fc"))?, py::getitem(&t237, &Py::str("fs"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t238 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2501(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2503: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2502 = v_inputs.clone(); f_linear_convolution(py::getitem(&t2502, &Py::str("x"))?, py::getitem(&t2502, &Py::str("h"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2503 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2562(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2564: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2563 = v_inputs.clone(); f_moving_average(py::getitem(&t2563, &Py::str("x"))?, py::getitem(&t2563, &Py::str("M"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2564 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2596(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2598: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2597 = v_inputs.clone(); f_mpsk_symbol_error(py::getitem(&t2597, &Py::str("M"))?, py::getitem(&t2597, &Py::str("ebn0_db"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2598 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2632(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2634: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2633 = v_inputs.clone(); f_mqam_symbol_error(py::getitem(&t2633, &Py::str("M"))?, py::getitem(&t2633, &Py::str("ebn0_db"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2634 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2683(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2685: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2684 = v_inputs.clone(); f_noise_figure_cascade(py::getitem(&t2684, &Py::str("nf_db"))?, py::getitem(&t2684, &Py::str("gain_db"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2685 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2715(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2717: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2716 = v_inputs.clone(); f_noncoherent_ber(py::getitem(&t2716, &Py::str("ebn0_db"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2717 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2753(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2755: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2754 = v_inputs.clone(); f_notch_filter_design(py::getitem(&t2754, &Py::str("f0"))?, py::getitem(&t2754, &Py::str("Q"))?, py::getitem(&t2754, &Py::str("fs"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2755 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2788(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2790: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2789 = v_inputs.clone(); f_nyquist_rate(py::getitem(&t2789, &Py::str("fmax"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2790 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2823(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2825: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2824 = v_inputs.clone(); f_period_from_frequency(py::getitem(&t2824, &Py::str("f"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2825 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_286(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t288: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t287 = v_inputs.clone(); f_bin_frequency(py::getitem(&t287, &Py::str("k"))?, py::getitem(&t287, &Py::str("fs"))?, py::getitem(&t287, &Py::str("N"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t288 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2900(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2902: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2901 = v_inputs.clone(); f_periodogram(py::getitem(&t2901, &Py::str("x"))?, py::getitem(&t2901, &Py::str("fs"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2902 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3003(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3005: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3004 = v_inputs.clone(); f_poles_zeros(py::getitem(&t3004, &Py::str("b"))?, py::getitem(&t3004, &Py::str("a"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3005 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3038(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3040: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3039 = v_inputs.clone(); f_power_ratio_to_db(py::getitem(&t3039, &Py::str("ratio"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3040 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3070(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3072: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3071 = v_inputs.clone(); f_qpsk_error_rates(py::getitem(&t3071, &Py::str("ebn0_db"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3072 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3115(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3117: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3116 = v_inputs.clone(); f_quantization_snr(py::getitem(&t3116, &Py::str("bits"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3117 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3155(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3157: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3156 = v_inputs.clone(); f_quarter_wave_transformer(py::getitem(&t3156, &Py::str("RL"))?, py::getitem(&t3156, &Py::str("Z0"))?, py::method(&t3156, "get", &[Py::str("f"), Py::None])?, py::method(&t3156, "get", &[Py::str("vf"), Py::Float(1.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3157 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_318(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t320: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t319 = v_inputs.clone(); f_bpsk_ber(py::getitem(&t319, &Py::str("ebn0_db"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t320 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3195(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3197: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3196 = v_inputs.clone(); f_radar_doppler(py::getitem(&t3196, &Py::str("v"))?, py::getitem(&t3196, &Py::str("f"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3197 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3233(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3235: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3234 = v_inputs.clone(); f_radar_max_range(py::getitem(&t3234, &Py::str("Pt"))?, py::getitem(&t3234, &Py::str("G_db"))?, py::getitem(&t3234, &Py::str("f"))?, py::getitem(&t3234, &Py::str("sigma"))?, py::getitem(&t3234, &Py::str("Pmin"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3235 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3271(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3273: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3272 = v_inputs.clone(); f_radar_received_power(py::getitem(&t3272, &Py::str("Pt"))?, py::getitem(&t3272, &Py::str("G_db"))?, py::getitem(&t3272, &Py::str("f"))?, py::getitem(&t3272, &Py::str("sigma"))?, py::getitem(&t3272, &Py::str("R"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3273 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3318(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3320: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3319 = v_inputs.clone(); f_rect_pulse_spectrum(py::getitem(&t3319, &Py::str("A"))?, py::getitem(&t3319, &Py::str("tau"))?, py::getitem(&t3319, &Py::str("f"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3320 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3357(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3359: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3358 = v_inputs.clone(); f_reflection_coefficient(py::getitem(&t3358, &Py::str("RL"))?, py::getitem(&t3358, &Py::str("Z0"))?, py::method(&t3358, "get", &[Py::str("XL"), Py::Float(0.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3359 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3394(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3396: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3395 = v_inputs.clone(); f_reflection_from_vswr(py::getitem(&t3395, &Py::str("vswr"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3396 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3435(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3437: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3436 = v_inputs.clone(); f_second_order_poles(py::getitem(&t3436, &Py::str("a1"))?, py::getitem(&t3436, &Py::str("a2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3437 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3473(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3475: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3474 = v_inputs.clone(); f_shannon_capacity(py::getitem(&t3474, &Py::str("B"))?, py::getitem(&t3474, &Py::str("snr_db"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3475 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3511(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3513: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3512 = v_inputs.clone(); f_shannon_min_snr(py::getitem(&t3512, &Py::str("C"))?, py::getitem(&t3512, &Py::str("B"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3513 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3555(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3557: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3556 = v_inputs.clone(); f_signal_statistics(py::getitem(&t3556, &Py::str("x"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3557 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3590(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3592: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3591 = v_inputs.clone(); f_sine_rms(py::getitem(&t3591, &Py::str("Vp"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3592 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3626(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3628: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3627 = v_inputs.clone(); f_snr_from_powers(py::getitem(&t3627, &Py::str("Ps"))?, py::getitem(&t3627, &Py::str("Pn"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3628 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3693(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3695: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3694 = v_inputs.clone(); f_step_response(py::getitem(&t3694, &Py::str("b"))?, py::getitem(&t3694, &Py::str("a"))?, py::getitem(&t3694, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3695 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3744(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3746: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3745 = v_inputs.clone(); f_thd(py::getitem(&t3745, &Py::str("V"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3746 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3781(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3783: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3782 = v_inputs.clone(); f_vswr_from_reflection(py::getitem(&t3782, &Py::str("gamma"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3783 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3816(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3818: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3817 = v_inputs.clone(); f_watts_to_dbm(py::getitem(&t3817, &Py::str("P"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3818 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3870(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3872: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3871 = v_inputs.clone(); f_window_coefficients(py::getitem(&t3871, &Py::str("window"))?, py::getitem(&t3871, &Py::str("N"))?, py::method(&t3871, "get", &[Py::str("form"), Py::str("symmetric")])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3872 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_390(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t392: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t391 = v_inputs.clone(); f_butterworth_design(py::getitem(&t391, &Py::str("N"))?, py::getitem(&t391, &Py::str("fc"))?, py::getitem(&t391, &Py::str("fs"))?, py::getitem(&t391, &Py::str("btype"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t392 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3924(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3926: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3925 = v_inputs.clone(); f_window_mainlobe_width(py::getitem(&t3925, &Py::str("window"))?, py::getitem(&t3925, &Py::str("N"))?, py::method(&t3925, "get", &[Py::str("fs"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3926 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4029(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t4031: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4030 = v_inputs.clone(); f_z_transform_sequence(py::getitem(&t4030, &Py::str("coefficients"))?, py::getitem(&t4030, &Py::str("ratios"))?, py::method(&t4030, "get", &[Py::str("powers"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4031 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_446(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t448: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t447 = v_inputs.clone(); f_butterworth_magnitude(py::getitem(&t447, &Py::str("N"))?, py::getitem(&t447, &Py::str("fc"))?, py::getitem(&t447, &Py::str("f"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t448 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_491(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t493: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t492 = v_inputs.clone(); f_butterworth_order_analog(py::getitem(&t492, &Py::str("fp"))?, py::getitem(&t492, &Py::str("fstop"))?, py::getitem(&t492, &Py::str("Rp"))?, py::getitem(&t492, &Py::str("As"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t493 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_539(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t541: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t540 = v_inputs.clone(); f_butterworth_order_digital(py::getitem(&t540, &Py::str("fp"))?, py::getitem(&t540, &Py::str("fstop"))?, py::getitem(&t540, &Py::str("fs"))?, py::getitem(&t540, &Py::str("Rp"))?, py::getitem(&t540, &Py::str("As"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t541 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_55(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t57: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t56 = v_inputs.clone(); f_am_modulation_index(py::getitem(&t56, &Py::str("Vmax"))?, py::getitem(&t56, &Py::str("Vmin"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t57 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_584(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t586: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t585 = v_inputs.clone(); f_chebyshev1_order_analog(py::getitem(&t585, &Py::str("fp"))?, py::getitem(&t585, &Py::str("fstop"))?, py::getitem(&t585, &Py::str("Rp"))?, py::getitem(&t585, &Py::str("As"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t586 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_631(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t633: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t632 = v_inputs.clone(); f_circular_convolution(py::getitem(&t632, &Py::str("x"))?, py::getitem(&t632, &Py::str("h"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t633 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_671(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t673: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t672 = v_inputs.clone(); f_coax_impedance(py::getitem(&t672, &Py::str("D"))?, py::getitem(&t672, &Py::str("d"))?, py::method(&t672, "get", &[Py::str("er"), Py::Float(1.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t673 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_722(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t724: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t723 = v_inputs.clone(); f_cross_correlation(py::getitem(&t723, &Py::str("x"))?, py::getitem(&t723, &Py::str("y"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t724 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_754(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t756: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t755 = v_inputs.clone(); f_db_to_amplitude_ratio(py::getitem(&t755, &Py::str("dB"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t756 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_786(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t788: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t787 = v_inputs.clone(); f_db_to_power_ratio(py::getitem(&t787, &Py::str("dB"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t788 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_829(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t831: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t830 = v_inputs.clone(); f_dbm_power_sum(py::getitem(&t830, &Py::str("levels"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t831 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_861(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t863: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t862 = v_inputs.clone(); f_dbm_to_watts(py::getitem(&t862, &Py::str("dBm"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t863 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_909(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t911: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t910 = v_inputs.clone(); f_dc_gain(py::getitem(&t910, &Py::str("b"))?, py::method(&t910, "get", &[Py::str("a"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t911 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_93(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t95: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t94 = v_inputs.clone(); f_am_power(py::getitem(&t94, &Py::str("Pc"))?, py::getitem(&t94, &Py::str("m"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t95 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_976(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t978: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t977 = v_inputs.clone(); f_dft_bin(py::getitem(&t977, &Py::str("x"))?, py::getitem(&t977, &Py::str("k"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t978 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    return Ok(f__resid(v_outputs.clone(), v_want.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}
