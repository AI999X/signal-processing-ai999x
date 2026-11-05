//! Signal Processing's own live figures (ax_engine::figkit): every capability's published figure kinds, drawn for the
//! visitor's inputs from what the model computed (sequences as stems, filter responses from the model's own frequency
//! response, poles in the z-plane, error-rate curves, spectra, ...). Written by hand for the Rust build. A figure the
//! inputs make impossible is left out, never drawn wrong.
//!
//! Part one: sequences, filters and planes. Part two (figures_comm.rs): spectra, modulation, links, value fields.

use crate::figures_comm as cm;
use crate::figures_kit::{self as g, Panel};
use ax_engine::figkit::{self as k, num, q, Solved};
use ax_engine::figures::Figure;
use ax_engine::{Answer, Input};
use serde_json::{json, Value};

/// The figures for one solved request (the dispatcher ax_engine calls: Pack::figures).
pub fn live(s: &Solved) -> Vec<Figure> {
    let v: Vec<Option<Figure>> = match s.key {
        "circular_convolution" => vec![conv(s, true)],
        "linear_convolution" => vec![conv(s, false)],
        "cross_correlation" => vec![xcorr(s)],
        "fir_filter_output" => vec![fir_stems(s), fir_signal(s)],
        "iir_filter_output" => vec![iir_stems(s)],
        "impulse_response" => vec![impulse(s)],
        "step_response" => vec![step(s)],
        "moving_average" => vec![moving_average(s)],
        "signal_statistics" => vec![statistics(s)],
        "inverse_dft" => vec![inverse_dft(s)],
        "dft_spectrum" => vec![dft_spectrum(s), dft_stems(s)],
        "dft_bin" => vec![dft_phasors(s)],
        "haar_dwt" => vec![haar(s)],
        "haar_idwt" => vec![haar_inverse(s)],
        "butterworth_design" => vec![design_response(s), design_stems(s)],
        "poles_zeros" => vec![poles_zeros(s), poles_zeros_two(s)],
        "notch_filter_design" => vec![notch_poles(s), notch_response(s)],
        "second_order_poles" => vec![second_order(s)],
        "reflection_coefficient" => vec![gamma_plane(s)],
        "frequency_response" => vec![response(s)],
        "dc_gain" => vec![dc_response(s)],
        "bilinear_rc_lowpass" => vec![bilinear(s)],
        "group_delay" => vec![group_delay(s)],
        "fir_window_design" => vec![window_responses(s), window_taps(s)],
        "window_coefficients" => vec![windows(s)],
        _ => cm::live(s),
    };
    v.into_iter().flatten().collect()
}

// ------------------------------------------------------------------------------------------------ small helpers
pub(crate) fn jl(v: &[f64]) -> Input {
    Input::Json(json!(v))
}

/// A list as text, at most 8 entries shown: "[1, 2, 3]" or "[1, 2, ..., 9] (20 values)".
pub(crate) fn lst(v: &[f64]) -> String {
    if v.len() <= 8 {
        return format!("[{}]", v.iter().map(|x| num(*x)).collect::<Vec<_>>().join(", "));
    }
    format!("[{}, ..., {}] ({} values)", v[..3].iter().map(|x| num(*x)).collect::<Vec<_>>().join(", "),
            num(v[v.len() - 1]), v.len())
}

/// A text cut to n characters.
pub(crate) fn clip(t: &str, n: usize) -> String {
    let c: Vec<char> = t.chars().collect();
    if c.len() <= n { t.to_string() } else { format!("{}...", c[..n - 3].iter().collect::<String>().trim_end()) }
}

pub(crate) fn idx(v: &[f64]) -> Vec<(f64, f64)> {
    v.iter().enumerate().map(|(i, y)| (i as f64, *y)).collect()
}

/// A frequency axis's scale: (divisor, unit) so that ticks stay short.
pub(crate) fn hz(max: f64) -> (f64, &'static str) {
    let m = max.abs();
    if m >= 1e9 { (1e9, "GHz") } else if m >= 1e6 { (1e6, "MHz") } else if m >= 1e4 { (1e3, "kHz") } else { (1.0, "Hz") }
}

/// The model's own frequency response of b/a at f (sampling rate fs).
pub(crate) fn fresp(s: &Solved, b: &[f64], a: &[f64], f: f64, fs: f64) -> Option<Answer> {
    s.solve_other("frequency_response", &[("b", jl(b)), ("a", jl(a)), ("f", Input::Float(f)), ("fs", Input::Float(fs))])
}

/// A phase sequence without jumps of 2π.
pub(crate) fn unwrap(v: &mut [(f64, f64)]) {
    for i in 1..v.len() {
        while v[i].1 - v[i - 1].1 > std::f64::consts::PI {
            v[i].1 -= 2.0 * std::f64::consts::PI;
        }
        while v[i].1 - v[i - 1].1 < -std::f64::consts::PI {
            v[i].1 += 2.0 * std::f64::consts::PI;
        }
    }
}

/// A complex number output ({"re", "im"} or a plain number).
pub(crate) fn cx(v: &Value) -> Option<(f64, f64)> {
    match v {
        Value::Number(n) => Some((n.as_f64()?, 0.0)),
        Value::Object(o) => Some((o.get("re")?.as_f64()?, o.get("im").and_then(|x| x.as_f64()).unwrap_or(0.0))),
        _ => None,
    }
    .filter(|z| z.0.is_finite() && z.1.is_finite())
}

/// A complex list output of an answer (an empty list is a valid answer: no zeros).
pub(crate) fn cxs(a: &Answer, sym: &str) -> Option<Vec<(f64, f64)>> {
    let v = a.values.iter().find(|x| x.0 == sym)?.1.as_array()?;
    let out: Vec<(f64, f64)> = v.iter().filter_map(cx).collect();
    (out.len() == v.len()).then_some(out)
}

pub(crate) fn zs(v: &[(f64, f64)]) -> String {
    let t: Vec<String> = v.iter().take(4).map(|z| {
        if z.1.abs() < 1e-12 * z.0.abs().max(1e-300) || z.1 == 0.0 { num(z.0) } else {
            format!("{} {} {}j", num(z.0), if z.1 < 0.0 { "-" } else { "+" }, num(z.1.abs()))
        }
    }).collect();
    let more = if v.len() > 4 { format!(" (+{} more)", v.len() - 4) } else { String::new() };
    format!("{}{more}", t.join(", "))
}

// ------------------------------------------------------------------------------------------------ stacked stem panels
/// One panel of a stem figure: its series (points, colour, stems or a line), marks and guides.
pub(crate) struct Sp {
    title: String,
    xlab: String,
    ylab: String,
    ser: Vec<(Vec<(f64, f64)>, usize, bool)>,
    marks: Vec<(f64, f64, String)>,
    hl: Vec<(f64, String)>,
    ints: bool,
    span: Option<(f64, f64)>,
    dotted: bool,
}

impl Sp {
    pub(crate) fn new(title: &str, xlab: &str, ylab: &str) -> Sp {
        Sp { title: title.into(), xlab: xlab.into(), ylab: ylab.into(), ser: vec![], marks: vec![], hl: vec![], ints: true,
             span: None, dotted: false }
    }
    /// Lines of this panel get a dot at each sample.
    pub(crate) fn dotted(mut self) -> Sp {
        self.dotted = true;
        self
    }
    /// The horizontal extent to show (first and last positions), shared by panels drawn one under another.
    pub(crate) fn over(mut self, lo: f64, hi: f64) -> Sp {
        self.span = Some((lo, hi));
        self
    }
    pub(crate) fn stem(mut self, pts: Vec<(f64, f64)>, c: usize) -> Sp {
        self.ser.push((pts, c, true));
        self
    }
    pub(crate) fn line(mut self, pts: Vec<(f64, f64)>, c: usize) -> Sp {
        self.ser.push((pts, c, false));
        self
    }
    pub(crate) fn mark(mut self, x: f64, y: f64, t: &str) -> Sp {
        self.marks.push((x, y, t.into()));
        self
    }
    pub(crate) fn hline(mut self, y: f64, t: &str) -> Sp {
        self.hl.push((y, t.into()));
        self
    }
    pub(crate) fn real(mut self) -> Sp {
        self.ints = false;
        self
    }
}

