//! Every Signal Processing capability's own live figures (src/figures.rs, figures_comm.rs): its worked example gives
//! each published figure kind, a changed input changes the drawing, and no drawing holds a non-finite number or text
//! outside its canvas.

use ax_engine::figures::{figures, Figure};
use ax_engine::Input;
use signal_processing_ai999x::pack;

/// The published figure kinds of every capability (publish/website/models/signal-processing-ai999x/figures, except
/// the generic "curve"), as tools/figure_parity.py lists them. A few are drawn by the generic engine (decision maps,
/// option bars): their slugs are "-live-" ones.
const KINDS: &[(&str, &[&str])] = &[
    ("alias_frequency", &["folding"]),
    ("am_modulation_index", &["waveform"]),
    ("am_power", &["efficiency", "spectrum"]),
    ("analytic_envelope", &["envelope"]),
    ("bilinear_rc_lowpass", &["response"]),
    ("bin_frequency", &["bins"]),
    ("bpsk_ber", &["ber"]),
    ("butterworth_design", &["response", "stems"]),
    ("butterworth_magnitude", &["response"]),
    ("butterworth_order_analog", &["decision", "order"]),
    ("butterworth_order_digital", &["decision", "order"]),
    ("chebyshev1_order_analog", &["decision", "order"]),
    ("circular_convolution", &["stems"]),
    ("coax_impedance", &["section"]),
    ("cross_correlation", &["stems"]),
    ("dbm_power_sum", &["bars"]),
    ("dc_gain", &["response"]),
    ("dft_bin", &["phasors"]),
    ("dft_spectrum", &["spectrum", "stems"]),
    ("exponential_pulse_spectrum", &["spectrum"]),
    ("fir_filter_output", &["signal", "stems"]),
    ("fir_window_design", &["response", "taps"]),
    ("fm_spectrum", &["spectrum"]),
    ("fourier_series_harmonic", &["harmonics"]),
    ("free_space_path_loss", &["field"]),
    ("frequency_resolution", &["field"]),
    ("frequency_response", &["response"]),
    ("friis_received_power", &["budget"]),
    ("group_delay", &["delay"]),
    ("haar_dwt", &["coefficients"]),
    ("haar_idwt", &["coefficients"]),
    ("iir_filter_output", &["stems"]),
    ("impulse_response", &["stems"]),
    ("inverse_dft", &["stems"]),
    ("linear_convolution", &["stems"]),
    ("moving_average", &["stems"]),
    ("mpsk_symbol_error", &["ber"]),
    ("mqam_symbol_error", &["ber"]),
    ("noise_figure_cascade", &["bars"]),
    ("noncoherent_ber", &["ber"]),
    ("notch_filter_design", &["poles", "response"]),
    ("periodogram", &["spectrum"]),
    ("poles_zeros", &["poles", "poles-2"]),
    ("qpsk_error_rates", &["ber"]),
    ("radar_doppler", &["field"]),
    ("rect_pulse_spectrum", &["spectrum"]),
    ("reflection_coefficient", &["gamma"]),
    ("second_order_poles", &["decision", "poles"]),
    ("shannon_capacity", &["field"]),
    ("signal_statistics", &["stems"]),
    ("sine_rms", &["waveform"]),
    ("snr_from_powers", &["field"]),
    ("step_response", &["stems"]),
    ("thd", &["harmonics"]),
    ("window_coefficients", &["windows"]),
    ("window_mainlobe_width", &["bars"]),
];

/// Kinds the generic engine draws (its decision maps and option bars, under its own time limit natively): checked by
/// tools/figure_parity.py on the WebAssembly module, here only when present.
const GENERIC: &[(&str, &str)] = &[
    ("butterworth_order_analog", "decision"),
    ("butterworth_order_digital", "decision"),
    ("chebyshev1_order_analog", "decision"),
    ("second_order_poles", "decision"),
    ("window_mainlobe_width", "bars"),
];

/// Inputs to change for a capability whose inputs are not plain quantities (a second, different example).
const OTHER: &[(&str, &str, &str)] = &[
    ("window_coefficients", "N", "9"),
    ("butterworth_design", "N", "3"),
    ("fir_window_design", "N", "9"),
    ("dft_bin", "k", "2"),
    ("haar_dwt", "x", "1, 3, 2, 8, 5, 5, 0, 4"),
    ("fourier_series_harmonic", "waveform", "triangle"),
    ("mpsk_symbol_error", "M", "8"),
    ("mqam_symbol_error", "M", "64"),
];

/// The arguments of a structured example: solve('key', a='1 V', b=8, c="text").
fn example(key: &str) -> Vec<(String, Input)> {
    let p = pack();
    let ex = &p.procedure(key).expect(key).structured_example;
    let cs: Vec<char> = ex.chars().collect();
    let mut i = ex.find(',').map(|x| ex[..x].chars().count() + 1).unwrap_or(cs.len());
    let mut out = vec![];
    while i < cs.len() {
        while i < cs.len() && (cs[i] == ' ' || cs[i] == ',') {
            i += 1;
        }
        if i >= cs.len() || cs[i] == ')' {
            break;
        }
        let st = i;
        while cs[i] != '=' {
            i += 1;
        }
        let name: String = cs[st..i].iter().collect();
        i += 1;
        let value: String;
        if cs[i] == '\'' || cs[i] == '"' {
            let qch = cs[i];
            let s0 = i + 1;
            i += 1;
            while cs[i] != qch {
                i += 1;
            }
            value = cs[s0..i].iter().collect();
            i += 1;
        } else {
            let s0 = i;
            while i < cs.len() && cs[i] != ',' && cs[i] != ')' {
                i += 1;
            }
            value = cs[s0..i].iter().collect();
        }
        out.push((name.trim().to_string(), Input::Text(value.trim().to_string())));
    }
    out
}

