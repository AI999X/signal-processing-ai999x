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
        "dish_antenna_gain" => {
            let r = f_dish_antenna_gain(match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eta") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1087(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "exponential_pulse_spectrum" => {
            let r = f_exponential_pulse_spectrum(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tau") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1128(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "fir_filter_output" => {
            let r = f_fir_filter_output(match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1180(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "fir_window_design" => {
            let r = f_fir_window_design(match v.get("N") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("window") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1250(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "fm_spectrum" => {
            let r = f_fm_spectrum(match v.get("df") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(1i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1306(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "fourier_series_harmonic" => {
            let r = f_fourier_series_harmonic(match v.get("waveform") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1366(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "free_space_path_loss" => {
            let r = f_free_space_path_loss(match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1404(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "frequency_from_period" => {
            let r = f_frequency_from_period(match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1439(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "frequency_resolution" => {
            let r = f_frequency_resolution(match v.get("fs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("N") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1490(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "frequency_response" => {
            let r = f_frequency_response(match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1565(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "friis_received_power" => {
            let r = f_friis_received_power(match v.get("Pt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Gt_db") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Gr_db") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1606(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "group_delay" => {
            let r = f_group_delay(match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fs") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1681(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "haar_dwt" => {
            let r = f_haar_dwt(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("levels") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Int(1i64)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1742(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "haar_idwt" => {
            let r = f_haar_idwt(match v.get("cA") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cD") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1789(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "iir_filter_output" => {
            let r = f_iir_filter_output(match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1841(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "impulse_response" => {
            let r = f_impulse_response(match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1908(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "input_impedance" => {
            let r = f_input_impedance(match v.get("RL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Z0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("length") { Some(x) => py::from_val(x), None => { return None } }, match v.get("XL") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1956(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "inverse_dft" => {
            let r = f_inverse_dft(match v.get("re_") { Some(x) => py::from_val(x), None => { return None } }, match v.get("im") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2026(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "linear_convolution" => {
            let r = f_linear_convolution(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2071(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "moving_average" => {
            let r = f_moving_average(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("M") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2132(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "mpsk_symbol_error" => {
            let r = f_mpsk_symbol_error(match v.get("M") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ebn0_db") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2166(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "mqam_symbol_error" => {
            let r = f_mqam_symbol_error(match v.get("M") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ebn0_db") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2202(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "noise_figure_cascade" => {
            let r = f_noise_figure_cascade(match v.get("nf_db") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gain_db") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2253(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "noncoherent_ber" => {
            let r = f_noncoherent_ber(match v.get("ebn0_db") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2285(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "notch_filter_design" => {
            let r = f_notch_filter_design(match v.get("f0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fs") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2323(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nyquist_rate" => {
            let r = f_nyquist_rate(match v.get("fmax") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2358(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "period_from_frequency" => {
            let r = f_period_from_frequency(match v.get("f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2393(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "periodogram" => {
            let r = f_periodogram(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fs") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2470(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "poles_zeros" => {
            let r = f_poles_zeros(match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2573(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "power_ratio_to_db" => {
            let r = f_power_ratio_to_db(match v.get("ratio") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2608(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "qpsk_error_rates" => {
            let r = f_qpsk_error_rates(match v.get("ebn0_db") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2640(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "quantization_snr" => {
            let r = f_quantization_snr(match v.get("bits") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2685(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "quarter_wave_transformer" => {
            let r = f_quarter_wave_transformer(match v.get("RL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Z0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("vf") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2725(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "radar_doppler" => {
            let r = f_radar_doppler(match v.get("v") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2765(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "radar_max_range" => {
            let r = f_radar_max_range(match v.get("Pt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("G_db") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Pmin") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2803(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "radar_received_power" => {
            let r = f_radar_received_power(match v.get("Pt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("G_db") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2841(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "rect_pulse_spectrum" => {
            let r = f_rect_pulse_spectrum(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tau") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2888(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "reflection_coefficient" => {
            let r = f_reflection_coefficient(match v.get("RL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Z0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("XL") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2927(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "reflection_from_vswr" => {
            let r = f_reflection_from_vswr(match v.get("vswr") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2964(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "second_order_poles" => {
            let r = f_second_order_poles(match v.get("a1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3005(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "shannon_capacity" => {
            let r = f_shannon_capacity(match v.get("B") { Some(x) => py::from_val(x), None => { return None } }, match v.get("snr_db") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3043(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "shannon_min_snr" => {
            let r = f_shannon_min_snr(match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("B") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3081(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "signal_statistics" => {
            let r = f_signal_statistics(match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3125(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "sine_rms" => {
            let r = f_sine_rms(match v.get("Vp") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3160(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "snr_from_powers" => {
            let r = f_snr_from_powers(match v.get("Ps") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Pn") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3196(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "step_response" => {
            let r = f_step_response(match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3263(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "thd" => {
            let r = f_thd(match v.get("V") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3314(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "vswr_from_reflection" => {
            let r = f_vswr_from_reflection(match v.get("gamma") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3351(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "watts_to_dbm" => {
            let r = f_watts_to_dbm(match v.get("P") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3386(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "window_coefficients" => {
            let r = f_window_coefficients(match v.get("window") { Some(x) => py::from_val(x), None => { return None } }, match v.get("N") { Some(x) => py::from_val(x), None => { return None } }, match v.get("form") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("symmetric")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3440(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "window_mainlobe_width" => {
            let r = f_window_mainlobe_width(match v.get("window") { Some(x) => py::from_val(x), None => { return None } }, match v.get("N") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fs") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3494(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
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
    v_real = py::sorted(&{ let mut out = vec![]; for t2519 in py::iter(&(v_roots.clone()))? { let mut v_r: Py; 
v_r = t2519;
if ({ let t2520 = py::abs(&py::imag(&v_r.clone())?)?; Py::Bool({ let t2521 = py::mul(&Py::Float(1e-12), &py::minmax(&[Py::Float(1.0), py::abs(&v_r.clone())?], true)?)?; if py::le(&t2520, &t2521)? { true } else { false } }) }).truthy() { out.push(py::add(&py::float(&py::real(&v_r.clone())?)?, &Py::Float(0.0))?); }} Py::list(out) }, (Py::Bool(false)).truthy())?;
    v_cplx = py::sorted_key(&{ let mut out = vec![]; for t2522 in py::iter(&(v_roots.clone()))? { let mut v_r: Py; 
v_r = t2522;
if ({ let t2523 = py::abs(&py::imag(&v_r.clone())?)?; Py::Bool({ let t2524 = py::mul(&Py::Float(1e-12), &py::minmax(&[Py::Float(1.0), py::abs(&v_r.clone())?], true)?)?; if py::gt(&t2523, &t2524)? { true } else { false } }) }).truthy() { out.push(py::complex(&[py::add(&py::real(&v_r.clone())?, &Py::Float(0.0))?, py::add(&py::imag(&v_r.clone())?, &Py::Float(0.0))?])?); }} Py::list(out) }, &{ let v_real = v_real.clone(); let v_roots = v_roots.clone(); py::func(move |a: &[Py]| -> R {
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
fn f__dft_direct(mut v_x: Py, mut v_sign: Py) -> R {
    let mut v_N: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_t: Py = Py::None;
    let t2449 = (Py::tuple(vec![py::len(&v_x.clone())?, Py::list(vec![])])).items()?;
    if t2449.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_N = t2449[0].clone();
    v_out = t2449[1].clone();
    for t2450 in py::range_iter(&[(v_N.clone()).i()?]) {
        v_k = t2450;
        v_t = { let mut out = vec![]; for t2451 in py::range_iter(&[(v_N.clone()).i()?]) { let mut v_n: Py; 
v_n = t2451;
out.push(py::mul(&py::complex(&[py::getitem(&v_x.clone(), &v_n.clone())?])?, &f__tw(py::mul(&v_k.clone(), &v_n.clone())?, v_N.clone(), v_sign.clone())?)?); } Py::list(out) };
        let _ = py::method(&v_out.clone(), "append", &[py::complex(&[py::fsum(&{ let mut out = vec![]; for t2452 in py::iter(&(v_t.clone()))? { let mut v_z: Py; 
v_z = t2452;
out.push(py::real(&v_z.clone())?); } Py::list(out) })?, py::fsum(&{ let mut out = vec![]; for t2453 in py::iter(&(v_t.clone()))? { let mut v_z: Py; 
v_z = t2453;
out.push(py::imag(&v_z.clone())?); } Py::list(out) })?])?])?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__diff(mut v_a: Py, mut v_b: Py) -> R {
    let mut v_d: Py = Py::None;
    if ({ let t3500 = Py::Bool(matches!(v_a.clone(), Py::Str(_))); if t3500.truthy() { t3500 } else { Py::Bool(matches!(v_b.clone(), Py::Str(_))) } }).truthy() {
        return Ok((if ({ let t3501 = v_a.clone(); Py::Bool({ let t3502 = v_b.clone(); if py::eq(&t3501, &t3502) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { c_INF()? }));
    }
    if ({ let t3503 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if t3503.truthy() { t3503 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }).truthy() {
        if ({ let t3507 = py::not(&{ let t3504 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if !t3504.truthy() { t3504 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }); if t3507.truthy() { t3507 } else { { let t3505 = py::len(&v_a.clone())?; Py::Bool({ let t3506 = py::len(&v_b.clone())?; if py::ne(&t3505, &t3506) { true } else { false } }) } } }).truthy() {
            return Ok(c_INF()?);
        }
        return Ok({ let t = { let mut out = vec![]; for t3508 in py::iter(&(py::zip(&[v_a.clone(), v_b.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t3509 = (t3508).items()?;
if t3509.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t3509[0].clone();
v_y = t3509[1].clone();
out.push(f__diff(v_x.clone(), v_y.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    }
    if ({ let t3514 = { let t3510 = v_a.clone(); Py::Bool({ let t3511 = Py::None; if py::is(&t3510, &t3511) { true } else { false } }) }; if t3514.truthy() { t3514 } else { { let t3512 = v_b.clone(); Py::Bool({ let t3513 = Py::None; if py::is(&t3512, &t3513) { true } else { false } }) } } }).truthy() {
        return Ok((if ({ let t3515 = v_a.clone(); Py::Bool({ let t3516 = v_b.clone(); if py::is(&t3515, &t3516) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { c_INF()? }));
    }
    let t3518: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            let t3517 = (Py::tuple(vec![py::complex(&[v_a.clone()])?, py::complex(&[v_b.clone()])?])).items()?;
            if t3517.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t3517[0].clone();
            v_b = t3517[1].clone();
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3518 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    if ({ let t3519 = v_a.clone(); Py::Bool({ let t3520 = v_b.clone(); if py::eq(&t3519, &t3520) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    v_d = py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?;
    if ({ let t3521 = v_d.clone(); Py::Bool({ let t3522 = v_d.clone(); if py::ne(&t3521, &t3522) { true } else { false } }) }).truthy() {
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
    return Ok((if ({ let t2440 = py::bitand(&v_N.clone(), &py::sub(&v_N.clone(), &Py::Int(1i64))?)?; Py::Bool({ let t2441 = Py::Int(0i64); if py::eq(&t2440, &t2441) { true } else { false } }) }).truthy() { f__fft2(Py::list((v_x.clone()).items()?), v_sign.clone())? } else { f__dft_direct(v_x.clone(), v_sign.clone())? }));
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
    if ({ let t2442 = v_N.clone(); Py::Bool({ let t2443 = Py::Int(1i64); if py::eq(&t2442, &t2443) { true } else { false } }) }).truthy() {
        return Ok(Py::list(vec![py::complex(&[py::getitem(&v_x.clone(), &Py::Int(0i64))?])?]));
    }
    let t2444 = (Py::tuple(vec![f__fft2(py::slice(&v_x.clone(), Some((Py::Int(0i64)).i()?), None, Some((Py::Int(2i64)).i()?))?, v_sign.clone())?, f__fft2(py::slice(&v_x.clone(), Some((Py::Int(1i64)).i()?), None, Some((Py::Int(2i64)).i()?))?, v_sign.clone())?])).items()?;
    if t2444.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ev = t2444[0].clone();
    v_od = t2444[1].clone();
    v_out = py::mul(&Py::list(vec![Py::Complex(0.0, 0.0)]), &v_N.clone())?;
    for t2445 in py::range_iter(&[(py::floordiv(&v_N.clone(), &Py::Int(2i64))?).i()?]) {
        v_k = t2445;
        v_t = py::mul(&f__tw(v_k.clone(), v_N.clone(), v_sign.clone())?, &py::getitem(&v_od.clone(), &v_k.clone())?)?;
        let t2448 = (Py::tuple(vec![py::add(&py::getitem(&v_ev.clone(), &v_k.clone())?, &v_t.clone())?, py::sub(&py::getitem(&v_ev.clone(), &v_k.clone())?, &v_t.clone())?])).items()?;
        if t2448.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        py::setitem(&v_out.clone(), &v_k.clone(), t2448[0].clone())?;
        py::setitem(&v_out.clone(), &py::add(&v_k.clone(), &py::floordiv(&v_N.clone(), &Py::Int(2i64))?)?, t2448[1].clone())?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fin(mut v_v: Py, mut v_name: Py) -> R {
    if ({ let t3041 = Py::Bool(matches!(v_v.clone(), Py::Bool(_))); if t3041.truthy() { t3041 } else { { let t3040 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_)))); if t3040.truthy() { t3040 } else { py::not(&py::math1("isfinite", &v_v.clone())?) } } } }).truthy() {
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
    let t2925 = (Py::tuple(vec![f__pos(v_RL.clone(), Py::str("load resistance"))?, f__fin(v_XL.clone(), Py::str("load reactance"))?, f__pos(v_Z0.clone(), Py::str("characteristic impedance"))?])).items()?;
    if t2925.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_RL = t2925[0].clone();
    v_XL = t2925[1].clone();
    v_Z0 = t2925[2].clone();
    v_d = py::add(&py::pow(&py::add(&v_RL.clone(), &v_Z0.clone())?, &Py::Int(2i64))?, &py::mul(&v_XL.clone(), &v_XL.clone())?)?;
    return Ok(Py::tuple(vec![py::complex(&[py::add(&py::truediv(&py::add(&py::mul(&py::sub(&v_RL.clone(), &v_Z0.clone())?, &py::add(&v_RL.clone(), &v_Z0.clone())?)?, &py::mul(&v_XL.clone(), &v_XL.clone())?)?, &v_d.clone())?, &Py::Float(0.0))?, py::add(&py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &v_Z0.clone())?, &v_XL.clone())?, &v_d.clone())?, &Py::Float(0.0))?])?, v_Z0.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__int(mut v_v: Py, mut v_name: Py, mut v_lo: Py, mut v_hi: Py) -> R {
    if ({ let t3476 = Py::Bool(matches!(v_v.clone(), Py::Bool(_))); if t3476.truthy() { t3476 } else { { let t3475 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_)))); if t3475.truthy() { t3475 } else { { let t3474 = py::not(&py::math1("isfinite", &v_v.clone())?); if t3474.truthy() { t3474 } else { { let t3472 = py::float(&v_v.clone())?; Py::Bool({ let t3473 = py::int(&v_v.clone())?; if py::ne(&t3472, &t3473) { true } else { false } }) } } } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be a whole number".to_string()].concat()))));
    }
    v_v = py::int(&v_v.clone())?;
    if ({ let t3481 = { let t3477 = v_lo.clone(); Py::Bool({ let t3478 = Py::None; if !py::is(&t3477, &t3478) { true } else { false } }) }; if !t3481.truthy() { t3481 } else { { let t3479 = v_v.clone(); Py::Bool({ let t3480 = v_lo.clone(); if py::lt(&t3479, &t3480)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be at least ".to_string(), py::format(&v_lo.clone(), "")?].concat()))));
    }
    if ({ let t3486 = { let t3482 = v_hi.clone(); Py::Bool({ let t3483 = Py::None; if !py::is(&t3482, &t3483) { true } else { false } }) }; if !t3486.truthy() { t3486 } else { { let t3484 = v_v.clone(); Py::Bool({ let t3485 = v_hi.clone(); if py::gt(&t3484, &t3485)? { true } else { false } }) } } }).truthy() {
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
    if ({ let t3254 = py::getitem(&v_a.clone(), &Py::Int(0i64))?; Py::Bool({ let t3255 = Py::Int(0i64); if py::eq(&t3254, &t3255) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a[0] must not be 0"))));
    }
    v_y = Py::list(vec![]);
    for t3256 in py::range_iter(&[(py::len(&v_x.clone())?).i()?]) {
        v_n = t3256;
        v_acc = py::fsum(&py::add(&{ let mut out = vec![]; for t3257 in py::range_iter(&[(py::len(&v_b.clone())?).i()?]) { let mut v_k: Py; 
v_k = t3257;
if ({ let t3258 = py::sub(&v_n.clone(), &v_k.clone())?; Py::Bool({ let t3259 = Py::Int(0i64); if py::ge(&t3258, &t3259)? { true } else { false } }) }).truthy() { out.push(py::mul(&py::getitem(&v_b.clone(), &v_k.clone())?, &py::getitem(&v_x.clone(), &py::sub(&v_n.clone(), &v_k.clone())?)?)?); }} Py::list(out) }, &{ let mut out = vec![]; for t3260 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::len(&v_a.clone())?).i()?]) { let mut v_k: Py; 
v_k = t3260;
if ({ let t3261 = py::sub(&v_n.clone(), &v_k.clone())?; Py::Bool({ let t3262 = Py::Int(0i64); if py::ge(&t3261, &t3262)? { true } else { false } }) }).truthy() { out.push(py::mul(&py::neg(&py::getitem(&v_a.clone(), &v_k.clone())?)?, &py::getitem(&v_y.clone(), &py::sub(&v_n.clone(), &v_k.clone())?)?)?); }} Py::list(out) })?)?;
        let _ = py::method(&v_y.clone(), "append", &[py::truediv(&v_acc.clone(), &py::getitem(&v_a.clone(), &Py::Int(0i64))?)?])?;
    }
    return Ok(v_y.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__nonneg(mut v_v: Py, mut v_name: Py) -> R {
    if ({ let t3348 = Py::Bool(matches!(v_v.clone(), Py::Bool(_))); if t3348.truthy() { t3348 } else { { let t3347 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_)))); if t3347.truthy() { t3347 } else { { let t3346 = py::not(&py::math1("isfinite", &v_v.clone())?); if t3346.truthy() { t3346 } else { { let t3344 = v_v.clone(); Py::Bool({ let t3345 = Py::Int(0i64); if py::lt(&t3344, &t3345)? { true } else { false } }) } } } } } } }).truthy() {
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
    return Ok((match ('g2464: { let mut out = vec![]; for t2465 in py::iter(&(py::enumerate(&v_mags.clone(), 0)?))? { let mut v_i: Py; let mut v_m: Py; 
let t2466 = (t2465).items()?;
if t2466.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_i = t2466[0].clone();
v_m = t2466[1].clone();
if ({ let t2467 = v_m.clone(); Py::Bool({ let t2468 = py::sub(&v_top.clone(), &py::mul(&Py::Float(1e-09), &py::minmax(&[v_scale.clone(), Py::Float(1e-300)], true)?)?)?; if py::ge(&t2467, &t2468)? { true } else { false } }) }).truthy() { out.push(v_i.clone()); break 'g2464 Py::list(out); }} Py::list(out) }).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__poly_at(mut v_c: Py, mut v_w: Py) -> R {
    let mut v_s: Py = Py::None;
    let mut v_sd: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_td: Py = Py::None;
    v_t = { let mut out = vec![]; for t1663 in py::iter(&(py::enumerate(&v_c.clone(), 0)?))? { let mut v_ck: Py; let mut v_k: Py; 
let t1664 = (t1663).items()?;
if t1664.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t1664[0].clone();
v_ck = t1664[1].clone();
out.push(py::mul(&v_ck.clone(), &py::cmath("exp", &[py::mul(&py::mul(&py::neg(&Py::Complex(0.0, 1.0))?, &v_w.clone())?, &v_k.clone())?])?)?); } Py::list(out) };
    v_td = { let mut out = vec![]; for t1665 in py::iter(&(py::enumerate(&v_t.clone(), 0)?))? { let mut v_k: Py; let mut v_z: Py; 
let t1666 = (t1665).items()?;
if t1666.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t1666[0].clone();
v_z = t1666[1].clone();
out.push(py::mul(&v_k.clone(), &v_z.clone())?); } Py::list(out) };
    v_s = py::complex(&[py::fsum(&{ let mut out = vec![]; for t1667 in py::iter(&(v_t.clone()))? { let mut v_z: Py; 
v_z = t1667;
out.push(py::real(&v_z.clone())?); } Py::list(out) })?, py::fsum(&{ let mut out = vec![]; for t1668 in py::iter(&(v_t.clone()))? { let mut v_z: Py; 
v_z = t1668;
out.push(py::imag(&v_z.clone())?); } Py::list(out) })?])?;
    v_sd = py::complex(&[py::fsum(&{ let mut out = vec![]; for t1669 in py::iter(&(v_td.clone()))? { let mut v_z: Py; 
v_z = t1669;
out.push(py::real(&v_z.clone())?); } Py::list(out) })?, py::fsum(&{ let mut out = vec![]; for t1670 in py::iter(&(v_td.clone()))? { let mut v_z: Py; 
v_z = t1670;
out.push(py::imag(&v_z.clone())?); } Py::list(out) })?])?;
    return Ok(Py::tuple(vec![v_s.clone(), v_sd.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pos(mut v_v: Py, mut v_name: Py) -> R {
    if ({ let t3493 = Py::Bool(matches!(v_v.clone(), Py::Bool(_))); if t3493.truthy() { t3493 } else { { let t3492 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_)))); if t3492.truthy() { t3492 } else { { let t3491 = py::not(&py::math1("isfinite", &v_v.clone())?); if t3491.truthy() { t3491 } else { { let t3489 = v_v.clone(); Py::Bool({ let t3490 = Py::Int(0i64); if py::le(&t3489, &t3490)? { true } else { false } }) } } } } } } }).truthy() {
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
    return Ok((if ({ let t1248 = py::abs(&v_x.clone())?; Py::Bool({ let t1249 = Py::Float(1e-15); if py::lt(&t1248, &t1249)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { py::add(&py::float(&Py::str(&[py::format(&v_x.clone(), ".12g")?].concat()))?, &Py::Float(0.0))? }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__radar_k(mut v_Pt: Py, mut v_G_db: Py, mut v_f: Py, mut v_sigma: Py) -> R {
    let mut v_G: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let t2838 = (Py::tuple(vec![f__pos(v_Pt.clone(), Py::str("peak transmit power"))?, f__pos(v_f.clone(), Py::str("frequency"))?, f__pos(v_sigma.clone(), Py::str("radar cross-section"))?])).items()?;
    if t2838.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_Pt = t2838[0].clone();
    v_f = t2838[1].clone();
    v_sigma = t2838[2].clone();
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
    for t3497 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3523 = (t3497).items()?;
        if t3523.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3523[0].clone();
        v_w = t3523[1].clone();
        if ({ let t3498 = v_k.clone(); Py::Bool({ let t3499 = v_got.clone(); if !py::contains(&t3499, &t3498)? { true } else { false } }) }).truthy() {
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
    v_c = { let mut out = vec![]; for t2525 in py::iter(&(v_c.clone()))? { let mut v_v: Py; 
v_v = t2525;
out.push(py::complex(&[v_v.clone()])?); } Py::list(out) };
    while ({ let t2528 = v_c.clone(); if !t2528.truthy() { t2528 } else { { let t2526 = py::getitem(&v_c.clone(), &Py::Int(0i64))?; Py::Bool({ let t2527 = Py::Int(0i64); if py::eq(&t2526, &t2527) { true } else { false } }) } } }).truthy() {
        let _ = py::method(&v_c.clone(), "pop", &[Py::Int(0i64)])?;
    }
    if (py::not(&v_c.clone())).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the polynomial is identically zero"))));
    }
    v_zeros = Py::Int(0i64);
    while ({ let t2533 = { let t2529 = py::len(&v_c.clone())?; Py::Bool({ let t2530 = Py::Int(1i64); if py::gt(&t2529, &t2530)? { true } else { false } }) }; if !t2533.truthy() { t2533 } else { { let t2531 = py::getitem(&v_c.clone(), &py::neg(&Py::Int(1i64))?)?; Py::Bool({ let t2532 = Py::Int(0i64); if py::eq(&t2531, &t2532) { true } else { false } }) } } }).truthy() {
        let _ = py::method(&v_c.clone(), "pop", &[])?;
        v_zeros = py::iadd(&v_zeros, &Py::Int(1i64))?;
    }
    v_n = py::sub(&py::len(&v_c.clone())?, &Py::Int(1i64))?;
    v_out = py::mul(&Py::list(vec![Py::Complex(0.0, 0.0)]), &v_zeros.clone())?;
    if ({ let t2534 = v_n.clone(); Py::Bool({ let t2535 = Py::Int(0i64); if py::eq(&t2534, &t2535) { true } else { false } }) }).truthy() {
        return Ok(v_out.clone());
    }
    v_a = { let mut out = vec![]; for t2536 in py::iter(&(v_c.clone()))? { let mut v_v: Py; 
v_v = t2536;
out.push(py::truediv(&v_v.clone(), &py::getitem(&v_c.clone(), &Py::Int(0i64))?)?); } Py::list(out) };
    if ({ let t2537 = v_n.clone(); Py::Bool({ let t2538 = Py::Int(1i64); if py::eq(&t2537, &t2538) { true } else { false } }) }).truthy() {
        return Ok(py::add(&v_out.clone(), &Py::list(vec![py::neg(&py::getitem(&v_a.clone(), &Py::Int(1i64))?)?]))?);
    }
    if ({ let t2539 = v_n.clone(); Py::Bool({ let t2540 = Py::Int(2i64); if py::eq(&t2539, &t2540) { true } else { false } }) }).truthy() {
        v_d = py::cmath("sqrt", &[py::sub(&py::mul(&py::getitem(&v_a.clone(), &Py::Int(1i64))?, &py::getitem(&v_a.clone(), &Py::Int(1i64))?)?, &py::mul(&Py::Int(4i64), &py::getitem(&v_a.clone(), &Py::Int(2i64))?)?)?])?;
        v_q = py::truediv(&py::neg(&py::add(&py::getitem(&v_a.clone(), &Py::Int(1i64))?, &(if ({ let t2541 = py::real(&py::mul(&py::method(&py::getitem(&v_a.clone(), &Py::Int(1i64))?, "conjugate", &[])?, &v_d.clone())?)?; Py::Bool({ let t2542 = Py::Int(0i64); if py::ge(&t2541, &t2542)? { true } else { false } }) }).truthy() { v_d.clone() } else { py::neg(&v_d.clone())? }))?)?, &Py::Int(2i64))?;
        return Ok(py::add(&v_out.clone(), &(if ({ let t2543 = v_q.clone(); Py::Bool({ let t2544 = Py::Int(0i64); if py::ne(&t2543, &t2544) { true } else { false } }) }).truthy() { Py::list(vec![v_q.clone(), py::truediv(&py::getitem(&v_a.clone(), &Py::Int(2i64))?, &v_q.clone())?]) } else { Py::list(vec![Py::Complex(0.0, 0.0), Py::Complex(0.0, 0.0)]) }))?);
    }
    v_p = { let v_a = v_a.clone(); let v_c = v_c.clone(); let v_d = v_d.clone(); let v_n = v_n.clone(); let v_out = v_out.clone(); let v_q = v_q.clone(); let v_zeros = v_zeros.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_z: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sum(&{ let mut out = vec![]; for t2545 in py::iter(&(py::enumerate(&v_a.clone(), 0)?))? { let mut v_ak: Py; let mut v_k: Py; 
let t2546 = (t2545).items()?;
if t2546.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2546[0].clone();
v_ak = t2546[1].clone();
out.push(py::mul(&v_ak.clone(), &py::pow(&v_z.clone(), &py::sub(&v_n.clone(), &v_k.clone())?)?)?); } Py::list(out) }, None)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_dp = { let v_a = v_a.clone(); let v_c = v_c.clone(); let v_d = v_d.clone(); let v_n = v_n.clone(); let v_out = v_out.clone(); let v_p = v_p.clone(); let v_q = v_q.clone(); let v_zeros = v_zeros.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_z: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sum(&{ let mut out = vec![]; for t2547 in py::iter(&(py::enumerate(&py::slice(&v_a.clone(), None, Some((py::neg(&Py::Int(1i64))?).i()?), None)?, 0)?))? { let mut v_ak: Py; let mut v_k: Py; 
let t2548 = (t2547).items()?;
if t2548.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2548[0].clone();
v_ak = t2548[1].clone();
out.push(py::mul(&py::mul(&v_ak.clone(), &py::sub(&v_n.clone(), &v_k.clone())?)?, &py::pow(&v_z.clone(), &py::sub(&py::sub(&v_n.clone(), &v_k.clone())?, &Py::Int(1i64))?)?)?); } Py::list(out) }, None)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_rad = py::add(&Py::Int(1i64), &py::minmax(&[{ let mut out = vec![]; for t2549 in py::iter(&(py::slice(&v_a.clone(), Some((Py::Int(1i64)).i()?), None, None)?))? { let mut v_v: Py; 
v_v = t2549;
out.push(py::abs(&v_v.clone())?); } Py::list(out) }], true)?)?;
    v_z = { let mut out = vec![]; for t2550 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t2550;
out.push(py::mul(&v_rad.clone(), &py::cmath("exp", &[py::mul(&Py::Complex(0.0, 1.0), &py::add(&py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_k.clone())?, &v_n.clone())?, &Py::Float(0.4))?)?])?)?); } Py::list(out) };
    for t2551 in py::range_iter(&[(Py::Int(500i64)).i()?]) {
        v__ = t2551;
        v_new = Py::list(vec![]);
        for t2552 in py::range_iter(&[(v_n.clone()).i()?]) {
            v_i = t2552;
            v_den = py::add(&Py::Int(1i64), &Py::Complex(0.0, 0.0))?;
            for t2553 in py::range_iter(&[(v_n.clone()).i()?]) {
                v_j = t2553;
                if ({ let t2554 = v_j.clone(); Py::Bool({ let t2555 = v_i.clone(); if py::ne(&t2554, &t2555) { true } else { false } }) }).truthy() {
                    v_den = py::mul(&v_den, &py::sub(&py::getitem(&v_z.clone(), &v_i.clone())?, &py::getitem(&v_z.clone(), &v_j.clone())?)?)?;
                }
            }
            let _ = py::method(&v_new.clone(), "append", &[(if ({ let t2556 = v_den.clone(); Py::Bool({ let t2557 = Py::Int(0i64); if py::ne(&t2556, &t2557) { true } else { false } }) }).truthy() { py::sub(&py::getitem(&v_z.clone(), &v_i.clone())?, &py::truediv(&py::call(&v_p.clone(), &[py::getitem(&v_z.clone(), &v_i.clone())?])?, &v_den.clone())?)? } else { py::add(&py::getitem(&v_z.clone(), &v_i.clone())?, &Py::Float(1e-09))? })])?;
        }
        v_done = { let t2558 = py::minmax(&[{ let mut out = vec![]; for t2560 in py::iter(&(py::zip(&[v_new.clone(), v_z.clone()])?))? { let mut v_u: Py; let mut v_v: Py; 
let t2561 = (t2560).items()?;
if t2561.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_u = t2561[0].clone();
v_v = t2561[1].clone();
out.push(py::abs(&py::sub(&v_u.clone(), &v_v.clone())?)?); } Py::list(out) }], true)?; Py::Bool({ let t2559 = py::mul(&Py::Float(1e-15), &py::minmax(&[Py::Float(1.0), py::minmax(&[{ let mut out = vec![]; for t2562 in py::iter(&(v_new.clone()))? { let mut v_v: Py; 
v_v = t2562;
out.push(py::abs(&v_v.clone())?); } Py::list(out) }], true)?], true)?)?; if py::le(&t2558, &t2559)? { true } else { false } }) };
        v_z = v_new.clone();
        if (v_done.clone()).truthy() {
            break;
        }
    }
    v_pol = Py::list(vec![]);
    for t2563 in py::iter(&(v_z.clone()))? {
        v_r = t2563;
        for t2564 in py::range_iter(&[(Py::Int(3i64)).i()?]) {
            v__ = t2564;
            v_d = py::call(&v_dp.clone(), &[v_r.clone()])?;
            if ({ let t2565 = v_d.clone(); Py::Bool({ let t2566 = Py::Int(0i64); if py::eq(&t2565, &t2566) { true } else { false } }) }).truthy() {
                break;
            }
            v_step = py::truediv(&py::call(&v_p.clone(), &[v_r.clone()])?, &v_d.clone())?;
            if ({ let t2567 = py::abs(&v_step.clone())?; Py::Bool({ let t2568 = py::mul(&Py::Float(1e-06), &py::minmax(&[Py::Float(1.0), py::abs(&v_r.clone())?], true)?)?; if py::gt(&t2567, &t2568)? { true } else { false } }) }).truthy() {
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
    return Ok(py::complex(&[py::add(&(if ({ let t2020 = py::abs(&py::real(&v_z.clone())?)?; Py::Bool({ let t2021 = v_tol.clone(); if py::gt(&t2020, &t2021)? { true } else { false } }) }).truthy() { py::real(&v_z.clone())? } else { Py::Float(0.0) }), &Py::Float(0.0))?, py::add(&(if ({ let t2022 = py::abs(&py::imag(&v_z.clone())?)?; Py::Bool({ let t2023 = v_tol.clone(); if py::gt(&t2022, &t2023)? { true } else { false } }) }).truthy() { py::imag(&v_z.clone())? } else { Py::Float(0.0) }), &Py::Float(0.0))?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__tw(mut v_m: Py, mut v_N: Py, mut v_sign: Py) -> R {
    let mut v_a: Py = Py::None;
    v_m = py::modulo(&v_m, &v_N.clone())?;
    if ({ let t2446 = py::modulo(&py::mul(&Py::Int(4i64), &v_m.clone())?, &v_N.clone())?; Py::Bool({ let t2447 = Py::Int(0i64); if py::eq(&t2446, &t2447) { true } else { false } }) }).truthy() {
        return Ok(py::getitem(&Py::tuple(vec![py::add(&Py::Int(1i64), &Py::Complex(0.0, 0.0))?, py::complex(&[Py::Int(0i64), v_sign.clone()])?, py::add(&py::neg(&Py::Int(1i64))?, &Py::Complex(0.0, 0.0))?, py::complex(&[Py::Int(0i64), py::neg(&v_sign.clone())?])?]), &py::floordiv(&py::mul(&Py::Int(4i64), &v_m.clone())?, &v_N.clone())?)?);
    }
    v_a = py::truediv(&py::mul(&py::mul(&py::mul(&v_sign.clone(), &Py::Float(2.0))?, &Py::Float(std::f64::consts::PI))?, &v_m.clone())?, &v_N.clone())?;
    return Ok(py::complex(&[py::math1("cos", &v_a.clone())?, py::math1("sin", &v_a.clone())?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__vec(mut v_v: Py, mut v_name: Py, mut v_lo: Py, mut v_hi: Py) -> R {
    if ({ let t3296 = py::not(&Py::Bool(matches!(v_v.clone(), Py::List(_) | Py::Tuple(_)))); if t3296.truthy() { t3296 } else { py::not(&Py::Bool(({ let mut out = vec![]; for t3293 in py::iter(&(v_v.clone()))? { let mut v_x: Py; 
v_x = t3293;
out.push({ let t3295 = Py::Bool(matches!(v_x.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3295.truthy() { t3295 } else { { let t3294 = py::not(&Py::Bool(matches!(v_x.clone(), Py::Bool(_)))); if !t3294.truthy() { t3294 } else { py::math1("isfinite", &v_x.clone())? } } } }); } Py::list(out) }).items()?.iter().all(|x| x.truthy()))) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be a list of finite numbers".to_string()].concat()))));
    }
    if ({ let t3297 = py::len(&v_v.clone())?; Py::Bool({ let t3298 = v_lo.clone(); if py::lt(&t3297, &t3298)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " needs at least ".to_string(), py::format(&v_lo.clone(), "")?, " values".to_string()].concat()))));
    }
    if ({ let t3303 = { let t3299 = v_hi.clone(); Py::Bool({ let t3300 = Py::None; if !py::is(&t3299, &t3300) { true } else { false } }) }; if !t3303.truthy() { t3303 } else { { let t3301 = py::len(&v_v.clone())?; Py::Bool({ let t3302 = v_hi.clone(); if py::gt(&t3301, &t3302)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " allows at most ".to_string(), py::format(&v_hi.clone(), "")?, " values".to_string()].concat()))));
    }
    return Ok({ let mut out = vec![]; for t3304 in py::iter(&(v_v.clone()))? { let mut v_x: Py; 
v_x = t3304;
out.push(py::float(&v_x.clone())?); } Py::list(out) });
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
fn f_dish_antenna_gain(mut v_D: Py, mut v_f: Py, mut v_eta: Py) -> R {
    let mut v_G: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let t1082 = (Py::tuple(vec![f__pos(v_D.clone(), Py::str("dish diameter"))?, f__pos(v_f.clone(), Py::str("frequency"))?, f__pos(v_eta.clone(), Py::str("aperture efficiency"))?])).items()?;
    if t1082.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_D = t1082[0].clone();
    v_f = t1082[1].clone();
    v_eta = t1082[2].clone();
    if ({ let t1083 = v_eta.clone(); Py::Bool({ let t1084 = Py::Int(1i64); if py::gt(&t1083, &t1084)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the aperture efficiency must be at most 1 (100 %)"))));
    }
    v_lam = py::truediv(&c_C0()?, &v_f.clone())?;
    if ({ let t1085 = v_D.clone(); Py::Bool({ let t1086 = v_lam.clone(); if py::lt(&t1085, &t1086)? { true } else { false } }) }).truthy() {
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
    let t1127 = (Py::tuple(vec![f__pos(v_A.clone(), Py::str("amplitude"))?, f__pos(v_tau.clone(), Py::str("time constant"))?, f__nonneg(v_f.clone(), Py::str("frequency"))?])).items()?;
    if t1127.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t1127[0].clone();
    v_tau = t1127[1].clone();
    v_f = t1127[2].clone();
    v_x = py::mul(&py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_f.clone())?, &v_tau.clone())?;
    return Ok(Py::dict(vec![(Py::str("magnitude"), py::truediv(&py::mul(&v_A.clone(), &v_tau.clone())?, &py::math1("sqrt", &py::add(&Py::Int(1i64), &py::mul(&v_x.clone(), &v_x.clone())?)?)?)?), (Py::str("phase"), py::add(&py::neg(&py::math1("atan", &v_x.clone())?)?, &Py::Float(0.0))?), (Py::str("re"), py::truediv(&py::mul(&v_A.clone(), &v_tau.clone())?, &py::add(&Py::Int(1i64), &py::mul(&v_x.clone(), &v_x.clone())?)?)?), (Py::str("im"), py::add(&py::truediv(&py::mul(&py::mul(&py::neg(&v_A.clone())?, &v_tau.clone())?, &v_x.clone())?, &py::add(&Py::Int(1i64), &py::mul(&v_x.clone(), &v_x.clone())?)?)?, &Py::Float(0.0))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fir_filter_output(mut v_b: Py, mut v_x: Py) -> R {
    let t1170 = (Py::tuple(vec![f__vec(v_b.clone(), Py::str("coefficients b"), Py::Int(1i64), Py::Int(10000i64))?, f__vec(v_x.clone(), Py::str("input x"), Py::Int(1i64), Py::Int(100000i64))?])).items()?;
    if t1170.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_b = t1170[0].clone();
    v_x = t1170[1].clone();
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
    let t1230 = (Py::tuple(vec![f__pos(v_fc.clone(), Py::str("cutoff frequency"))?, f__pos(v_fs.clone(), Py::str("sampling rate"))?])).items()?;
    if t1230.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fc = t1230[0].clone();
    v_fs = t1230[1].clone();
    if ({ let t1231 = v_fc.clone(); Py::Bool({ let t1232 = py::truediv(&v_fs.clone(), &Py::Int(2i64))?; if py::ge(&t1231, &t1232)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the cutoff must lie below fs/2"))));
    }
    v_w = (if ({ let t1233 = v_N.clone(); Py::Bool({ let t1234 = Py::Int(1i64); if py::gt(&t1233, &t1234)? { true } else { false } }) }).truthy() { py::getitem(&f_window_coefficients(v_window.clone(), v_N.clone(), Py::str("symmetric"))?, &Py::str("w"))? } else { Py::list(vec![Py::Float(1.0)]) });
    v_c = py::truediv(&py::mul(&Py::Int(2i64), &v_fc.clone())?, &v_fs.clone())?;
    v_alpha = py::truediv(&py::sub(&v_N.clone(), &Py::Int(1i64))?, &Py::Int(2i64))?;
    v_h = Py::list(vec![]);
    for t1244 in py::range_iter(&[(v_N.clone()).i()?]) {
        v_n = t1244;
        v_x = py::mul(&v_c.clone(), &py::sub(&v_n.clone(), &v_alpha.clone())?)?;
        let _ = py::method(&v_h.clone(), "append", &[py::mul(&py::mul(&v_c.clone(), &(if ({ let t1245 = v_x.clone(); Py::Bool({ let t1246 = Py::Int(0i64); if py::eq(&t1245, &t1246) { true } else { false } }) }).truthy() { Py::Float(1.0) } else { py::truediv(&py::math1("sin", &py::mul(&Py::Float(std::f64::consts::PI), &v_x.clone())?)?, &py::mul(&Py::Float(std::f64::consts::PI), &v_x.clone())?)? }))?, &py::getitem(&v_w.clone(), &v_n.clone())?)?])?;
    }
    v_s = py::fsum(&v_h.clone())?;
    return Ok(Py::dict(vec![(Py::str("h"), { let mut out = vec![]; for t1247 in py::iter(&(v_h.clone()))? { let mut v_v: Py; 
v_v = t1247;
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
    let t1285 = (Py::tuple(vec![f__pos(v_df.clone(), Py::str("peak frequency deviation"))?, f__pos(v_fm.clone(), Py::str("modulating frequency"))?])).items()?;
    if t1285.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_df = t1285[0].clone();
    v_fm = t1285[1].clone();
    v_n = f__int(v_n.clone(), Py::str("sideband number"), Py::Int(0i64), Py::Int(100i64))?;
    v_beta = py::truediv(&v_df.clone(), &v_fm.clone())?;
    if ({ let t1301 = v_beta.clone(); Py::Bool({ let t1302 = Py::Int(100i64); if py::gt(&t1301, &t1302)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the modulation index df/fm must be at most 100"))));
    }
    v_jn = f__besselj(v_n.clone(), v_beta.clone())?;
    v_nmax = py::int(&py::math1("floor", &py::add(&py::add(&v_beta.clone(), &Py::Int(1i64))?, &Py::Float(1e-09))?)?)?;
    v_pin = py::add(&py::pow(&f__besselj(Py::Int(0i64), v_beta.clone())?, &Py::Int(2i64))?, &py::mul(&Py::Int(2i64), &py::fsum(&{ let mut out = vec![]; for t1303 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_nmax.clone(), &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t1303;
out.push(py::pow(&f__besselj(v_k.clone(), v_beta.clone())?, &Py::Int(2i64))?); } Py::list(out) })?)?)?;
    return Ok(Py::dict(vec![(Py::str("carson_bandwidth"), py::mul(&Py::Int(2i64), &py::add(&v_df.clone(), &v_fm.clone())?)?), (Py::str("beta"), v_beta.clone()), (Py::str("carrier"), f__besselj(Py::Int(0i64), v_beta.clone())?), (Py::str("Jn"), v_jn.clone()), (Py::str("pair_power"), py::mul(&py::mul(&v_jn.clone(), &v_jn.clone())?, &(if ({ let t1304 = v_n.clone(); Py::Bool({ let t1305 = Py::Int(0i64); if py::eq(&t1304, &t1305) { true } else { false } }) }).truthy() { Py::Int(1i64) } else { Py::Int(2i64) }))?), (Py::str("power_in_carson"), v_pin.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fourier_series_harmonic(mut v_waveform: Py, mut v_A: Py, mut v_n: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_rel: Py = Py::None;
    if ({ let t1336 = v_waveform.clone(); Py::Bool({ let t1337 = c__WAVES()?; if !py::contains(&t1337, &t1336)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("waveform must be square, triangle or sawtooth"))));
    }
    let t1358 = (Py::tuple(vec![f__pos(v_A.clone(), Py::str("peak amplitude"))?, f__int(v_n.clone(), Py::str("harmonic number"), Py::Int(1i64), Py::Int(1000i64))?])).items()?;
    if t1358.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t1358[0].clone();
    v_n = t1358[1].clone();
    if ({ let t1359 = v_waveform.clone(); Py::Bool({ let t1360 = Py::str("square"); if py::eq(&t1359, &t1360) { true } else { false } }) }).truthy() {
        let t1361 = ((if (py::modulo(&v_n.clone(), &Py::Int(2i64))?).truthy() { Py::tuple(vec![py::truediv(&py::mul(&Py::Int(4i64), &v_A.clone())?, &py::mul(&v_n.clone(), &Py::Float(std::f64::consts::PI))?)?, py::truediv(&Py::Float(1.0), &v_n.clone())?]) } else { Py::tuple(vec![Py::Float(0.0), Py::Float(0.0)]) })).items()?;
        if t1361.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_c = t1361[0].clone();
        v_rel = t1361[1].clone();
    } else {
        if ({ let t1362 = v_waveform.clone(); Py::Bool({ let t1363 = Py::str("triangle"); if py::eq(&t1362, &t1363) { true } else { false } }) }).truthy() {
            let t1364 = ((if (py::modulo(&v_n.clone(), &Py::Int(2i64))?).truthy() { Py::tuple(vec![py::truediv(&py::mul(&Py::Int(8i64), &v_A.clone())?, &py::mul(&py::mul(&py::pow(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?, &v_n.clone())?, &v_n.clone())?)?, py::truediv(&Py::Float(1.0), &py::mul(&v_n.clone(), &v_n.clone())?)?]) } else { Py::tuple(vec![Py::Float(0.0), Py::Float(0.0)]) })).items()?;
            if t1364.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_c = t1364[0].clone();
            v_rel = t1364[1].clone();
        } else {
            let t1365 = (Py::tuple(vec![py::truediv(&py::mul(&Py::Int(2i64), &v_A.clone())?, &py::mul(&v_n.clone(), &Py::Float(std::f64::consts::PI))?)?, py::truediv(&Py::Float(1.0), &v_n.clone())?])).items()?;
            if t1365.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_c = t1365[0].clone();
            v_rel = t1365[1].clone();
        }
    }
    return Ok(Py::dict(vec![(Py::str("amplitude"), v_c.clone()), (Py::str("relative"), v_rel.clone()), (Py::str("rms"), py::truediv(&v_c.clone(), &py::math1("sqrt", &Py::Int(2i64))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_free_space_path_loss(mut v_d: Py, mut v_f: Py) -> R {
    let mut v_L: Py = Py::None;
    let t1401 = (Py::tuple(vec![f__pos(v_d.clone(), Py::str("distance"))?, f__pos(v_f.clone(), Py::str("frequency"))?])).items()?;
    if t1401.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_d = t1401[0].clone();
    v_f = t1401[1].clone();
    if ({ let t1402 = v_d.clone(); Py::Bool({ let t1403 = py::truediv(&c_C0()?, &v_f.clone())?; if py::lt(&t1402, &t1403)? { true } else { false } }) }).truthy() {
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
    let t1489 = (Py::tuple(vec![f__pos(v_fs.clone(), Py::str("sampling rate"))?, f__int(v_N.clone(), Py::str("number of points"), Py::Int(2i64), Py::None)?])).items()?;
    if t1489.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fs = t1489[0].clone();
    v_N = t1489[1].clone();
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
    v_a = (if ({ let t1532 = v_a.clone(); Py::Bool({ let t1533 = Py::None; if py::is(&t1532, &t1533) { true } else { false } }) }).truthy() { Py::list(vec![Py::Float(1.0)]) } else { f__vec(v_a.clone(), Py::str("a"), Py::Int(1i64), Py::Int(10000i64))? });
    let t1544 = (Py::tuple(vec![f__pos(v_fs.clone(), Py::str("sampling rate"))?, f__nonneg(v_f.clone(), Py::str("frequency"))?])).items()?;
    if t1544.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fs = t1544[0].clone();
    v_f = t1544[1].clone();
    if ({ let t1545 = v_f.clone(); Py::Bool({ let t1546 = py::mul(&py::truediv(&v_fs.clone(), &Py::Int(2i64))?, &py::add(&Py::Int(1i64), &Py::Float(1e-12))?)?; if py::gt(&t1545, &t1546)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the frequency must lie between 0 and fs/2"))));
    }
    v_w = py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_f.clone())?, &v_fs.clone())?;
    v_B = f__snap(py::getitem(&f__poly_at(v_b.clone(), v_w.clone())?, &Py::Int(0i64))?, py::fsum(&{ let mut out = vec![]; for t1559 in py::iter(&(v_b.clone()))? { let mut v_v: Py; 
v_v = t1559;
out.push(py::abs(&v_v.clone())?); } Py::list(out) })?)?;
    v_A = f__snap(py::getitem(&f__poly_at(v_a.clone(), v_w.clone())?, &Py::Int(0i64))?, py::fsum(&{ let mut out = vec![]; for t1560 in py::iter(&(v_a.clone()))? { let mut v_v: Py; 
v_v = t1560;
out.push(py::abs(&v_v.clone())?); } Py::list(out) })?)?;
    if ({ let t1561 = v_A.clone(); Py::Bool({ let t1562 = Py::Int(0i64); if py::eq(&t1561, &t1562) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the denominator vanishes at this frequency (a pole on the unit circle)"))));
    }
    v_H = py::truediv(&v_B.clone(), &v_A.clone())?;
    if ({ let t1563 = v_H.clone(); Py::Bool({ let t1564 = Py::Int(0i64); if py::eq(&t1563, &t1564) { true } else { false } }) }).truthy() {
        return Ok(Py::dict(vec![(Py::str("magnitude"), Py::Float(0.0)), (Py::str("magnitude_db"), py::neg(&c_INF()?)?), (Py::str("phase"), Py::Float(0.0))]));
    }
    return Ok(Py::dict(vec![(Py::str("magnitude"), py::abs(&v_H.clone())?), (Py::str("magnitude_db"), f__db20(py::abs(&v_H.clone())?)?), (Py::str("phase"), py::atan2(&py::imag(&v_H.clone())?, &py::real(&v_H.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_friis_received_power(mut v_Pt: Py, mut v_Gt_db: Py, mut v_Gr_db: Py, mut v_f: Py, mut v_d: Py) -> R {
    let mut v_pr_dbm: Py = Py::None;
    let t1600 = (Py::tuple(vec![f__pos(v_Pt.clone(), Py::str("transmit power"))?, f__pos(v_f.clone(), Py::str("frequency"))?, f__pos(v_d.clone(), Py::str("distance"))?])).items()?;
    if t1600.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_Pt = t1600[0].clone();
    v_f = t1600[1].clone();
    v_d = t1600[2].clone();
    let t1603 = (Py::tuple(vec![f__fin(v_Gt_db.clone(), Py::str("transmit antenna gain"))?, f__fin(v_Gr_db.clone(), Py::str("receive antenna gain"))?])).items()?;
    if t1603.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Gt_db = t1603[0].clone();
    v_Gr_db = t1603[1].clone();
    if ({ let t1604 = v_d.clone(); Py::Bool({ let t1605 = py::truediv(&c_C0()?, &v_f.clone())?; if py::lt(&t1604, &t1605)? { true } else { false } }) }).truthy() {
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
    v_a = (if ({ let t1648 = v_a.clone(); Py::Bool({ let t1649 = Py::None; if py::is(&t1648, &t1649) { true } else { false } }) }).truthy() { Py::list(vec![Py::Float(1.0)]) } else { f__vec(v_a.clone(), Py::str("a"), Py::Int(1i64), Py::Int(10000i64))? });
    let t1660 = (Py::tuple(vec![f__pos(v_fs.clone(), Py::str("sampling rate"))?, f__nonneg(v_f.clone(), Py::str("frequency"))?])).items()?;
    if t1660.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fs = t1660[0].clone();
    v_f = t1660[1].clone();
    if ({ let t1661 = v_f.clone(); Py::Bool({ let t1662 = py::mul(&py::truediv(&v_fs.clone(), &Py::Int(2i64))?, &py::add(&Py::Int(1i64), &Py::Float(1e-12))?)?; if py::gt(&t1661, &t1662)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the frequency must lie between 0 and fs/2"))));
    }
    v_w = py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_f.clone())?, &v_fs.clone())?;
    let t1671 = (Py::tuple(vec![f__poly_at(v_b.clone(), v_w.clone())?, f__poly_at(v_a.clone(), v_w.clone())?])).items()?;
    if t1671.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    let t1672 = (t1671[0].clone()).items()?;
    if t1672.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_B = t1672[0].clone();
    v_Bd = t1672[1].clone();
    let t1673 = (t1671[1].clone()).items()?;
    if t1673.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t1673[0].clone();
    v_Ad = t1673[1].clone();
    if ({ let t1680 = { let t1674 = py::abs(&v_B.clone())?; Py::Bool({ let t1675 = py::mul(&Py::Float(1e-12), &py::fsum(&{ let mut out = vec![]; for t1676 in py::iter(&(v_b.clone()))? { let mut v_v: Py; 
v_v = t1676;
out.push(py::abs(&v_v.clone())?); } Py::list(out) })?)?; if py::le(&t1674, &t1675)? { true } else { false } }) }; if t1680.truthy() { t1680 } else { { let t1677 = py::abs(&v_A.clone())?; Py::Bool({ let t1678 = py::mul(&Py::Float(1e-12), &py::fsum(&{ let mut out = vec![]; for t1679 in py::iter(&(v_a.clone()))? { let mut v_v: Py; 
v_v = t1679;
out.push(py::abs(&v_v.clone())?); } Py::list(out) })?)?; if py::le(&t1677, &t1678)? { true } else { false } }) } } }).truthy() {
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
    let t1738 = (Py::tuple(vec![v_x.clone(), Py::list(vec![])])).items()?;
    if t1738.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t1738[0].clone();
    v_details = t1738[1].clone();
    for t1739 in py::range_iter(&[(v_levels.clone()).i()?]) {
        v__ = t1739;
        let _ = py::method(&v_details.clone(), "insert", &[Py::Int(0i64), { let mut out = vec![]; for t1740 in py::range_iter(&[(py::floordiv(&py::len(&v_a.clone())?, &Py::Int(2i64))?).i()?]) { let mut v_i: Py; 
v_i = t1740;
out.push(py::mul(&c__S2()?, &py::sub(&py::getitem(&v_a.clone(), &py::mul(&Py::Int(2i64), &v_i.clone())?)?, &py::getitem(&v_a.clone(), &py::add(&py::mul(&Py::Int(2i64), &v_i.clone())?, &Py::Int(1i64))?)?)?)?); } Py::list(out) }])?;
        v_a = { let mut out = vec![]; for t1741 in py::range_iter(&[(py::floordiv(&py::len(&v_a.clone())?, &Py::Int(2i64))?).i()?]) { let mut v_i: Py; 
v_i = t1741;
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
    let t1784 = (Py::tuple(vec![f__vec(v_cA.clone(), Py::str("approximation coefficients"), Py::Int(1i64), Py::Int(32768i64))?, f__vec(v_cD.clone(), Py::str("detail coefficients"), Py::Int(1i64), Py::Int(32768i64))?])).items()?;
    if t1784.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_cA = t1784[0].clone();
    v_cD = t1784[1].clone();
    if ({ let t1785 = py::len(&v_cA.clone())?; Py::Bool({ let t1786 = py::len(&v_cD.clone())?; if py::ne(&t1785, &t1786) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the approximation and detail lists must have the same length"))));
    }
    v_x = Py::list(vec![]);
    for t1787 in py::iter(&(py::zip(&[v_cA.clone(), v_cD.clone()])?))? {
        let t1788 = (t1787).items()?;
        if t1788.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a = t1788[0].clone();
        v_d = t1788[1].clone();
        v_x = py::iadd(&v_x, &Py::list(vec![py::mul(&c__S2()?, &py::add(&v_a.clone(), &v_d.clone())?)?, py::mul(&c__S2()?, &py::sub(&v_a.clone(), &v_d.clone())?)?]))?;
    }
    return Ok(Py::dict(vec![(Py::str("x"), v_x.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_iir_filter_output(mut v_b: Py, mut v_a: Py, mut v_x: Py) -> R {
    let t1831 = (Py::tuple(vec![f__vec(v_b.clone(), Py::str("b"), Py::Int(1i64), Py::Int(1000i64))?, f__vec(v_a.clone(), Py::str("a"), Py::Int(1i64), Py::Int(1000i64))?, f__vec(v_x.clone(), Py::str("input x"), Py::Int(1i64), Py::Int(100000i64))?])).items()?;
    if t1831.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_b = t1831[0].clone();
    v_a = t1831[1].clone();
    v_x = t1831[2].clone();
    return Ok(Py::dict(vec![(Py::str("y"), f__lfilter(v_b.clone(), v_a.clone(), v_x.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_impulse_response(mut v_b: Py, mut v_a: Py, mut v_n: Py) -> R {
    let t1883 = (Py::tuple(vec![f__vec(v_b.clone(), Py::str("b"), Py::Int(1i64), Py::Int(1000i64))?, f__vec(v_a.clone(), Py::str("a"), Py::Int(1i64), Py::Int(1000i64))?])).items()?;
    if t1883.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_b = t1883[0].clone();
    v_a = t1883[1].clone();
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
    let t1946 = (f__gamma(v_RL.clone(), v_XL.clone(), v_Z0.clone())?).items()?;
    if t1946.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_g = t1946[0].clone();
    v_Z0 = t1946[1].clone();
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
    v_im = (if ({ let t1998 = v_im.clone(); Py::Bool({ let t1999 = Py::None; if py::is(&t1998, &t1999) { true } else { false } }) }).truthy() { py::mul(&Py::list(vec![Py::Float(0.0)]), &py::len(&v_re_.clone())?)? } else { f__vec(v_im.clone(), Py::str("imaginary parts"), Py::Int(1i64), Py::Int(65536i64))? });
    if ({ let t2000 = py::len(&v_im.clone())?; Py::Bool({ let t2001 = py::len(&v_re_.clone())?; if py::ne(&t2000, &t2001) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the real and imaginary parts must have the same length"))));
    }
    v_X = { let mut out = vec![]; for t2002 in py::iter(&(py::zip(&[v_re_.clone(), v_im.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t2003 = (t2002).items()?;
if t2003.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t2003[0].clone();
v_b = t2003[1].clone();
out.push(py::complex(&[v_a.clone(), v_b.clone()])?); } Py::list(out) };
    v_N = py::len(&v_X.clone())?;
    v_s = py::truediv(&py::fsum(&{ let mut out = vec![]; for t2004 in py::iter(&(v_X.clone()))? { let mut v_z: Py; 
v_z = t2004;
out.push(py::abs(&v_z.clone())?); } Py::list(out) })?, &v_N.clone())?;
    v_x = { let mut out = vec![]; for t2019 in py::iter(&(f__fft(v_X.clone(), py::pos(&Py::Int(1i64))?)?))? { let mut v_z: Py; 
v_z = t2019;
out.push(f__snap(py::truediv(&v_z.clone(), &v_N.clone())?, v_s.clone())?); } Py::list(out) };
    return Ok(Py::dict(vec![(Py::str("real"), { let mut out = vec![]; for t2024 in py::iter(&(v_x.clone()))? { let mut v_z: Py; 
v_z = t2024;
out.push(py::real(&v_z.clone())?); } Py::list(out) }), (Py::str("imag"), { let mut out = vec![]; for t2025 in py::iter(&(v_x.clone()))? { let mut v_z: Py; 
v_z = t2025;
out.push(py::imag(&v_z.clone())?); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_linear_convolution(mut v_x: Py, mut v_h: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_y: Py = Py::None;
    let t2068 = (Py::tuple(vec![f__vec(v_x.clone(), Py::str("x"), Py::Int(1i64), Py::Int(20000i64))?, f__vec(v_h.clone(), Py::str("h"), Py::Int(1i64), Py::Int(20000i64))?])).items()?;
    if t2068.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t2068[0].clone();
    v_h = t2068[1].clone();
    v_n = py::sub(&py::add(&py::len(&v_x.clone())?, &py::len(&v_h.clone())?)?, &Py::Int(1i64))?;
    v_y = { let mut out = vec![]; for t2069 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t2069;
out.push(py::fsum(&{ let mut out = vec![]; for t2070 in py::range_iter(&[(py::minmax(&[Py::Int(0i64), py::add(&py::sub(&v_i.clone(), &py::len(&v_h.clone())?)?, &Py::Int(1i64))?], true)?).i()?, (py::add(&py::minmax(&[v_i.clone(), py::sub(&py::len(&v_x.clone())?, &Py::Int(1i64))?], false)?, &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t2070;
out.push(py::mul(&py::getitem(&v_x.clone(), &v_k.clone())?, &py::getitem(&v_h.clone(), &py::sub(&v_i.clone(), &v_k.clone())?)?)?); } Py::list(out) })?); } Py::list(out) };
    return Ok(Py::dict(vec![(Py::str("y"), v_y.clone()), (Py::str("length"), v_n.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_moving_average(mut v_x: Py, mut v_M: Py) -> R {
    v_x = f__vec(v_x.clone(), Py::str("signal x"), Py::Int(1i64), Py::Int(100000i64))?;
    v_M = f__int(v_M.clone(), Py::str("window length M"), Py::Int(1i64), Py::Int(100000i64))?;
    return Ok(Py::dict(vec![(Py::str("y"), { let mut out = vec![]; for t2128 in py::range_iter(&[(py::len(&v_x.clone())?).i()?]) { let mut v_n: Py; 
v_n = t2128;
out.push(py::truediv(&py::fsum(&{ let mut out = vec![]; for t2129 in py::range_iter(&[(v_M.clone()).i()?]) { let mut v_i: Py; 
v_i = t2129;
if ({ let t2130 = py::sub(&v_n.clone(), &v_i.clone())?; Py::Bool({ let t2131 = Py::Int(0i64); if py::ge(&t2130, &t2131)? { true } else { false } }) }).truthy() { out.push(py::getitem(&v_x.clone(), &py::sub(&v_n.clone(), &v_i.clone())?)?); }} Py::list(out) })?, &v_M.clone())?); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mpsk_symbol_error(mut v_M: Py, mut v_ebn0_db: Py) -> R {
    let mut v_gs: Py = Py::None;
    let mut v_ps: Py = Py::None;
    let mut v_s2: Py = Py::None;
    let mut v_v: Py = Py::None;
    if ({ let t2162 = Py::str(&py::to_str(&v_M.clone())); Py::Bool({ let t2163 = c__PSK_M()?; if !py::contains(&t2163, &t2162)? { true } else { false } }) }).truthy() {
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
    v_M = (if ({ let t2196 = Py::str(&py::to_str(&v_M.clone())); Py::Bool({ let t2197 = c__QAM_M()?; if py::contains(&t2197, &t2196)? { true } else { false } }) }).truthy() { py::int(&v_M.clone())? } else { Py::None });
    if ({ let t2198 = v_M.clone(); Py::Bool({ let t2199 = Py::None; if py::is(&t2198, &t2199) { true } else { false } }) }).truthy() {
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
    let t2244 = (Py::tuple(vec![f__vec(v_nf_db.clone(), Py::str("noise figures in dB"), Py::Int(1i64), Py::Int(100i64))?, f__vec(v_gain_db.clone(), Py::str("gains in dB"), Py::Int(1i64), Py::Int(100i64))?])).items()?;
    if t2244.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_nf_db = t2244[0].clone();
    v_gain_db = t2244[1].clone();
    if ({ let t2245 = py::len(&v_nf_db.clone())?; Py::Bool({ let t2246 = py::len(&v_gain_db.clone())?; if py::ne(&t2245, &t2246) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give one noise figure and one gain per stage"))));
    }
    if (Py::Bool(({ let mut out = vec![]; for t2247 in py::iter(&(v_nf_db.clone()))? { let mut v_v: Py; 
v_v = t2247;
out.push({ let t2248 = v_v.clone(); Py::Bool({ let t2249 = Py::Int(0i64); if py::lt(&t2248, &t2249)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a noise figure in dB is never negative"))));
    }
    let t2250 = (Py::tuple(vec![py::pow(&Py::Int(10i64), &py::truediv(&py::getitem(&v_nf_db.clone(), &Py::Int(0i64))?, &Py::Int(10i64))?)?, py::pow(&Py::Int(10i64), &py::truediv(&py::getitem(&v_gain_db.clone(), &Py::Int(0i64))?, &Py::Int(10i64))?)?])).items()?;
    if t2250.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_F = t2250[0].clone();
    v_g = t2250[1].clone();
    v_terms = Py::list(vec![v_F.clone()]);
    for t2251 in py::iter(&(py::zip(&[py::slice(&v_nf_db.clone(), Some((Py::Int(1i64)).i()?), None, None)?, py::slice(&v_gain_db.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? {
        let t2252 = (t2251).items()?;
        if t2252.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_nf = t2252[0].clone();
        v_gd = t2252[1].clone();
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
    let t2320 = (Py::tuple(vec![f__pos(v_f0.clone(), Py::str("notch frequency"))?, f__pos(v_Q.clone(), Py::str("quality factor"))?, f__pos(v_fs.clone(), Py::str("sampling rate"))?])).items()?;
    if t2320.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_f0 = t2320[0].clone();
    v_Q = t2320[1].clone();
    v_fs = t2320[2].clone();
    if ({ let t2321 = v_f0.clone(); Py::Bool({ let t2322 = py::truediv(&v_fs.clone(), &Py::Int(2i64))?; if py::ge(&t2321, &t2322)? { true } else { false } }) }).truthy() {
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
    v_X = f__fft({ let mut out = vec![]; for t2454 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t2454;
out.push(py::sub(&v_v.clone(), &v_mu.clone())?); } Py::list(out) }, py::neg(&Py::Int(1i64))?)?;
    v_K = py::add(&py::floordiv(&v_N.clone(), &Py::Int(2i64))?, &Py::Int(1i64))?;
    v_psd = Py::list(vec![]);
    for t2455 in py::range_iter(&[(v_K.clone()).i()?]) {
        v_k = t2455;
        v_p = py::truediv(&py::pow(&py::abs(&py::getitem(&v_X.clone(), &v_k.clone())?)?, &Py::Int(2i64))?, &py::mul(&v_fs.clone(), &v_N.clone())?)?;
        if ({ let t2463 = { let t2456 = v_k.clone(); Py::Bool({ let t2457 = Py::Int(0i64); if py::ne(&t2456, &t2457) { true } else { false } }) }; if !t2463.truthy() { t2463 } else { py::not(&{ let t2462 = { let t2458 = py::modulo(&v_N.clone(), &Py::Int(2i64))?; Py::Bool({ let t2459 = Py::Int(0i64); if py::eq(&t2458, &t2459) { true } else { false } }) }; if !t2462.truthy() { t2462 } else { { let t2460 = v_k.clone(); Py::Bool({ let t2461 = py::floordiv(&v_N.clone(), &Py::Int(2i64))?; if py::eq(&t2460, &t2461) { true } else { false } }) } } }) } }).truthy() {
            v_p = py::mul(&v_p, &Py::Int(2i64))?;
        }
        let _ = py::method(&v_psd.clone(), "append", &[v_p.clone()])?;
    }
    v_i = f__peak(v_psd.clone(), py::minmax(&[v_psd.clone()], true)?)?;
    return Ok(Py::dict(vec![(Py::str("peak_frequency"), py::truediv(&py::mul(&v_i.clone(), &v_fs.clone())?, &v_N.clone())?), (Py::str("freqs"), { let mut out = vec![]; for t2469 in py::range_iter(&[(v_K.clone()).i()?]) { let mut v_k: Py; 
v_k = t2469;
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
    let t2512 = (Py::tuple(vec![f__vec(v_b.clone(), Py::str("numerator b"), Py::Int(1i64), Py::Int(5i64))?, f__vec(v_a.clone(), Py::str("denominator a"), Py::Int(1i64), Py::Int(5i64))?])).items()?;
    if t2512.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_b = t2512[0].clone();
    v_a = t2512[1].clone();
    if ({ let t2513 = py::getitem(&v_a.clone(), &Py::Int(0i64))?; Py::Bool({ let t2514 = Py::Int(0i64); if py::eq(&t2513, &t2514) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a[0] must not be 0"))));
    }
    if (Py::Bool(({ let mut out = vec![]; for t2515 in py::iter(&(v_b.clone()))? { let mut v_v: Py; 
v_v = t2515;
out.push({ let t2516 = v_v.clone(); Py::Bool({ let t2517 = Py::Int(0i64); if py::eq(&t2516, &t2517) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the numerator is identically zero"))));
    }
    let t2518 = (f__pad2(v_b.clone(), v_a.clone())?).items()?;
    if t2518.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_bp = t2518[0].clone();
    v_ap = t2518[1].clone();
    let t2569 = (Py::tuple(vec![f__canon_roots(f__roots(v_bp.clone())?)?, f__canon_roots(f__roots(v_ap.clone())?)?])).items()?;
    if t2569.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_z = t2569[0].clone();
    v_p = t2569[1].clone();
    v_rad = { let t = { let mut out = vec![]; for t2570 in py::iter(&(v_p.clone()))? { let mut v_v: Py; 
v_v = t2570;
out.push(py::abs(&v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } };
    return Ok(Py::dict(vec![(Py::str("max_pole_radius"), v_rad.clone()), (Py::str("zeros"), v_z.clone()), (Py::str("poles"), v_p.clone()), (Py::str("stability"), (if ({ let t2571 = v_rad.clone(); Py::Bool({ let t2572 = Py::Int(1i64); if py::lt(&t2571, &t2572)? { true } else { false } }) }).truthy() { Py::str("stable") } else { Py::str("unstable") }))]));
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
    let t2720 = (Py::tuple(vec![f__pos(v_RL.clone(), Py::str("load resistance"))?, f__pos(v_Z0.clone(), Py::str("line impedance to match"))?])).items()?;
    if t2720.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_RL = t2720[0].clone();
    v_Z0 = t2720[1].clone();
    v_vf = f__pos(v_vf.clone(), Py::str("velocity factor"))?;
    if ({ let t2721 = v_vf.clone(); Py::Bool({ let t2722 = Py::Int(1i64); if py::gt(&t2721, &t2722)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the velocity factor must be at most 1"))));
    }
    v_out = Py::dict(vec![(Py::str("Zt"), py::math1("sqrt", &py::mul(&v_RL.clone(), &v_Z0.clone())?)?)]);
    if ({ let t2723 = v_f.clone(); Py::Bool({ let t2724 = Py::None; if !py::is(&t2723, &t2724) { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("length"), py::truediv(&py::mul(&v_vf.clone(), &c_C0()?)?, &py::mul(&Py::Int(4i64), &f__pos(v_f.clone(), Py::str("frequency"))?)?)?)?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_radar_doppler(mut v_v: Py, mut v_f: Py) -> R {
    let t2762 = (Py::tuple(vec![f__fin(v_v.clone(), Py::str("radial velocity"))?, f__pos(v_f.clone(), Py::str("frequency"))?])).items()?;
    if t2762.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_v = t2762[0].clone();
    v_f = t2762[1].clone();
    if ({ let t2763 = py::abs(&v_v.clone())?; Py::Bool({ let t2764 = py::truediv(&c_C0()?, &Py::Int(10i64))?; if py::ge(&t2763, &t2764)? { true } else { false } }) }).truthy() {
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
    let t2883 = (Py::tuple(vec![f__fin(v_A.clone(), Py::str("amplitude"))?, f__pos(v_tau.clone(), Py::str("pulse width"))?, f__nonneg(v_f.clone(), Py::str("frequency"))?])).items()?;
    if t2883.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t2883[0].clone();
    v_tau = t2883[1].clone();
    v_f = t2883[2].clone();
    if ({ let t2884 = v_A.clone(); Py::Bool({ let t2885 = Py::Int(0i64); if py::eq(&t2884, &t2885) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the amplitude must not be 0"))));
    }
    v_x = py::mul(&v_f.clone(), &v_tau.clone())?;
    v_s = (if ({ let t2886 = v_x.clone(); Py::Bool({ let t2887 = Py::Int(0i64); if py::eq(&t2886, &t2887) { true } else { false } }) }).truthy() { Py::Float(1.0) } else { py::truediv(&py::math1("sin", &py::mul(&Py::Float(std::f64::consts::PI), &v_x.clone())?)?, &py::mul(&Py::Float(std::f64::consts::PI), &v_x.clone())?)? });
    return Ok(Py::dict(vec![(Py::str("X"), py::mul(&py::mul(&v_A.clone(), &v_tau.clone())?, &v_s.clone())?), (Py::str("magnitude"), py::abs(&py::mul(&py::mul(&v_A.clone(), &v_tau.clone())?, &v_s.clone())?)?), (Py::str("first_null"), py::truediv(&Py::Float(1.0), &v_tau.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_reflection_coefficient(mut v_RL: Py, mut v_Z0: Py, mut v_XL: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_g: Py = Py::None;
    let t2926 = (f__gamma(v_RL.clone(), v_XL.clone(), v_Z0.clone())?).items()?;
    if t2926.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_g = t2926[0].clone();
    v_Z0 = t2926[1].clone();
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
    if ({ let t2962 = v_s.clone(); Py::Bool({ let t2963 = Py::Int(1i64); if py::lt(&t2962, &t2963)? { true } else { false } }) }).truthy() {
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
    let t2996 = (Py::tuple(vec![f__fin(v_a1.clone(), Py::str("a1"))?, f__fin(v_a2.clone(), Py::str("a2"))?])).items()?;
    if t2996.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a1 = t2996[0].clone();
    v_a2 = t2996[1].clone();
    if ({ let t2997 = v_a2.clone(); Py::Bool({ let t2998 = Py::Int(0i64); if py::eq(&t2997, &t2998) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a2 must not be 0 (otherwise the system is first order)"))));
    }
    v_disc = py::sub(&py::mul(&v_a1.clone(), &v_a1.clone())?, &py::mul(&Py::Int(4i64), &v_a2.clone())?)?;
    if ({ let t2999 = v_disc.clone(); Py::Bool({ let t3000 = Py::Int(0i64); if py::ge(&t2999, &t3000)? { true } else { false } }) }).truthy() {
        v_q = py::truediv(&py::neg(&py::add(&v_a1.clone(), &py::copysign(&py::math1("sqrt", &v_disc.clone())?, &v_a1.clone())?)?)?, &Py::Int(2i64))?;
        v_r = py::sorted(&Py::list(vec![v_q.clone(), py::truediv(&v_a2.clone(), &v_q.clone())?]), (Py::Bool(false)).truthy())?;
        v_p = Py::list(vec![py::add(&py::getitem(&v_r.clone(), &Py::Int(0i64))?, &Py::Float(0.0))?, py::add(&py::getitem(&v_r.clone(), &Py::Int(1i64))?, &Py::Float(0.0))?]);
    } else {
        let t3001 = (Py::tuple(vec![py::add(&py::truediv(&py::neg(&v_a1.clone())?, &Py::Int(2i64))?, &Py::Float(0.0))?, py::truediv(&py::math1("sqrt", &py::neg(&v_disc.clone())?)?, &Py::Int(2i64))?])).items()?;
        if t3001.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_re_ = t3001[0].clone();
        v_im = t3001[1].clone();
        v_p = Py::list(vec![py::complex(&[v_re_.clone(), v_im.clone()])?, py::complex(&[v_re_.clone(), py::neg(&v_im.clone())?])?]);
    }
    v_rad = py::minmax(&[{ let mut out = vec![]; for t3002 in py::iter(&(v_p.clone()))? { let mut v_v: Py; 
v_v = t3002;
out.push(py::abs(&v_v.clone())?); } Py::list(out) }], true)?;
    return Ok(Py::dict(vec![(Py::str("radius"), v_rad.clone()), (Py::str("p1"), py::getitem(&v_p.clone(), &Py::Int(0i64))?), (Py::str("p2"), py::getitem(&v_p.clone(), &Py::Int(1i64))?), (Py::str("stability"), (if ({ let t3003 = v_rad.clone(); Py::Bool({ let t3004 = Py::Int(1i64); if py::lt(&t3003, &t3004)? { true } else { false } }) }).truthy() { Py::str("stable") } else { Py::str("unstable") }))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_shannon_capacity(mut v_B: Py, mut v_snr_db: Py) -> R {
    let mut v_C: Py = Py::None;
    let t3042 = (Py::tuple(vec![f__pos(v_B.clone(), Py::str("bandwidth"))?, f__fin(v_snr_db.clone(), Py::str("SNR in dB"))?])).items()?;
    if t3042.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_B = t3042[0].clone();
    v_snr_db = t3042[1].clone();
    v_C = py::mul(&v_B.clone(), &py::math1("log2", &py::add(&Py::Int(1i64), &py::pow(&Py::Int(10i64), &py::truediv(&v_snr_db.clone(), &Py::Int(10i64))?)?)?)?)?;
    return Ok(Py::dict(vec![(Py::str("C"), v_C.clone()), (Py::str("efficiency"), py::truediv(&v_C.clone(), &v_B.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_shannon_min_snr(mut v_C: Py, mut v_B: Py) -> R {
    let mut v_eta: Py = Py::None;
    let mut v_s: Py = Py::None;
    let t3078 = (Py::tuple(vec![f__pos(v_C.clone(), Py::str("data rate"))?, f__pos(v_B.clone(), Py::str("bandwidth"))?])).items()?;
    if t3078.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_C = t3078[0].clone();
    v_B = t3078[1].clone();
    v_eta = py::truediv(&v_C.clone(), &v_B.clone())?;
    if ({ let t3079 = v_eta.clone(); Py::Bool({ let t3080 = Py::Int(200i64); if py::gt(&t3079, &t3080)? { true } else { false } }) }).truthy() {
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
    v_e = py::fsum(&{ let mut out = vec![]; for t3123 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t3123;
out.push(py::mul(&v_v.clone(), &v_v.clone())?); } Py::list(out) })?;
    return Ok(Py::dict(vec![(Py::str("mean"), py::truediv(&py::fsum(&v_x.clone())?, &v_n.clone())?), (Py::str("rms"), py::math1("sqrt", &py::truediv(&v_e.clone(), &v_n.clone())?)?), (Py::str("energy"), v_e.clone()), (Py::str("power"), py::truediv(&v_e.clone(), &v_n.clone())?), (Py::str("peak"), py::minmax(&[{ let mut out = vec![]; for t3124 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t3124;
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
    let t3195 = (Py::tuple(vec![f__pos(v_Ps.clone(), Py::str("signal power"))?, f__pos(v_Pn.clone(), Py::str("noise power"))?])).items()?;
    if t3195.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Ps = t3195[0].clone();
    v_Pn = t3195[1].clone();
    return Ok(Py::dict(vec![(Py::str("snr"), py::truediv(&v_Ps.clone(), &v_Pn.clone())?), (Py::str("snr_db"), f__db10(py::truediv(&v_Ps.clone(), &v_Pn.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_step_response(mut v_b: Py, mut v_a: Py, mut v_n: Py) -> R {
    let t3238 = (Py::tuple(vec![f__vec(v_b.clone(), Py::str("b"), Py::Int(1i64), Py::Int(1000i64))?, f__vec(v_a.clone(), Py::str("a"), Py::Int(1i64), Py::Int(1000i64))?])).items()?;
    if t3238.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_b = t3238[0].clone();
    v_a = t3238[1].clone();
    v_n = f__int(v_n.clone(), Py::str("number of samples"), Py::Int(1i64), Py::Int(100000i64))?;
    return Ok(Py::dict(vec![(Py::str("s"), f__lfilter(v_b.clone(), v_a.clone(), py::mul(&Py::list(vec![Py::Float(1.0)]), &v_n.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_thd(mut v_V: Py) -> R {
    let mut v_t: Py = Py::None;
    v_V = f__vec(v_V.clone(), Py::str("harmonic amplitudes"), Py::Int(2i64), Py::Int(10000i64))?;
    if ({ let t3310 = { let t3305 = py::getitem(&v_V.clone(), &Py::Int(0i64))?; Py::Bool({ let t3306 = Py::Int(0i64); if py::le(&t3305, &t3306)? { true } else { false } }) }; if t3310.truthy() { t3310 } else { Py::Bool(({ let mut out = vec![]; for t3307 in py::iter(&(v_V.clone()))? { let mut v_v: Py; 
v_v = t3307;
out.push({ let t3308 = v_v.clone(); Py::Bool({ let t3309 = Py::Int(0i64); if py::lt(&t3308, &t3309)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the amplitudes must be >= 0 and the fundamental > 0"))));
    }
    v_t = py::truediv(&py::math1("sqrt", &py::fsum(&{ let mut out = vec![]; for t3311 in py::iter(&(py::slice(&v_V.clone(), Some((Py::Int(1i64)).i()?), None, None)?))? { let mut v_v: Py; 
v_v = t3311;
out.push(py::mul(&v_v.clone(), &v_v.clone())?); } Py::list(out) })?)?, &py::getitem(&v_V.clone(), &Py::Int(0i64))?)?;
    return Ok(Py::dict(vec![(Py::str("thd"), v_t.clone()), (Py::str("percent"), py::mul(&Py::Int(100i64), &v_t.clone())?), (Py::str("db"), (if ({ let t3312 = v_t.clone(); Py::Bool({ let t3313 = Py::Int(0i64); if py::gt(&t3312, &t3313)? { true } else { false } }) }).truthy() { f__db20(v_t.clone())? } else { py::neg(&c_INF()?)? }))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_vswr_from_reflection(mut v_gamma: Py) -> R {
    let mut v_a: Py = Py::None;
    v_a = f__nonneg(v_gamma.clone(), Py::str("reflection coefficient magnitude"))?;
    if ({ let t3349 = v_a.clone(); Py::Bool({ let t3350 = Py::Int(1i64); if py::ge(&t3349, &t3350)? { true } else { false } }) }).truthy() {
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
    if ({ let t3416 = v_window.clone(); Py::Bool({ let t3417 = c__MAINLOBE_BINS()?; if !py::contains(&t3417, &t3416)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&py::add(&Py::str("window must be one of "), &py::method(&Py::str(", "), "join", &[c__WINDOWS()?])?)?)));
    }
    if ({ let t3418 = v_form.clone(); Py::Bool({ let t3419 = Py::tuple(vec![Py::str("symmetric"), Py::str("periodic")]); if !py::contains(&t3419, &t3418)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("form must be symmetric or periodic"))));
    }
    v_N = f__int(v_N.clone(), Py::str("window length N"), Py::Int(2i64), Py::Int(100000i64))?;
    v_M = (if ({ let t3435 = v_form.clone(); Py::Bool({ let t3436 = Py::str("symmetric"); if py::eq(&t3435, &t3436) { true } else { false } }) }).truthy() { py::sub(&v_N.clone(), &Py::Int(1i64))? } else { v_N.clone() });
    v_coef = py::getitem(&Py::dict(vec![(Py::str("rectangular"), Py::list(vec![Py::Float(1.0)])), (Py::str("hann"), Py::list(vec![Py::Float(0.5), Py::Float(0.5)])), (Py::str("hamming"), Py::list(vec![Py::Float(0.54), Py::Float(0.46)])), (Py::str("blackman"), Py::list(vec![Py::Float(0.42), Py::Float(0.5), Py::Float(0.08)]))]), &v_window.clone())?;
    v_w = { let mut out = vec![]; for t3437 in py::range_iter(&[(v_N.clone()).i()?]) { let mut v_n: Py; 
v_n = t3437;
out.push(py::fsum(&{ let mut out = vec![]; for t3438 in py::iter(&(py::enumerate(&v_coef.clone(), 0)?))? { let mut v_c: Py; let mut v_k: Py; 
let t3439 = (t3438).items()?;
if t3439.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3439[0].clone();
v_c = t3439[1].clone();
out.push(py::mul(&py::mul(&py::pow(&py::neg(&Py::Int(1i64))?, &v_k.clone())?, &v_c.clone())?, &py::math1("cos", &py::truediv(&py::mul(&py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_k.clone())?, &v_n.clone())?, &v_M.clone())?)?)?); } Py::list(out) })?); } Py::list(out) };
    return Ok(Py::dict(vec![(Py::str("w"), v_w.clone()), (Py::str("coherent_gain"), py::truediv(&py::fsum(&v_w.clone())?, &v_N.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_window_mainlobe_width(mut v_window: Py, mut v_N: Py, mut v_fs: Py) -> R {
    let mut v_bins: Py = Py::None;
    let mut v_out: Py = Py::None;
    if ({ let t3470 = v_window.clone(); Py::Bool({ let t3471 = c__MAINLOBE_BINS()?; if !py::contains(&t3471, &t3470)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&py::add(&Py::str("window must be one of "), &py::method(&Py::str(", "), "join", &[c__WINDOWS()?])?)?)));
    }
    v_N = f__int(v_N.clone(), Py::str("window length N"), Py::Int(8i64), Py::None)?;
    v_bins = py::float(&py::getitem(&c__MAINLOBE_BINS()?, &v_window.clone())?)?;
    v_out = Py::dict(vec![(Py::str("width_bins"), v_bins.clone()), (Py::str("first_null_bins"), py::truediv(&v_bins.clone(), &Py::Int(2i64))?)]);
    if ({ let t3487 = v_fs.clone(); Py::Bool({ let t3488 = Py::None; if !py::is(&t3487, &t3488) { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("width_hz"), py::truediv(&py::mul(&v_bins.clone(), &f__pos(v_fs.clone(), Py::str("sampling rate"))?)?, &v_N.clone())?)?;
    }
    return Ok(v_out.clone());
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
fn s_1087(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t1089: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1088 = v_inputs.clone(); f_dish_antenna_gain(py::getitem(&t1088, &Py::str("D"))?, py::getitem(&t1088, &Py::str("f"))?, py::getitem(&t1088, &Py::str("eta"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1089 {
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
fn s_1128(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t1130: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1129 = v_inputs.clone(); f_exponential_pulse_spectrum(py::getitem(&t1129, &Py::str("A"))?, py::getitem(&t1129, &Py::str("tau"))?, py::getitem(&t1129, &Py::str("f"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1130 {
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
fn s_1180(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t1182: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1181 = v_inputs.clone(); f_fir_filter_output(py::getitem(&t1181, &Py::str("b"))?, py::getitem(&t1181, &Py::str("x"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1182 {
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
fn s_1250(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t1252: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1251 = v_inputs.clone(); f_fir_window_design(py::getitem(&t1251, &Py::str("N"))?, py::getitem(&t1251, &Py::str("fc"))?, py::getitem(&t1251, &Py::str("fs"))?, py::getitem(&t1251, &Py::str("window"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1252 {
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
fn s_1306(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t1308: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1307 = v_inputs.clone(); f_fm_spectrum(py::getitem(&t1307, &Py::str("df"))?, py::getitem(&t1307, &Py::str("fm"))?, py::method(&t1307, "get", &[Py::str("n"), Py::Int(1i64)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1308 {
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
fn s_1366(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t1368: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1367 = v_inputs.clone(); f_fourier_series_harmonic(py::getitem(&t1367, &Py::str("waveform"))?, py::getitem(&t1367, &Py::str("A"))?, py::getitem(&t1367, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1368 {
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
fn s_1404(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t1406: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1405 = v_inputs.clone(); f_free_space_path_loss(py::getitem(&t1405, &Py::str("d"))?, py::getitem(&t1405, &Py::str("f"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1406 {
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
fn s_1439(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t1441: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1440 = v_inputs.clone(); f_frequency_from_period(py::getitem(&t1440, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1441 {
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
fn s_1490(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t1492: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1491 = v_inputs.clone(); f_frequency_resolution(py::getitem(&t1491, &Py::str("fs"))?, py::getitem(&t1491, &Py::str("N"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1492 {
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
fn s_1565(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t1567: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1566 = v_inputs.clone(); f_frequency_response(py::getitem(&t1566, &Py::str("b"))?, py::getitem(&t1566, &Py::str("f"))?, py::getitem(&t1566, &Py::str("fs"))?, py::method(&t1566, "get", &[Py::str("a"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1567 {
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
fn s_1606(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t1608: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1607 = v_inputs.clone(); f_friis_received_power(py::getitem(&t1607, &Py::str("Pt"))?, py::getitem(&t1607, &Py::str("Gt_db"))?, py::getitem(&t1607, &Py::str("Gr_db"))?, py::getitem(&t1607, &Py::str("f"))?, py::getitem(&t1607, &Py::str("d"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1608 {
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
fn s_1681(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t1683: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1682 = v_inputs.clone(); f_group_delay(py::getitem(&t1682, &Py::str("b"))?, py::getitem(&t1682, &Py::str("f"))?, py::getitem(&t1682, &Py::str("fs"))?, py::method(&t1682, "get", &[Py::str("a"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1683 {
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
fn s_1742(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t1744: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1743 = v_inputs.clone(); f_haar_dwt(py::getitem(&t1743, &Py::str("x"))?, py::method(&t1743, "get", &[Py::str("levels"), Py::Int(1i64)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1744 {
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
fn s_1789(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t1791: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1790 = v_inputs.clone(); f_haar_idwt(py::getitem(&t1790, &Py::str("cA"))?, py::getitem(&t1790, &Py::str("cD"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1791 {
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
fn s_1841(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t1843: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1842 = v_inputs.clone(); f_iir_filter_output(py::getitem(&t1842, &Py::str("b"))?, py::getitem(&t1842, &Py::str("a"))?, py::getitem(&t1842, &Py::str("x"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1843 {
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
            v_want = { let t1909 = v_inputs.clone(); f_impulse_response(py::getitem(&t1909, &Py::str("b"))?, py::getitem(&t1909, &Py::str("a"))?, py::getitem(&t1909, &Py::str("n"))?)? };
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
fn s_1956(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t1958: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1957 = v_inputs.clone(); f_input_impedance(py::getitem(&t1957, &Py::str("RL"))?, py::getitem(&t1957, &Py::str("Z0"))?, py::getitem(&t1957, &Py::str("length"))?, py::method(&t1957, "get", &[Py::str("XL"), Py::Float(0.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1958 {
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
fn s_2026(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2028: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2027 = v_inputs.clone(); f_inverse_dft(py::getitem(&t2027, &Py::str("re_"))?, py::method(&t2027, "get", &[Py::str("im"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2028 {
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
fn s_2071(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2073: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2072 = v_inputs.clone(); f_linear_convolution(py::getitem(&t2072, &Py::str("x"))?, py::getitem(&t2072, &Py::str("h"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2073 {
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
fn s_2132(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2134: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2133 = v_inputs.clone(); f_moving_average(py::getitem(&t2133, &Py::str("x"))?, py::getitem(&t2133, &Py::str("M"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2134 {
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
fn s_2166(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2168: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2167 = v_inputs.clone(); f_mpsk_symbol_error(py::getitem(&t2167, &Py::str("M"))?, py::getitem(&t2167, &Py::str("ebn0_db"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2168 {
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
fn s_2202(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2204: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2203 = v_inputs.clone(); f_mqam_symbol_error(py::getitem(&t2203, &Py::str("M"))?, py::getitem(&t2203, &Py::str("ebn0_db"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2204 {
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
            v_want = { let t2254 = v_inputs.clone(); f_noise_figure_cascade(py::getitem(&t2254, &Py::str("nf_db"))?, py::getitem(&t2254, &Py::str("gain_db"))?)? };
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
fn s_2285(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2287: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2286 = v_inputs.clone(); f_noncoherent_ber(py::getitem(&t2286, &Py::str("ebn0_db"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2287 {
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
fn s_2323(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2325: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2324 = v_inputs.clone(); f_notch_filter_design(py::getitem(&t2324, &Py::str("f0"))?, py::getitem(&t2324, &Py::str("Q"))?, py::getitem(&t2324, &Py::str("fs"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2325 {
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
fn s_2358(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2360: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2359 = v_inputs.clone(); f_nyquist_rate(py::getitem(&t2359, &Py::str("fmax"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2360 {
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
fn s_2393(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2395: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2394 = v_inputs.clone(); f_period_from_frequency(py::getitem(&t2394, &Py::str("f"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2395 {
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
fn s_2470(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2472: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2471 = v_inputs.clone(); f_periodogram(py::getitem(&t2471, &Py::str("x"))?, py::getitem(&t2471, &Py::str("fs"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2472 {
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
fn s_2573(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2575: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2574 = v_inputs.clone(); f_poles_zeros(py::getitem(&t2574, &Py::str("b"))?, py::getitem(&t2574, &Py::str("a"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2575 {
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
fn s_2608(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2610: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2609 = v_inputs.clone(); f_power_ratio_to_db(py::getitem(&t2609, &Py::str("ratio"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2610 {
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
fn s_2640(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2642: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2641 = v_inputs.clone(); f_qpsk_error_rates(py::getitem(&t2641, &Py::str("ebn0_db"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2642 {
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
fn s_2685(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2687: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2686 = v_inputs.clone(); f_quantization_snr(py::getitem(&t2686, &Py::str("bits"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2687 {
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
fn s_2725(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2727: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2726 = v_inputs.clone(); f_quarter_wave_transformer(py::getitem(&t2726, &Py::str("RL"))?, py::getitem(&t2726, &Py::str("Z0"))?, py::method(&t2726, "get", &[Py::str("f"), Py::None])?, py::method(&t2726, "get", &[Py::str("vf"), Py::Float(1.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2727 {
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
fn s_2765(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2767: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2766 = v_inputs.clone(); f_radar_doppler(py::getitem(&t2766, &Py::str("v"))?, py::getitem(&t2766, &Py::str("f"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2767 {
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
fn s_2803(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2805: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2804 = v_inputs.clone(); f_radar_max_range(py::getitem(&t2804, &Py::str("Pt"))?, py::getitem(&t2804, &Py::str("G_db"))?, py::getitem(&t2804, &Py::str("f"))?, py::getitem(&t2804, &Py::str("sigma"))?, py::getitem(&t2804, &Py::str("Pmin"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2805 {
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
fn s_2841(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2843: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2842 = v_inputs.clone(); f_radar_received_power(py::getitem(&t2842, &Py::str("Pt"))?, py::getitem(&t2842, &Py::str("G_db"))?, py::getitem(&t2842, &Py::str("f"))?, py::getitem(&t2842, &Py::str("sigma"))?, py::getitem(&t2842, &Py::str("R"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2843 {
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
fn s_2888(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2890: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2889 = v_inputs.clone(); f_rect_pulse_spectrum(py::getitem(&t2889, &Py::str("A"))?, py::getitem(&t2889, &Py::str("tau"))?, py::getitem(&t2889, &Py::str("f"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2890 {
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
fn s_2927(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2929: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2928 = v_inputs.clone(); f_reflection_coefficient(py::getitem(&t2928, &Py::str("RL"))?, py::getitem(&t2928, &Py::str("Z0"))?, py::method(&t2928, "get", &[Py::str("XL"), Py::Float(0.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2929 {
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
fn s_2964(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2966: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2965 = v_inputs.clone(); f_reflection_from_vswr(py::getitem(&t2965, &Py::str("vswr"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2966 {
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
fn s_3005(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3007: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3006 = v_inputs.clone(); f_second_order_poles(py::getitem(&t3006, &Py::str("a1"))?, py::getitem(&t3006, &Py::str("a2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3007 {
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
fn s_3043(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3045: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3044 = v_inputs.clone(); f_shannon_capacity(py::getitem(&t3044, &Py::str("B"))?, py::getitem(&t3044, &Py::str("snr_db"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3045 {
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
fn s_3081(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3083: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3082 = v_inputs.clone(); f_shannon_min_snr(py::getitem(&t3082, &Py::str("C"))?, py::getitem(&t3082, &Py::str("B"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3083 {
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
fn s_3125(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3127: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3126 = v_inputs.clone(); f_signal_statistics(py::getitem(&t3126, &Py::str("x"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3127 {
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
fn s_3160(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3162: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3161 = v_inputs.clone(); f_sine_rms(py::getitem(&t3161, &Py::str("Vp"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3162 {
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
fn s_3196(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3198: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3197 = v_inputs.clone(); f_snr_from_powers(py::getitem(&t3197, &Py::str("Ps"))?, py::getitem(&t3197, &Py::str("Pn"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3198 {
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
fn s_3263(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3265: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3264 = v_inputs.clone(); f_step_response(py::getitem(&t3264, &Py::str("b"))?, py::getitem(&t3264, &Py::str("a"))?, py::getitem(&t3264, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3265 {
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
fn s_3314(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3316: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3315 = v_inputs.clone(); f_thd(py::getitem(&t3315, &Py::str("V"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3316 {
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
fn s_3351(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3353: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3352 = v_inputs.clone(); f_vswr_from_reflection(py::getitem(&t3352, &Py::str("gamma"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3353 {
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
fn s_3386(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3388: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3387 = v_inputs.clone(); f_watts_to_dbm(py::getitem(&t3387, &Py::str("P"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3388 {
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
fn s_3440(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3442: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3441 = v_inputs.clone(); f_window_coefficients(py::getitem(&t3441, &Py::str("window"))?, py::getitem(&t3441, &Py::str("N"))?, py::method(&t3441, "get", &[Py::str("form"), Py::str("symmetric")])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3442 {
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
fn s_3494(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3496: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3495 = v_inputs.clone(); f_window_mainlobe_width(py::getitem(&t3495, &Py::str("window"))?, py::getitem(&t3495, &Py::str("N"))?, py::method(&t3495, "get", &[Py::str("fs"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3496 {
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