/// Panels one under another (stems, lines, marks), a heading and a legend over them: the body and its height.
pub(crate) fn panels(head: &str, items: &[(String, usize)], ps: &[Sp]) -> Option<(String, f64)> {
    let n = ps.len();
    if n == 0 || ps.iter().any(|p| p.ser.iter().all(|x| x.0.is_empty())) {
        return None;
    }
    let ph = match n {
        1 => 200.0,
        2 => 128.0,
        _ => 96.0,
    };
    let lrows = if items.is_empty() { 0 } else { g::legend_rows(items) };
    let (rows, h) = g::stack_rows(n, ph, lrows);
    let mut out = String::new();
    g::header(&mut out, head, h);
    if !items.is_empty() {
        g::legend(&mut out, items, 40.0, h);
    }
    for (sp, (top, bot)) in ps.iter().zip(rows) {
        let xs: Vec<f64> = sp.ser.iter().flat_map(|x| x.0.iter().map(|p| p.0)).chain(sp.marks.iter().map(|m| m.0)).collect();
        let ys: Vec<f64> = sp.ser.iter().flat_map(|x| x.0.iter().map(|p| p.1)).chain(sp.marks.iter().map(|m| m.1))
            .chain(sp.hl.iter().map(|x| x.0)).collect();
        let stems = sp.ser.iter().any(|x| x.2);
        let yr = g::range(&ys, stems);
        let (x0, x1) = xs.iter().filter(|x| x.is_finite()).chain(sp.span.iter().flat_map(|s| [&s.0, &s.1]))
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(a, b), x| (a.min(*x), b.max(*x)));
        if !x0.is_finite() {
            return None;
        }
        let pad = if sp.ints { 0.6 } else if x1 > x0 { (x1 - x0) * 0.04 } else { 0.5 };
        let xr = (x0 - pad, x1 + pad);
        let mut p = if sp.ints { Panel::ints(xr, yr, false, top, bot, h) } else {
            Panel::new(xr, yr, (false, false), top, bot, h)
        };
        let nst = sp.ser.iter().filter(|x| x.2).count();
        let span_px = (p.xa.px(x1) - p.xa.px(x0)).abs().max(1.0);
        let npts = sp.ser.iter().filter(|x| x.2).map(|x| x.0.len()).max().unwrap_or(1).max(1);
        let gap = (span_px / npts as f64 / (nst as f64 + 1.0)).clamp(1.5, 7.0);
        let mut j = 0;
        for (pts, c, stem) in &sp.ser {
            if *stem {
                let dx = if nst > 1 { (j as f64 - (nst as f64 - 1.0) / 2.0) * gap } else { 0.0 };
                p.stems(pts, *c, dx);
                j += 1;
            } else {
                p.line(pts, *c);
                if sp.dotted && pts.len() <= 64 {
                    p.dots(pts, *c, 3.0);
                }
            }
        }
        for (y, t) in &sp.hl {
            p.hline(*y, t);
        }
        for (x, y, t) in &sp.marks {
            p.mark(*x, *y, t);
        }
        p.title(&sp.title);
        p.labels(&sp.xlab, &sp.ylab);
        out.push_str(&p.done());
    }
    Some((out, h))
}

// ------------------------------------------------------------------------------------------------ sequences
fn conv(s: &Solved, circular: bool) -> Option<Figure> {
    let (x, h) = (s.vec("x")?, s.vec("h")?);
    let y = s.out_list("y")?;
    if x.len() > 200 || h.len() > 200 {
        return None;
    }
    let what = if circular { "Circular" } else { "Linear" };
    let last = (x.len().max(h.len()).max(y.len()) - 1) as f64;
    let (body, ht) = panels(&clip(&format!("y = {}", lst(&y)), 52), &[], &[
        Sp::new("input x[n]", "", "x").stem(idx(&x), 2).over(0.0, last),
        Sp::new("impulse response h[n]", "", "h").stem(idx(&h), 2).over(0.0, last),
        Sp::new("output y[n] returned", "Sample n", "y").stem(idx(&y), 0).over(0.0, last),
    ])?;
    let extra = if circular { format!(" (length {}, indices wrap around)", y.len()) } else {
        format!(", of length {} + {} - 1 = {}", x.len(), h.len(), y.len())
    };
    Some(s.figure("stems", &format!("{what} convolution: x, h and y"),
                  &format!("{what} convolution of x = {} with h = {}: the model returns y = {}{extra}.", lst(&x), lst(&h),
                           lst(&y)), &body, ht))
}

fn xcorr(s: &Solved) -> Option<Figure> {
    let (x, y) = (s.vec("x")?, s.vec("y")?);
    let (r, lags) = (s.out_list("r")?, s.out_list("lags")?);
    let (pl, pv) = (s.out("peak_lag")?, s.out("peak_value")?);
    if r.len() != lags.len() || r.len() > 400 {
        return None;
    }
    let rl: Vec<(f64, f64)> = lags.iter().copied().zip(r.iter().copied()).collect();
    let (body, ht) = panels(&format!("peak r = {} at lag {}", num(pv), num(pl)), &[], &[
        Sp::new("signal x[n]", "", "x").stem(idx(&x), 2),
        Sp::new("signal y[n]", "", "y").stem(idx(&y), 2),
        Sp::new("cross-correlation r at each lag", "Lag", "r").stem(rl, 0).mark(pl, pv, &format!("lag {}", num(pl))),
    ])?;
    Some(s.figure("stems", "Cross-correlation: two signals and their best lag",
                  &format!("Cross-correlation of x = {} with y = {}: the model returns r at lags {} to {}, peaking at lag \
                            {} with r = {}, the shift at which y best lines up with x.", lst(&x), lst(&y),
                           num(lags[0]), num(lags[lags.len() - 1]), num(pl), num(pv)), &body, ht))
}

fn fir_stems(s: &Solved) -> Option<Figure> {
    let (b, x) = (s.vec("b")?, s.vec("x")?);
    let y = s.out_list("y")?;
    let (body, ht) = panels(&clip(&format!("y = {}", lst(&y)), 52), &[], &[
        Sp::new("filter taps b[k]", "", "b").stem(idx(&b), 2),
        Sp::new("input x[n]", "", "x").stem(idx(&x), 2),
        Sp::new("output y[n] returned", "Sample n", "y").stem(idx(&y), 0),
    ])?;
    Some(s.figure("stems", "FIR filter: taps, input and output",
                  &format!("The FIR filter b = {} applied to x = {}: the model returns y = {}, each output the \
                            taps-weighted sum of the current and previous inputs.", lst(&b), lst(&x), lst(&y)), &body, ht))
}

fn fir_signal(s: &Solved) -> Option<Figure> {
    let b = s.vec("b")?;
    if b.len() > 64 {
        return None;
    }
    // a chosen test signal: a slow tone (48 samples a period) plus a fast one (3/8 of a cycle a sample)
    let xt: Vec<f64> = (0..72).map(|n| {
        let n = n as f64;
        (2.0 * std::f64::consts::PI * n / 48.0).sin() + 0.4 * (2.0 * std::f64::consts::PI * 3.0 * n / 8.0).sin()
    }).collect();
    let a = s.with_input(&[("x", jl(&xt))])?;
    let yt = k::list_of(&a, "y")?;
    let items = vec![("test input".to_string(), 3), ("output of the filter b".to_string(), 0)];
    let (body, ht) = panels(&clip(&format!("b = {}", lst(&b)), 52), &items, &[
        Sp::new("", "Sample n", "Amplitude").line(idx(&xt), 3).line(idx(&yt), 0).real(),
    ])?;
    Some(s.figure("signal", "Filtering a signal: input and FIR output",
                  &format!("A test signal chosen for the drawing (a slow tone of 48 samples a period plus a fast tone of \
                            amplitude 0.4 at 3/8 cycle a sample, 72 samples) and its output through the FIR filter b = {} \
                            given, computed sample by sample by the model. Your own input x = {} gives y = {}.",
                           lst(&b), lst(&s.vec("x")?), lst(&s.out_list("y")?)), &body, ht))
}