fn check_texts(f: &Figure) {
    if let Err(e) = ax_engine::figkit::check_figure(f) {
        panic!("{e}");
    }
}

#[test]
fn every_capability_draws_its_published_kinds() {
    let p = pack();
    let mut missing = vec![];
    for (key, kinds) in KINDS {
        let given = example(key);
        let figs = figures(p, key, &given, None, None);
        for kind in *kinds {
            match figs.iter().find(|f| f.kind == *kind) {
                Some(f) => {
                    if !f.slug.contains("-live-") {
                        assert_eq!(f.slug, format!("{}-{kind}", key.replace('_', "-")));
                        check_texts(f);
                    }
                    assert!(!f.svg.contains("NaN") && !f.svg.contains("inf\""), "{key} {kind}: a non-finite number");
                }
                None if GENERIC.contains(&(*key, *kind)) => {}
                None => missing.push(format!("{key}: {kind}")),
            }
        }
        if let Some(i) = figs.iter().position(|f| f.slug.contains("-live-")) {
            assert!(figs[i..].iter().all(|f| f.slug.contains("-live-")), "{key}: generic figures before own ones");
        }
    }
    assert!(missing.is_empty(), "missing figures: {missing:?}");
}

/// Numbers in a text multiplied by f ("12 V" -> "16.44 V"; "3, 6" -> "4.11, 8.22").
fn scale_text(t: &str, f: f64) -> Option<String> {
    let mut out = String::new();
    let mut cur = String::new();
    let mut changed = false;
    let flush = |cur: &mut String, out: &mut String, changed: &mut bool| {
        if !cur.is_empty() {
            match cur.parse::<f64>() {
                Ok(x) if x != 0.0 => {
                    out.push_str(&format!("{}", x * f));
                    *changed = true;
                }
                _ => out.push_str(cur),
            }
            cur.clear();
        }
    };
    for c in t.chars() {
        if c.is_ascii_digit() || c == '.' || (c == '-' && cur.is_empty()) || ((c == 'e' || c == 'E') && !cur.is_empty()) {
            cur.push(c);
        } else {
            flush(&mut cur, &mut out, &mut changed);
            out.push(c);
        }
    }
    flush(&mut cur, &mut out, &mut changed);
    changed.then_some(out)
}

#[test]
fn a_changed_input_changes_the_drawing() {
    let p = pack();
    let mut unchanged = vec![];
    for (key, kinds) in KINDS {
        let given = example(key);
        let base = figures(p, key, &given, None, None);
        let own: Vec<&str> = kinds.iter().copied()
            .filter(|kd| base.iter().any(|f| f.kind == *kd && !f.slug.contains("-live-"))).collect();
        if own.is_empty() {
            continue;
        }
        let mut tries: Vec<Vec<(String, Input)>> = vec![];
        if let Some((_, sym, val)) = OTHER.iter().find(|o| o.0 == *key) {
            let mut g = given.clone();
            for e in g.iter_mut() {
                if e.0 == *sym {
                    e.1 = Input::Text(val.to_string());
                }
            }
            tries.push(g);
        }
        for f in [1.37, 0.73] {
            for i in 0..given.len() {
                if let Input::Text(t) = &given[i].1 {
                    if let Some(t2) = scale_text(t, f) {
                        let mut g = given.clone();
                        g[i].1 = Input::Text(t2);
                        tries.push(g);
                    }
                }
            }
        }
        let mut ok = false;
        for g in tries {
            let a = ax_engine::solve(p, key, &g, None);
            if !a.solved() {
                continue;
            }
            let figs = figures(p, key, &g, None, None);
            let differs = own.iter().all(|kind| {
                match (base.iter().find(|f| f.kind == *kind), figs.iter().find(|f| f.kind == *kind)) {
                    (Some(a), Some(b)) => a.svg != b.svg,
                    _ => false,
                }
            });
            if differs {
                ok = true;
                for f in figs.iter().filter(|f| !f.slug.contains("-live-")) {
                    assert!(!f.svg.contains("NaN"), "{key}: a non-finite number");
                    check_texts(f);
                }
                break;
            }
        }
        if !ok {
            unchanged.push(*key);
        }
    }
    assert!(unchanged.is_empty(), "drawing did not follow a changed input: {unchanged:?}");
}

#[test]
fn an_impossible_input_draws_nothing() {
    // a coaxial line whose inner conductor is wider than the outer one is refused: no figure, never a wrong one
    let p = pack();
    let given: Vec<(String, Input)> = [("D", "0.001 m"), ("d", "0.0035 m"), ("er", "2.25")]
        .iter().map(|(k, v)| (k.to_string(), Input::Text(v.to_string()))).collect();
    let a = ax_engine::solve(p, "coax_impedance", &given, None);
    let figs = figures(p, "coax_impedance", &given, None, None);
    if !a.solved() {
        assert!(figs.is_empty());
    } else {
        assert!(figs.iter().all(|f| f.kind != "section"), "a cross-section drawn for an impossible line");
    }
}