fn iir_stems(s: &Solved) -> Option<Figure> {
    let (b, a, x) = (s.vec("b")?, s.vec("a")?, s.vec("x")?);
    let y = s.out_list("y")?;
    let xt = [1.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let at = s.with_input(&[("x", jl(&xt))])?;
    let yt = k::list_of(&at, "y")?;
    let items = vec![("input x".to_string(), 3), ("output y".to_string(), 0)];
    let (body, ht) = panels(&clip(&format!("b = {}, a = {}", lst(&b), lst(&a)), 52), &items, &[
        Sp::new("your input and the output returned", "Sample n", "Value").stem(idx(&x), 3).stem(idx(&y), 0),
        Sp::new("a chosen input: an impulse, then a pulse of six samples", "Sample n", "Value").stem(idx(&xt), 3)
            .stem(idx(&yt), 0),
    ])?;
    Some(s.figure("stems", "Recursive (IIR) filter: input and output",
                  &format!("The recursive filter b = {}, a = {} driven by your input x = {} (the model returns y = {}) \
                            and by an input chosen for the drawing, an impulse then a pulse of six samples, whose output \
                            the model also computes: {}.", lst(&b), lst(&a), lst(&x), lst(&y), lst(&yt)), &body, ht))
}

fn impulse(s: &Solved) -> Option<Figure> {
    let (b, a) = (s.vec("b")?, s.vec("a")?);
    let h0 = s.out_list("h")?;
    let n = h0.len().max(20);
    let h = if n == h0.len() { h0.clone() } else { k::list_of(&s.with_input(&[("n", Input::Int(n as i64))])?, "h")? };
    // the mirrored filter H(-z): every odd-index coefficient negated (poles and zeros reflected through 0)
    let flip = |v: &[f64]| -> Vec<f64> { v.iter().enumerate().map(|(i, c)| if i % 2 == 1 { -c } else { *c }).collect() };
    let (bm, am) = (flip(&b), flip(&a));
    let mut ps = Sp::new("", "Sample n", "h[n]").stem(idx(&h), 0);
    let mut items = vec![(clip(&format!("b = {}, a = {}", lst(&b), lst(&a)), 40), 0)];
    let mut note = String::new();
    if am != a || bm != b {
        if let Some(r) = s.with_input(&[("b", jl(&bm)), ("a", jl(&am)), ("n", Input::Int(n as i64))]) {
            if let Some(hm) = k::list_of(&r, "h") {
                ps = ps.stem(idx(&hm), 2);
                items.push((clip(&format!("mirrored a = {}", lst(&am)), 40), 2));
                note = format!(" Beside it, the mirrored filter b = {}, a = {} (z replaced by -z), also computed by the \
                                model: the same decay with alternating sign.", lst(&bm), lst(&am));
            }
        }
    }
    let (body, ht) = panels(&clip(&format!("h = {}", lst(&h0)), 52), &items, &[ps])?;
    Some(s.figure("stems", "Impulse response of the filter",
                  &format!("The impulse response returned by the model for b = {}, a = {}: h = {} for the {} samples \
                            asked, drawn over {} samples.{note}", lst(&b), lst(&a), lst(&h0), h0.len(), n), &body, ht))
}

fn step(s: &Solved) -> Option<Figure> {
    let (b, a) = (s.vec("b")?, s.vec("a")?);
    let s0 = s.out_list("s")?;
    let n = s0.len().max(24);
    let mut ps = Sp::new("", "Sample n", "Step response").dotted();
    let mut items = vec![];
    let mut notes = vec![];
    let mut filters: Vec<Vec<f64>> = vec![a.clone()];
    for p in [0.8, -0.5] {
        let c = vec![1.0, -p];
        if c != a {
            filters.push(c);
        }
    }
    for (i, af) in filters.iter().take(3).enumerate() {
        let r = s.with_input(&[("a", jl(af)), ("n", Input::Int(n as i64))]);
        let Some(sv) = r.as_ref().and_then(|r| k::list_of(r, "s")) else {
            if i == 0 {
                return None;
            }
            continue;
        };
        let c = [0, 1, 2][i];
        ps = ps.line(idx(&sv), c);
        let dc = s.solve_other("dc_gain", &[("b", jl(&b)), ("a", jl(af))]).and_then(|r| k::num_of(&r, "gain"));
        let name = if i == 0 { format!("yours: a = {}", lst(af)) } else { format!("a = {}", lst(af)) };
        items.push((clip(&name, 36), c));
        if let Some(d) = dc {
            ps = ps.hline(d, &format!("DC gain {}", num(d)));
            notes.push(format!("a = {}: settles at its DC gain {}", lst(af), num(d)));
        }
    }
    let (body, ht) = panels(&clip(&format!("s = {}", lst(&s0)), 52), &items, &[ps])?;
    Some(s.figure("stems", "Step responses for three pole positions",
                  &format!("Step responses returned by the model ({n} samples, one dot per sample) for b = {} with your denominator a = {} \
                            and two comparison denominators; each settles at its DC gain (dashed, from the model's DC-gain \
                            capability): {}. Your {} samples: s = {}.", lst(&b), lst(&a), notes.join("; "), s0.len(),
                           lst(&s0)), &body, ht))
}

fn moving_average(s: &Solved) -> Option<Figure> {
    let x = s.vec("x")?;
    let y = s.out_list("y")?;
    let m = s.val("M")?.int();
    let xt: Vec<f64> = (0..24).map(|n| {
        5.0 + 3.0 * (2.0 * std::f64::consts::PI * n as f64 / 16.0).sin() + if n % 3 == 0 { 2.0 } else { -1.0 }
    }).collect();
    let yt = k::list_of(&s.with_input(&[("x", jl(&xt))])?, "y")?;
    let items = vec![("input x".to_string(), 3), (format!("{m}-point average y"), 0)];
    let (body, ht) = panels(&clip(&format!("y = {}", lst(&y)), 52), &items, &[
        Sp::new("your signal", "Sample n", "Value").stem(idx(&x), 3).stem(idx(&y), 0),
        Sp::new("a chosen signal: a slow wave with a ripple on every third sample", "Sample n", "Value")
            .line(idx(&xt), 3).line(idx(&yt), 0).real(),
    ])?;
    Some(s.figure("stems", "Moving average: a smoothed signal",
                  &format!("A {m}-point moving average: your x = {} gives y = {} (returned by the model); below, a \
                            signal chosen for the drawing, x[n] = 5 + 3 sin(2πn/16) plus +2 on every third sample and -1 \
                            elsewhere, n = 0 to 23, averaged by the model: the slow wave stays and the ripple is \
                            smoothed.", lst(&x), lst(&y)), &body, ht))
}

fn statistics(s: &Solved) -> Option<Figure> {
    let x = s.vec("x")?;
    let (mean, rms, peak) = (s.out("mean")?, s.out("rms")?, s.out("peak")?);
    let (body, ht) = panels(&format!("mean {}, rms {}, peak {}", num(mean), num(rms), num(peak)), &[], &[
        Sp::new("", "Sample n", "x[n]").stem(idx(&x), 0).hline(mean, &format!("mean {}", num(mean)))
            .hline(rms, &format!("rms {}", num(rms))).hline(-rms, &format!("-rms {}", num(-rms)))
            .hline(peak, &format!("peak {}", num(peak))),
    ])?;
    Some(s.figure("stems", "Samples with their mean, rms and peak",
                  &format!("The samples x = {} with the statistics returned by the model: mean {}, rms {} (drawn at \
                            ±rms), peak |x| {}, energy {}, average power {}.", lst(&x), num(mean), num(rms), num(peak),
                           s.out("energy").map(num).unwrap_or_default(), s.out("power").map(num).unwrap_or_default()),
                  &body, ht))
}

fn inverse_dft(s: &Solved) -> Option<Figure> {
    let re = s.vec("re_")?;
    let im = s.vec("im").unwrap_or_else(|| vec![0.0; re.len()]);
    let (xr, xi) = (s.out_list("real")?, s.out_list("imag")?);
    let items = vec![("real part".to_string(), 0), ("imaginary part".to_string(), 2)];
    let (body, ht) = panels(&clip(&format!("x = {}", lst(&xr)), 52), &items, &[
        Sp::new("spectrum given: X[k]", "Bin k", "X[k]").stem(idx(&re), 0).stem(idx(&im), 2),
        Sp::new("samples returned: x[n]", "Sample n", "x[n]").stem(idx(&xr), 0).stem(idx(&xi), 2),
    ])?;
    let real = xi.iter().all(|v| v.abs() <= 1e-9 * xr.iter().fold(1.0f64, |m, x| m.max(x.abs())));
    Some(s.figure("stems", "Inverse DFT: from a spectrum back to samples",
                  &format!("The inverse DFT of the spectrum Re = {}, Im = {}: the model returns x = {} with imaginary \
                            parts {}{}.", lst(&re), lst(&im), lst(&xr), lst(&xi),
                           if real { ", a real signal (the spectrum is conjugate-symmetric)" } else { "" }), &body, ht))
}

fn dft_stems(s: &Solved) -> Option<Figure> {
    let x = s.vec("x")?;
    let (mag, ph) = (s.out_list("magnitude")?, s.out_list("phase")?);
    let (pk, pb) = (s.out("peak_magnitude")?, s.out("peak_bin")?);
    if x.len() > 512 {
        return None;
    }
    let (body, ht) = panels(&format!("largest |X[{}]| = {}", num(pb), num(pk)), &[], &[
        Sp::new("signal x[n]", "Sample n", "x").stem(idx(&x), 2),
        Sp::new("magnitude |X[k]| returned", "Bin k", "|X|").stem(idx(&mag), 0).mark(pb, pk, &format!("|X[{}]|", num(pb))),
        Sp::new("phase of X[k] returned (rad)", "Bin k", "phase").stem(idx(&ph), 3),
    ])?;
    Some(s.figure("stems", "A signal, its DFT magnitudes and phases",
                  &format!("The signal x = {} and its DFT returned by the model: magnitudes {} and phases {} rad for bins \
                            k = 0 to {}; largest |X[{}]| = {}.", lst(&x), lst(&mag), lst(&ph), mag.len() - 1, num(pb),
                           num(pk)), &body, ht))
}

fn dft_spectrum(s: &Solved) -> Option<Figure> {
    let x = s.vec("x")?;
    let mag = s.out_list("magnitude")?;
    let n = mag.len();
    if n < 2 || n > 4096 {
        return None;
    }
    let half: Vec<(f64, f64)> = (0..=n / 2).map(|i| (i as f64, mag[i])).collect();
    let (kpk, vpk) = half.iter().copied().fold((0.0, f64::NEG_INFINITY), |m, p| if p.1 > m.1 { p } else { m });
    let (body, ht) = panels(&format!("|X[{}]| = {}", num(kpk), num(vpk)), &[], &[
        Sp::new("", "Bin k (0 to N/2; the rest mirror them for a real signal)", "|X[k]|").stem(half, 0)
            .mark(kpk, vpk, &format!("bin {}", num(kpk))),
    ])?;
    Some(s.figure("spectrum", "DFT magnitude spectrum",
                  &format!("Magnitude of the {n}-point DFT returned by the model for x = {}, bins 0 to {} shown. Marked: \
                            the largest of them, |X[{}]| = {}.", lst(&x), n / 2, num(kpk), num(vpk)), &body, ht))
}

fn dft_phasors(s: &Solved) -> Option<Figure> {
    let x = s.vec("x")?;
    let k0 = s.val("k")?.int();
    let n = x.len();
    if n == 0 || n > 24 {
        return None;
    }
    let mut zsv = vec![];
    for kk in 0..n {
        let a = if kk as i64 == k0 { None } else { s.with_input(&[("k", Input::Int(kk as i64))]) };
        let (re, im) = match a {
            Some(a) => (k::num_of(&a, "re")?, k::num_of(&a, "im")?),
            None => (s.out("re")?, s.out("im")?),
        };
        zsv.push((kk, re, im));
    }
    if !zsv.iter().any(|z| z.0 as i64 == k0) {
        zsv.push((k0 as usize, s.out("re")?, s.out("im")?));
    }
    let r = zsv.iter().fold(0.0f64, |m, z| m.max(z.1.abs()).max(z.2.abs())).max(1e-12) * 1.18;
    let mut p = g::plane(r, false);
    for (kk, re, im) in &zsv {
        let mine = *kk as i64 == k0;
        p.phasor(*re, *im, if mine { 0 } else { 3 });
        if mine || n <= 8 {
            p.label_at(*re, *im, &format!("X[{kk}]"), if mine { "axp-mt" } else { "axp-lt" });
        }
    }
    p.labels("", "Imaginary part");
    let mut body = String::new();
    g::header(&mut body, &format!("X[{k0}] = {} {} {}j", num(s.out("re")?), if s.out("im")? < 0.0 { "-" } else { "+" },
                                  num(s.out("im")?.abs())), 340.0);
    body.push_str(&p.done());
    let mut notes: Vec<(String, &str, Option<usize>)> = vec![
        (format!("bin k = {k0}"), "axp-bt", Some(0)),
        (format!("|X| = {}", num(s.out("magnitude")?)), "axp-vt", None),
        (format!("phase {} rad", num(s.out("phase")?)), "axp-vt", None),
        ("other bins".to_string(), "axp-bt", Some(3)),
    ];
    for (kk, re, im) in zsv.iter().filter(|z| z.0 as i64 != k0).take(7) {
        notes.push((format!("X[{kk}] = {}", zs(&[(*re, *im)])), "axp-lt", None));
    }
    g::side_notes(&mut body, &notes, 60.0, 340.0);
    let all: Vec<String> = zsv.iter().map(|z| format!("X[{}] = {}", z.0, zs(&[(z.1, z.2)]))).collect();
    Some(s.figure("phasors", "DFT bins as phasors",
                  &format!("The DFT bins of x = {} as phasors in the complex plane (real part across), one run of the model \
                            per bin: {}. Highlighted: bin k = {k0}, |X| = {}, phase {} rad.", lst(&x), all.join("; "),
                           num(s.out("magnitude")?), num(s.out("phase")?)), &body, 340.0))
}

fn haar(s: &Solved) -> Option<Figure> {
    let x = s.vec("x")?;
    let ap = s.out_list("approximation")?;
    let det: Vec<Vec<f64>> = s.outj("details")?.as_array()?.iter()
        .map(|l| l.as_array().map(|v| v.iter().filter_map(|y| y.as_f64()).collect::<Vec<f64>>()))
        .collect::<Option<Vec<_>>>()?;
    let n = x.len();
    if n > 1024 || det.is_empty() {
        return None;
    }
    // a level-l coefficient summarises 2^l samples: drawn at their centre
    let at = |v: &[f64]| -> Option<Vec<(f64, f64)>> {
        if v.is_empty() || n % v.len() != 0 {
            return None;
        }
        let w = (n / v.len()) as f64;
        Some(v.iter().enumerate().map(|(j, c)| (j as f64 * w + (w - 1.0) / 2.0, *c)).collect())
    };
    let last = (n - 1) as f64;
    let mut dp = Sp::new("detail coefficients, each at the centre of the samples it spans", "Sample position", "detail")
        .over(0.0, last);
    let mut items = vec![];
    let mut levels = vec![];
    for d in det.iter().rev().take(4) {
        let lv = ((n / d.len().max(1)) as f64).log2().round() as usize;
        let c = (lv - 1) % 4;
        dp = dp.stem(at(d)?, c);
        items.push((format!("level {lv}"), c));
        levels.push(format!("level {lv}: {}", lst(d)));
    }
    let la = ((n / ap.len().max(1)) as f64).log2().round() as usize;
    let (body, ht) = panels(&clip(&format!("approximation {}", lst(&ap)), 52), &items, &[
        Sp::new("signal x[n]", "Sample n", "x").stem(idx(&x), 3),
        dp,
        Sp::new(&format!("approximation after level {la}"), "Sample position", "approx.").stem(at(&ap)?, 0).over(0.0, last),
    ])?;
    Some(s.figure("coefficients", "Haar wavelet coefficients level by level",
                  &format!("The Haar wavelet transform of x = {} returned by the model: each coefficient is drawn at the \
                            centre of the samples it summarises (a level-l coefficient spans 2^l samples). Details {}; \
                            approximation {}. Large details mark sharp changes.", lst(&x), levels.join("; "), lst(&ap)),
                  &body, ht))
}

fn haar_inverse(s: &Solved) -> Option<Figure> {
    let (ca, cd) = (s.vec("cA")?, s.vec("cD")?);
    let x = s.out_list("x")?;
    if ca.len() != cd.len() || x.len() != 2 * ca.len() {
        return None;
    }
    let mid = |v: &[f64]| -> Vec<(f64, f64)> { v.iter().enumerate().map(|(j, c)| (2.0 * j as f64 + 0.5, *c)).collect() };
    let items = vec![("approximation cA".to_string(), 2), ("detail cD".to_string(), 3)];
    let (body, ht) = panels(&clip(&format!("x = {}", lst(&x)), 52), &items, &[
        Sp::new("coefficients given, at the centre of the two samples each rebuilds", "Sample position", "coefficient")
            .stem(mid(&ca), 2).stem(mid(&cd), 3).over(0.0, (x.len() - 1) as f64),
        Sp::new("samples returned", "Sample n", "x[n]").stem(idx(&x), 0),
    ])?;
    Some(s.figure("coefficients", "Inverse Haar step: coefficients to samples",
                  &format!("One level of the inverse Haar transform: approximation cA = {} and details cD = {} (inputs) \
                            give x = {} (returned by the model): each pair is (cA ± cD)/√2.", lst(&ca), lst(&cd), lst(&x)),
                  &body, ht))
}

// ------------------------------------------------------------------------------------------------ filter responses
/// Gain (dB) and phase of b/a over `fr` from the model's frequency response: (gain points, phase points).
fn gain_phase(s: &Solved, b: &[f64], a: &[f64], fr: &[f64], fs: f64, floor_db: f64) -> (Vec<(f64, f64)>, Vec<(f64, f64)>) {
    let (mut gpts, mut ppts) = (vec![], vec![]);
    for f in fr {
        if let Some(r) = fresp(s, b, a, *f, fs) {
            if let (Some(db), Some(ph)) = (k::num_of(&r, "magnitude_db"), k::num_of(&r, "phase")) {
                gpts.push((*f, if db < floor_db { f64::NAN } else { db }));
                ppts.push((*f, ph));
            } else if let Some(ph) = k::num_of(&r, "phase") {
                // a gain of exactly zero: no dB value
                gpts.push((*f, f64::NAN));
                ppts.push((*f, ph));
            }
        }
    }
    unwrap(&mut ppts);
    (gpts, ppts)
}

fn design_response(s: &Solved) -> Option<Figure> {
    let (n, fc, fs) = (s.val("N")?.int(), s.inp("fc")?, s.inp("fs")?);
    let bt = s.txt("btype")?;
    let (b, a) = (s.out_list("b")?, s.out_list("a")?);
    let other = if bt == "lowpass" { "highpass" } else { "lowpass" };
    let mut designs = vec![(format!("yours: order {n} {bt}"), b.clone(), a.clone())];
    if let Some(r) = s.with_input(&[("btype", Input::Text(other.into()))]) {
        designs.push((format!("order {n} {other}"), k::list_of(&r, "b")?, k::list_of(&r, "a")?));
    }
    let n2 = if n + 2 <= 8 { n + 2 } else { (n - 2).max(1) };
    if n2 != n {
        if let Some(r) = s.with_input(&[("N", Input::Int(n2))]) {
            designs.push((format!("order {n2} {bt}"), k::list_of(&r, "b")?, k::list_of(&r, "a")?));
        }
    }
    let fr = k::span(fs / 400.0, fs / 2.0 * 0.995, 61, true);
    let (mut gs, mut ps) = (vec![], vec![]);
    for (name, bb, aa) in &designs {
        let (gp, pp) = gain_phase(s, bb, aa, &fr, fs, -100.0);
        gs.push((name.clone(), gp));
        ps.push((name.clone(), pp));
    }
    let gfc = fresp(s, &b, &a, fc, fs).and_then(|r| k::num_of(&r, "magnitude_db"));
    let top = k::lines(&gs, "", "Gain (dB)", &format!("order {n} {bt}, fc = {}", q(fc, "Hz")), gfc.map(|v| (fc, v)),
                       &gfc.map(|v| format!("{} dB at fc", num(v))).unwrap_or_default(), true);
    let pfc = fresp(s, &b, &a, fc, fs).and_then(|r| k::num_of(&r, "phase"));
    let bottom = k::lines(&ps, "Frequency (Hz)", "Phase (rad)", "", None, "", true);
    let _ = pfc;
    let (body, h) = k::stack(&[top, bottom]);
    let list: Vec<String> = designs.iter().map(|d| format!("{} (b = {}, a = {})", d.0, lst(&d.1), lst(&d.2))).collect();
    Some(s.figure("response", "Designed Butterworth filters: gain and phase",
                  &format!("Gain (top, down to -100 dB) and unwrapped phase (bottom) of digital Butterworth filters \
                            designed by the model for fc = {} at fs = {}: {}. Each response is 61 runs of the model's \
                            frequency response from fs/400 to fs/2 (logarithmic axis). Marked: the gain of your design at \
                            fc{}.", q(fc, "Hz"), q(fs, "Hz"), list.join("; "),
                           gfc.map(|v| format!(", {} dB", num(v))).unwrap_or_default()), &body, h))
}

fn design_stems(s: &Solved) -> Option<Figure> {
    let (b, a) = (s.out_list("b")?, s.out_list("a")?);
    let n = 30;
    let h = s.solve_other("impulse_response", &[("b", jl(&b)), ("a", jl(&a)), ("n", Input::Int(n))])
        .and_then(|r| k::list_of(&r, "h"))?;
    let st = s.solve_other("step_response", &[("b", jl(&b)), ("a", jl(&a)), ("n", Input::Int(n))])
        .and_then(|r| k::list_of(&r, "s"))?;
    let items = vec![("impulse response".to_string(), 0), ("step response".to_string(), 2)];
    let last = st[st.len() - 1];
    let (body, ht) = panels(&format!("step response reaches {} after {n} samples", num(last)), &items, &[
        Sp::new("", "Sample n", "Response").stem(idx(&h), 0).line(idx(&st), 2),
    ])?;
    Some(s.figure("stems", "A designed digital filter: impulse and step response",
                  &format!("Impulse response (stems) and step response (line) of the Butterworth filter designed by the \
                            model (b = {}, a = {}), {n} samples each computed by the model's impulse- and step-response \
                            capabilities; the step response ends at {}.", lst(&b), lst(&a), num(last)), &body, ht))
}

fn response(s: &Solved) -> Option<Figure> {
    let (b, f, fs) = (s.vec("b")?, s.inp("f")?, s.inp("fs")?);
    let a = s.vec("a").unwrap_or_else(|| vec![1.0]);
    let (m, ph) = (s.out("magnitude")?, s.out("phase")?);
    let fr = k::span(0.0, fs / 2.0 * 0.995, 101, false);
    let (mut mp, mut pp) = (vec![], vec![]);
    for x in &fr {
        if let Some(r) = fresp(s, &b, &a, *x, fs) {
            if let (Some(mm), Some(p)) = (k::num_of(&r, "magnitude"), k::num_of(&r, "phase")) {
                mp.push((*x, mm));
                pp.push((*x, p));
            }
        }
    }
    if mp.len() < 2 {
        return None;
    }
    unwrap(&mut pp);
    // the unwrapped phase placed so that it passes through the phase returned at f
    if let Some(near) = pp.iter().min_by(|u, v| (u.0 - f).abs().partial_cmp(&(v.0 - f).abs()).unwrap_or(std::cmp::Ordering::Equal)) {
        let shift = ((ph - near.1) / (2.0 * std::f64::consts::PI)).round() * 2.0 * std::f64::consts::PI;
        for p in pp.iter_mut() {
            p.1 += shift;
        }
    }
    let top = k::lines_y(&[("|H|".into(), mp)], "", "Gain |H|", &format!("|H| = {} ({} dB) at f = {}", num(m),
                         num(s.out("magnitude_db").unwrap_or(f64::NAN)), q(f, "Hz")), Some((f, m)),
                         &format!("|H| = {}", num(m)), false, true);
    let bottom = k::lines(&[("phase".into(), pp)], "Frequency (Hz)", "Phase (rad)", "", Some((f, ph)),
                          &format!("{} rad", num(ph)), false);
    let (body, h) = k::stack(&[top, bottom]);
    Some(s.figure("response", "Frequency response: gain and phase",
                  &format!("Gain and phase of the filter b = {}, a = {} at fs = {}, from 0 to fs/2 (101 runs of the \
                            model). Marked: f = {}, |H| = {}, phase {} rad.", lst(&b), lst(&a), q(fs, "Hz"), q(f, "Hz"),
                           num(m), num(ph)), &body, h))
}

fn dc_response(s: &Solved) -> Option<Figure> {
    let b = s.vec("b")?;
    let a = s.vec("a").unwrap_or_else(|| vec![1.0]);
    let (gn, gdb) = (s.out("gain")?, s.out("gain_db")?);
    let mut pts = vec![];
    for f in k::span(0.0, 0.5, 101, false) {
        if let Some(m) = fresp(s, &b, &a, f, 1.0).and_then(|r| k::num_of(&r, "magnitude")) {
            pts.push((f, m));
        }
    }
    if pts.len() < 2 {
        return None;
    }
    let body = k::lines_y(&[("|H|".into(), pts)], "Frequency (cycles per sample, fs = 1)", "Gain |H|",
                          &format!("DC gain {} ({} dB)", num(gn), num(gdb)), Some((0.0, gn.abs())),
                          &format!("DC gain {}", num(gn)), false, true);
    Some(s.chart("response", "DC gain as the zero-frequency end of the response",
                 &format!("The gain of the filter b = {}, a = {} from 0 to half the sampling rate (101 frequency-response \
                           runs of the model, fs = 1): at f = 0 it equals the returned DC gain {} ({} dB), marked.",
                          lst(&b), lst(&a), num(gn), num(gdb)), &body))
}

fn bilinear(s: &Solved) -> Option<Figure> {
    let (fc, fs) = (s.inp("fc")?, s.inp("fs")?);
    let (b0, b1, a1) = (s.out("b0")?, s.out("b1")?, s.out("a1")?);
    let (b, a) = ([b0, b1], [1.0, a1]);
    let fr = k::span(fs / 400.0, fs / 2.0 * 0.999, 81, true);
    let (gp, _) = gain_phase(s, &b, &a, &fr, fs, -80.0);
    if gp.iter().filter(|p| p.1.is_finite()).count() < 2 {
        return None;
    }
    let gfc = fresp(s, &b, &a, fc, fs).and_then(|r| k::num_of(&r, "magnitude_db"));
    let body = k::lines(&[("gain".into(), gp)], "Frequency (Hz)", "Gain (dB)",
                        &format!("b0 = b1 = {}, a1 = {}", num(b0), num(a1)), gfc.map(|v| (fc, v)),
                        &gfc.map(|v| format!("{} dB at fc", num(v))).unwrap_or_default(), true);
    Some(s.chart("response", "Bilinear-transform RC low-pass: gain",
                 &format!("The gain of the first-order digital low-pass obtained from an RC filter by the bilinear \
                           transform for fc = {} at fs = {}: the model returns b0 = {}, b1 = {}, a1 = {}, and the response \
                           of H(z) = (b0 + b1 z⁻¹)/(1 + a1 z⁻¹) is 81 frequency-response runs from fs/400 to fs/2 \
                           (logarithmic axis). Marked: the gain at fc{}.", q(fc, "Hz"), q(fs, "Hz"), num(b0), num(b1),
                          num(a1), gfc.map(|v| format!(", {} dB", num(v))).unwrap_or_default()), &body))
}

fn group_delay(s: &Solved) -> Option<Figure> {
    let (b, f, fs) = (s.vec("b")?, s.inp("f")?, s.inp("fs")?);
    let d = s.out("samples")?;
    let fr = k::span(0.0, fs / 2.0 * 0.98, 60, false);
    let mine = s.curve("f", &fr, "samples", &[]);
    if mine.len() < 2 {
        return None;
    }
    let mut series = vec![("your filter".to_string(), mine)];
    if let Some(r) = s.solve_other("butterworth_design", &[("N", Input::Int(2)), ("fc", Input::Float(fs / 8.0)),
                                                        ("fs", Input::Float(fs)), ("btype", Input::Text("lowpass".into()))]) {
        if let (Some(bb), Some(aa)) = (k::list_of(&r, "b"), k::list_of(&r, "a")) {
            let mut pts = vec![];
            for x in &fr {
                if let Some(v) = s.solve_other("group_delay", &[("b", jl(&bb)), ("a", jl(&aa)), ("f", Input::Float(*x)),
                                                                ("fs", Input::Float(fs))]).and_then(|r| k::num_of(&r, "samples")) {
                    pts.push((*x, v));
                }
            }
            if pts.len() >= 2 {
                series.push((format!("order-2 Butterworth low-pass, fc = {}", q(fs / 8.0, "Hz")), pts));
            }
        }
    }
    let body = k::lines_y(&series, "Frequency (Hz)", "Group delay (samples)",
                          &format!("{} samples = {} at f = {}", num(d), q(s.out("seconds").unwrap_or(f64::NAN), "s"),
                                   q(f, "Hz")), Some((f, d)), &format!("{} samples", num(d)), false, true);
    Some(s.chart("delay", "Group delay against frequency",
                 &format!("Group delay of the filter b = {}{} against frequency at fs = {}, 60 runs of the model; for \
                           comparison the order-2 Butterworth low-pass the model designs for fc = fs/8, its delay computed \
                           by the model too. Marked: f = {}, {} samples.", lst(&b),
                          s.vec("a").map(|a| format!(", a = {}", lst(&a))).unwrap_or_default(), q(fs, "Hz"), q(f, "Hz"),
                          num(d)), &body))
}

const WINDOWS: [&str; 4] = ["rectangular", "hann", "hamming", "blackman"];

fn window_designs(s: &Solved) -> Option<Vec<(String, Vec<f64>)>> {
    let w0 = s.txt("window")?;
    let mut out = vec![];
    for w in WINDOWS {
        let h = if w == w0 { s.out_list("h")? } else {
            match s.with_input(&[("window", Input::Text(w.into()))]).and_then(|r| k::list_of(&r, "h")) {
                Some(h) => h,
                None => continue,
            }
        };
        out.push((w.to_string(), h));
    }
    Some(out)
}

fn window_responses(s: &Solved) -> Option<Figure> {
    let (fc, fs) = (s.inp("fc")?, s.inp("fs")?);
    let n = s.val("N")?.int();
    let w0 = s.txt("window")?;
    let designs = window_designs(s)?;
    let fr = k::span(0.0, fs / 2.0, 41, false);
    let mut series = vec![];
    for (w, h) in &designs {
        let (gp, _) = gain_phase(s, h, &[1.0], &fr, fs, -110.0);
        series.push((if *w == w0 { format!("{w} (yours)") } else { w.clone() }, gp));
    }
    let body = k::lines_y(&series, "Frequency (Hz)", "Gain (dB)", &format!("{n} taps, fc = {}", q(fc, "Hz")), None, "",
                          false, true);
    Some(s.chart("response", "FIR window choice: stopband against transition width",
                 &format!("Gain of the four {n}-tap FIR low-pass designs of fc = {}, fs = {} (taps from the model, each \
                           response 41 frequency-response runs, clipped at -110 dB): the rectangular window gives the \
                           sharpest edge but the highest stopband ripple; Blackman the deepest stopband and widest edge. \
                           Your design ({w0} window): h = {}.", q(fc, "Hz"), q(fs, "Hz"), lst(&s.out_list("h")?)), &body))
}

fn window_taps(s: &Solved) -> Option<Figure> {
    let (fc, fs) = (s.inp("fc")?, s.inp("fs")?);
    let n = s.val("N")?.int();
    let w0 = s.txt("window")?;
    let designs = window_designs(s)?;
    if n > 200 {
        return None;
    }
    let mut sp = Sp::new("", "Tap index", "Tap value");
    let mut items = vec![];
    let mut centres = vec![];
    for (i, (w, h)) in designs.iter().enumerate() {
        sp = sp.stem(idx(h), i);
        items.push((if *w == w0 { format!("{w} (yours)") } else { w.clone() }, i));
        centres.push(format!("{w} centre tap {}", num(h[h.len() / 2])));
    }
    let (body, ht) = panels(&format!("{n} taps, fc = {}, fs = {}", q(fc, "Hz"), q(fs, "Hz")), &items, &[sp])?;
    Some(s.figure("taps", "Windowed-sinc FIR taps for four windows",
                  &format!("Taps of {n}-tap low-pass FIR filters designed by the model for fc = {} at fs = {} with four \
                            windows: the windowed sinc, tapered toward zero at the ends by every window but the \
                            rectangular one; {}. Your window: {w0}, h = {}.", q(fc, "Hz"), q(fs, "Hz"), centres.join("; "),
                           lst(&s.out_list("h")?)),
                  &body, ht))
}

fn windows(s: &Solved) -> Option<Figure> {
    let n = s.val("N")?.int();
    let w0 = s.txt("window")?;
    let form = s.txt("form").unwrap_or_else(|| "symmetric".into());
    if n > 4096 {
        return None;
    }
    let mut sp = Sp::new("", "Index n", "w[n]");
    let mut items = vec![];
    let mut gains = vec![];
    for (i, w) in WINDOWS.iter().enumerate() {
        let (wv, cg) = if *w == w0 { (s.out_list("w")?, s.out("coherent_gain")?) } else {
            match s.with_input(&[("window", Input::Text((*w).into()))]) {
                Some(r) => (k::list_of(&r, "w")?, k::num_of(&r, "coherent_gain")?),
                None => continue,
            }
        };
        sp = if n <= 40 { sp.stem(idx(&wv), i) } else { sp.line(idx(&wv), i) };
        items.push((format!("{w}{} (gain {})", if *w == w0 { ", yours" } else { "" }, num(cg)), i));
        gains.push(format!("{w} {}", num(cg)));
    }
    let (body, ht) = panels(&format!("{n}-point {form} windows"), &items, &[sp])?;
    Some(s.figure("windows", "Four window shapes and their coherent gains",
                  &format!("The four window shapes of length N = {n} ({form} form) returned by the model, each a run of \
                            it; the legend gives each window's coherent gain ({}). Your window: {w0}.", gains.join(", ")),
                  &body, ht))
}

// ------------------------------------------------------------------------------------------------ planes
fn pz_lists(a: &Answer) -> Option<(Vec<(f64, f64)>, Vec<(f64, f64)>)> {
    Some((cxs(a, "poles")?, cxs(a, "zeros")?))
}

fn reach(v: &[(f64, f64)]) -> f64 {
    v.iter().fold(0.0f64, |m, z| m.max(z.0.abs()).max(z.1.abs()))
}

fn pz_plane(s: &Solved, sets: &[(String, Vec<(f64, f64)>, Vec<(f64, f64)>, usize)], head: &str,
            extra: &[(String, &'static str, Option<usize>)]) -> (String, f64) {
    let r = sets.iter().map(|x| reach(&x.1).max(reach(&x.2))).fold(1.0f64, f64::max) * 1.18;
    let r = r.min(1e6);
    let mut p = g::zplane(r);
    for (_, poles, zeros, c) in sets {
        p.rings(zeros, *c);
        p.crosses(poles, *c);
    }
    p.labels("", "Imaginary part");
    let mut body = String::new();
    g::header(&mut body, head, 340.0);
    body.push_str(&p.done());
    let mut notes: Vec<(String, &str, Option<usize>)> = vec![("× poles, ○ zeros".into(), "axp-bt", None)];
    notes.extend(extra.iter().cloned());
    g::side_notes(&mut body, &notes, 62.0, 340.0);
    let _ = s;
    (body, 340.0)
}

fn poles_zeros(s: &Solved) -> Option<Figure> {
    let (b, a) = (s.vec("b")?, s.vec("a")?);
    let (poles, zeros) = pz_lists(s.a)?;
    let rad = s.out("max_pole_radius")?;
    let st = s.out_text("stability")?;
    let (body, h) = pz_plane(s, &[("yours".into(), poles.clone(), zeros.clone(), 0)],
                             &format!("largest pole radius {} ({st})", num(rad)), &[
        (clip(&format!("poles: {}", zs(&poles)), 22), "axp-vt", Some(0)),
        (clip(&format!("zeros: {}", zs(&zeros)), 22), "axp-vt", None),
        (format!("radius {}", num(rad)), "axp-vt", None),
        (st.clone(), "axp-bt", None),
    ]);
    Some(s.figure("poles", "Poles and zeros in the z-plane",
                  &format!("Poles and zeros returned by the model for b = {}, a = {}: poles {}, zeros {}; largest pole \
                            radius {} ({st}; stable when every pole is inside the unit circle).", lst(&b), lst(&a),
                           zs(&poles), zs(&zeros), num(rad)), &body, h))
}

fn poles_zeros_two(s: &Solved) -> Option<Figure> {
    let (b, a) = (s.vec("b")?, s.vec("a")?);
    let (poles, zeros) = pz_lists(s.a)?;
    let (rb, ra) = ([1.0, 0.0, -1.0], [1.0, -1.131, 0.64]);
    let r = s.with_input(&[("b", jl(&rb)), ("a", jl(&ra))])?;
    let (p2, z2) = pz_lists(&r)?;
    let rad2 = k::num_of(&r, "max_pole_radius")?;
    let (body, h) = pz_plane(s, &[("yours".into(), poles.clone(), zeros.clone(), 0),
                                  ("resonator".into(), p2.clone(), z2.clone(), 2)],
                             "your filter beside a two-pole resonator", &[
        ("your filter".into(), "axp-bt", Some(0)),
        (clip(&format!("poles {}", zs(&poles)), 22), "axp-vt", None),
        (format!("radius {}", num(s.out("max_pole_radius")?)), "axp-vt", None),
        ("resonator".into(), "axp-bt", Some(2)),
        (clip(&format!("poles {}", zs(&p2)), 22), "axp-vt", None),
        (format!("radius {}", num(rad2)), "axp-vt", None),
    ]);
    Some(s.figure("poles-2", "Poles and zeros: your filter and a resonator",
                  &format!("Poles and zeros returned by the model for your filter b = {}, a = {} (poles {}, zeros {}) \
                            and for a two-pole resonator chosen for the drawing, b = [1, 0, -1], a = [1, -1.131, 0.64] \
                            (poles {}, radius {}, zeros {}).", lst(&b), lst(&a), zs(&poles), zs(&zeros), zs(&p2),
                           num(rad2), zs(&z2)), &body, h))
}

fn notch_poles(s: &Solved) -> Option<Figure> {
    let (f0, qq, fs) = (s.inp("f0")?, s.inp("Q")?, s.inp("fs")?);
    let (b, a) = (s.out_list("b")?, s.out_list("a")?);
    let r = s.solve_other("poles_zeros", &[("b", jl(&b)), ("a", jl(&a))])?;
    let (poles, zeros) = pz_lists(&r)?;
    let rad = k::num_of(&r, "max_pole_radius")?;
    let st = k::num_of(&r, "stability").map(num).or_else(|| r.values.iter().find(|x| x.0 == "stability")
        .and_then(|x| x.1.as_str().map(|t| t.to_string()))).unwrap_or_default();
    let (body, h) = pz_plane(s, &[("notch".into(), poles.clone(), zeros.clone(), 0)],
                             &format!("notch at f0 = {}, Q = {}", q(f0, "Hz"), num(qq)), &[
        (clip(&format!("poles {}", zs(&poles)), 22), "axp-vt", Some(0)),
        (clip(&format!("zeros {}", zs(&zeros)), 22), "axp-vt", None),
        (format!("pole radius {}", num(rad)), "axp-vt", None),
        (st.clone(), "axp-bt", None),
    ]);
    Some(s.figure("poles", "Notch filter poles and zeros",
                  &format!("Poles and zeros of the notch filter designed by the model for f0 = {}, Q = {}, fs = {} (from \
                            the model's pole-zero capability): zeros {} on the unit circle at the notch angle, poles {} \
                            just inside them, largest radius {} ({st}). The filter: b = {}, a = {}.", q(f0, "Hz"), num(qq),
                           q(fs, "Hz"), zs(&zeros), zs(&poles), num(rad), lst(&b), lst(&a)), &body, h))
}

fn notch_response(s: &Solved) -> Option<Figure> {
    let (f0, qq, fs) = (s.inp("f0")?, s.inp("Q")?, s.inp("fs")?);
    let (b, a) = (s.out_list("b")?, s.out_list("a")?);
    // dense near f0: f0 ± (f0/2) t³
    let mut fr: Vec<f64> = (0..60).map(|i| {
        let t = 1.0 - i as f64 / 60.0;
        f0 - f0 / 2.0 * t * t * t
    }).collect();
    fr.push(f0);
    fr.extend((1..=60).map(|i| {
        let t = i as f64 / 60.0;
        f0 + f0 / 2.0 * t * t * t
    }));
    fr.retain(|f| *f > 0.0 && *f < fs / 2.0);
    let mut pts = vec![];
    for f in &fr {
        if let Some(r) = fresp(s, &b, &a, *f, fs) {
            let db = k::num_of(&r, "magnitude_db").unwrap_or(-60.0);
            pts.push((*f, db.max(-60.0)));
        }
    }
    if pts.len() < 2 {
        return None;
    }
    let at = fresp(s, &b, &a, f0, fs).map(|r| k::num_of(&r, "magnitude_db").unwrap_or(-60.0).max(-60.0))?;
    let body = k::lines_y(&[("gain".into(), pts)], "Frequency (Hz)", "Gain (dB, clipped at -60)",
                          &format!("f0 = {}, Q = {}", q(f0, "Hz"), num(qq)), Some((f0, at)),
                          &format!("f0 = {}", q(f0, "Hz")), false, true);
    Some(s.chart("response", "Notch filter: a narrow rejection at f0",
                 &format!("The gain of the second-order notch filter designed by the model for f0 = {}, Q = {}, fs = {} \
                           (b = {}, a = {}): {} frequency-response runs of the model from f0/2 to 1.5 f0, dense near f0; \
                           the plot is clipped at -60 dB.", q(f0, "Hz"), num(qq), q(fs, "Hz"), lst(&b), lst(&a),
                          fr.len()), &body))
}

fn second_order(s: &Solved) -> Option<Figure> {
    let (a1, a2) = (s.inp("a1")?, s.inp("a2")?);
    let vals: Vec<f64> = if a2 > 0.0 { vec![a2 * 0.5, a2, a2 * 1.62, a2 * 2.42] } else {
        vec![a2 - 0.5, a2, a2 + 0.3, a2 + 0.6]
    };
    let mut sets = vec![];
    let mut notes: Vec<(String, &'static str, Option<usize>)> = vec![];
    let mut cap = vec![];
    for (i, v) in vals.iter().enumerate() {
        let r = if *v == a2 { None } else {
            match s.with(&[("a2", *v)]) {
                Some(r) => Some(r),
                None => continue,
            }
        };
        let ans = r.as_ref().unwrap_or(s.a);
        let p1 = cx(&ans.values.iter().find(|x| x.0 == "p1")?.1)?;
        let p2 = cx(&ans.values.iter().find(|x| x.0 == "p2")?.1)?;
        let rad = k::num_of(ans, "radius")?;
        let st = ans.values.iter().find(|x| x.0 == "stability").and_then(|x| x.1.as_str()).unwrap_or("").to_string();
        let c = i % 4;
        sets.push((format!("a2 = {}", num(*v)), vec![p1, p2], vec![], c));
        notes.push((format!("a2 = {}{}", num(*v), if *v == a2 { " (yours)" } else { "" }), "axp-bt", Some(c)));
        notes.push((format!("r = {}, {st}", num(rad)), "axp-vt", None));
        cap.push(format!("a2 = {}: {}, radius {}, {st}", num(*v), zs(&[p1, p2]), num(rad)));
    }
    let (body, h) = pz_plane(s, &sets, &format!("a1 = {}: poles as a2 grows", num(a1)), &notes);
    Some(s.figure("poles", "Second-order poles move outward as a2 grows",
                  &format!("Poles of 1/(1 + a1 z⁻¹ + a2 z⁻²) returned by the model for a1 = {} and four values of a2 \
                            (yours is a2 = {}): {}.", num(a1), num(a2), cap.join("; ")), &body, h))
}

fn gamma_plane(s: &Solved) -> Option<Figure> {
    let (rl, z0) = (s.inp("RL")?, s.inp("Z0")?);
    let xl = s.inp("XL").unwrap_or(0.0);
    let (gr, gi, gm) = (s.out("gamma_re")?, s.out("gamma_im")?, s.out("gamma_mag")?);
    let mut dots = vec![];
    for r in k::span(z0 / 10.0, z0 * 10.0, 15, true) {
        if let Some(a) = s.with(&[("RL", r), ("XL", 0.0)]) {
            if let (Some(x), Some(y)) = (k::num_of(&a, "gamma_re"), k::num_of(&a, "gamma_im")) {
                dots.push((x, y));
            }
        }
    }
    let mut p = g::zplane(1.15);
    p.circle(0.0, 0.0, gm);
    p.dots(&dots, 3, 3.0);
    p.phasor(gr, gi, 0);
    p.mark(gr, gi, "Γ");
    p.labels("", "Imaginary part of Γ");
    let mut body = String::new();
    g::header(&mut body, &format!("Γ = {} {} {}j", num(gr), if gi < 0.0 { "-" } else { "+" }, num(gi.abs())), 340.0);
    body.push_str(&p.done());
    let notes: Vec<(String, &str, Option<usize>)> = vec![
        (format!("ZL = {} {} j{} Ω", num(rl), if xl < 0.0 { "-" } else { "+" }, num(xl.abs())), "axp-bt", Some(0)),
        (format!("Z0 = {} Ω", num(z0)), "axp-lt", None),
        (format!("|Γ| = {}", num(gm)), "axp-vt", None),
        (format!("angle {}°", num(s.out("gamma_angle_deg")?)), "axp-vt", None),
        (format!("VSWR {}", num(s.out("vswr")?)), "axp-vt", None),
        (format!("return loss {} dB", num(s.out("return_loss_db")?)), "axp-vt", None),
        ("resistive loads".into(), "axp-lt", Some(3)),
    ];
    g::side_notes(&mut body, &notes, 62.0, 340.0);
    Some(s.figure("gamma", "Reflection coefficient in the Γ-plane",
                  &format!("The reflection coefficient Γ of the load ZL = {} {} j{} Ω on a Z0 = {} Ω line in the unit \
                            circle of the Γ-plane: the model returns Γ = {} {} j{} (|Γ| = {}), VSWR {}, return loss {} dB. \
                            Dashed: the circle of loads with the same |Γ|. Grey dots: purely resistive loads from Z0/10 to \
                            10 Z0, each a run of the model.", num(rl), if xl < 0.0 { "-" } else { "+" }, num(xl.abs()),
                           num(z0), num(gr), if gi < 0.0 { "-" } else { "+" }, num(gi.abs()), num(gm),
                           num(s.out("vswr")?), num(s.out("return_loss_db")?)), &body, 340.0))
}
